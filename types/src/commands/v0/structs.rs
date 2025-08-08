//! # v0
//! This crate defines the `v0` of the Assembly Syntax of LRTEXEC Bytecode (also mentioned as assembly in many places)
//! 
//! Structs related to v0 Assembly Syntax

use std::fmt::Debug;

use crate::common::FFIableObject;

#[repr(C)]
#[derive(Default)]
pub struct FnStackV0 {
  /// Return value (identifier in MemoryMap)
  pub ret: Option<FFIableObject>,
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
  _inner: RegValueV0
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