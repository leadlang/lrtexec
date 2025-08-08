//! # v0
//! This crate defines the `v0` of the Assembly Syntax of LRTEXEC Bytecode (also mentioned as assembly in many places)
//! 
//! Structs related to v0 Assembly Syntax

use std::{cell::UnsafeCell, fmt::Debug, mem::replace};

use crate::common::{r#async::{AsyncInterface, ReturnReg}, FFIableObject};

#[repr(C)]
pub struct FnStackV0 {
  /// Return value (identifier in MemoryMap)
  // Do not set a public since it can be misused
  // Still it may be misused by redeclaraing in the crate, but still, safety measure

  // TODO: Shift to something else that has even better mem safety
  // What about using FFIableObject itself, but thats not the best way
  pub ret: UnsafeCell<ReturnReg<FFIableObject>>,
  /// Registers
  pub r1: WrapperRegValueV0,
  pub r2: WrapperRegValueV0,
  pub r3: WrapperRegValueV0,
  pub r4: WrapperRegValueV0,
  pub r5: WrapperRegValueV0,
  pub r6: WrapperRegValueV0,
  pub r7: WrapperRegValueV0,
  pub r8: WrapperRegValueV0,
}

#[cfg(not(feature = "fnstack_no_cleanup"))]
macro_rules! cleanup {
  (
    $(
      $x:expr
    ),*
  ) => {
    $(
      $x._inner = RegValueV0::Null;
    )*
  };
}

impl FnStackV0 {
  /// Set either the output or the async task, doing both will override one of the results
  pub fn set_output(&self, out: FFIableObject) {
    * unsafe { self.ret.as_mut_unchecked() } = ReturnReg::Output(out);
  }

  /// Set either the output or the async task, doing both will override one of the results
  pub fn set_async_task(&self, out: AsyncInterface<FFIableObject>) {
    * unsafe { self.ret.as_mut_unchecked() } = ReturnReg::Async(out);
  }

  // The library side can't run this since this is provided to the library as a `*const FnStackV0`
  pub fn postrun(&mut self) -> Option<ReturnReg<FFIableObject>> {
    let ret_to_replace = ReturnReg::<FFIableObject>::Null;

    let ret = replace(self.ret.get_mut(), ret_to_replace);

    // Cleanup
    #[cfg(not(feature = "fnstack_no_cleanup"))]
    {
      cleanup!(
        self.r1,
        self.r2,
        self.r3,
        self.r4,
        self.r5,
        self.r6,
        self.r7,
        self.r8
      );
    }

    if matches!(ret, ReturnReg::Null) {
      return None;
    }

    Some(ret)
  }
}

macro_rules! fnstack {
    (
      $($t:ident => $x:expr),*
    ) => {
      $(
        pub static $t: u16 = $x;
      )*
    };
}

fnstack! {
  REGISTER_R1 => 0,
  REGISTER_R2 => 1,
  REGISTER_R3 => 2,
  REGISTER_R4 => 3,
  REGISTER_R5 => 4,
  REGISTER_R6 => 5,
  REGISTER_R7 => 6,
  REGISTER_R8 => 7,
  REGISTER_RET => 8
}

#[repr(C)]
#[derive(Default)]
pub enum RegValueV0 {
  Moved(*mut FFIableObject),
  Mut(*mut FFIableObject),
  Ref(*const FFIableObject),
  #[default]
  Null
}

#[repr(C)]
#[derive(Default)]
pub struct WrapperRegValueV0 {
  pub(crate) _inner: RegValueV0
}

impl WrapperRegValueV0 {
  /// Get a shared reference to the inner `FFIableObject` if it isn't null.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it relies on the correctness of the `RegValueV0` enum
  /// instance and the provided type `T` by the caller. If the enum instance is `Null`, this function will return `None`. If it is
  /// not `Null`, this function will return a shared reference to the `FFIableObject`
  /// stored in the enum instance.
  pub unsafe fn get_ptr<T>(&self) -> Option<&T> {
    unsafe {
      Some(match self._inner {
        RegValueV0::Moved(ptr) => (&*ptr).get(),
        RegValueV0::Mut(ptr) => (&*ptr).get(),
        RegValueV0::Ref(ptr) => (&*ptr).get(),
        RegValueV0::Null => return None,
      })
    }
  }

  /// Get a mutable reference to the inner `FFIableObject` if it isn't null.
  ///
  /// # Safety
  ///
  /// This function is unsafe because it relies on the correctness of the `RegValueV0` enum
  /// instance and the provided type `T` by the caller. If the enum instance is `Null`, this function will return `None`. If it is
  /// not `Null`, this function will return a mutable reference to the `FFIableObject`
  /// stored in the enum instance.
  pub unsafe fn get_ptr_mut<'a, T>(&'a self) -> Option<&'a mut T> {
    unsafe {
      Some(match self._inner {
        RegValueV0::Moved(ptr) => &mut *ptr,
        RegValueV0::Mut(ptr) => &mut *ptr,
        _ => return None,
      }.get_mut::<T>())
    }
  }

  /// Get a mutable reference to the inner `FFIableObject` if it isn't null.
  ///
  /// # Note
  /// You are given a `&mut T` instance of the `FFIableObject` stored in the enum instance.
  /// 
  /// # Safety
  ///
  /// This function is unsafe because it relies on the correctness of the `RegValueV0` enum
  /// instance and the provided type `T` by the caller. If the enum instance is `Null`, this function will return `None`. If it is
  /// not `Null`, this function will return a mutable reference to the `FFIableObject`
  /// stored in the enum instance.
  pub unsafe fn remove_ptr(&self) -> Option<FFIableObject> {
    unsafe {
      Some(match self._inner {
        RegValueV0::Moved(ptr) => &mut *ptr,
        _ => return None
      }.transfer_ownership())
    }
  }

  /// Get a mutable reference to the inner `FFIableObject` if it isn't null.
  ///
  /// # Note
  /// You are given a `&mut T` instance of the `FFIableObject` stored in the enum instance.
  /// 
  /// # Safety
  ///
  /// This function is unsafe because it relies on the correctness of the `RegValueV0` enum
  /// instance and the provided type `T` by the caller. If the enum instance is `Null`, this function will return `None`. If it is
  /// not `Null`, this function will return a mutable reference to the `FFIableObject`
  /// stored in the enum instance.
  pub unsafe fn remove_ptr_reconstruct<T: Debug>(&self) -> Option<T> {
    unsafe {
      Some(self.remove_ptr()?.reconstruct())
    }
  }
}