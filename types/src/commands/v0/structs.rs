//! # v0
//! This crate defines the `v0` of the Assembly Syntax of LRTEXEC Bytecode (also mentioned as assembly in many places)
//! 
//! Structs related to v0 Assembly Syntax

use std::{fmt::Debug, os::raw::c_void};

use crate::common::{others::FFISafeString, FFIableObject};

#[repr(C)]
pub enum VariableDataV0 {
  Inbuilt(ContainerV0),
  Object(FFIableObject)
}

#[repr(C)]
pub struct ContainerV0 {
  pub data: *mut c_void,
  pub drop: extern "C" fn(*mut c_void),
  pub id: u8
}

impl Drop for ContainerV0 {
  fn drop(&mut self) {
    (self.drop)(self.data)
  }
}

extern "C" fn general_drop<T>(ptrr: *mut c_void) {
  unsafe {
    _ = Box::from_raw(ptrr as *mut T);
  }
}

macro_rules! implement {
  (
    $($num:expr => $t:ty),*
  ) => {
    pastey::paste! {
        $(
          impl Into<ContainerV0> for $t {
            fn into(self) -> ContainerV0 {
              let data = Box::new(self);
              let d = Box::into_raw(data);

              ContainerV0 {
                data: d as *mut c_void,
                drop: general_drop::<$t>,
                id: $num
              }
            }
          }

          impl ContainerV0 {
            /// Returns `None` is types do not match
            pub fn [<as_ $t>](&self) -> Option<&$t> {
              if self.id != $num {
                return None;
              }

              unsafe {
                Some(self.[<as_ $t _unchecked>]())
              }
            }

            /// Returns `None` is types do not match
            pub fn [<as_ $t _mut>](&self) -> Option<&mut $t> {
              if self.id != $num {
                return None;
              }

              unsafe {
                Some(self.[<as_ $t _mut_unchecked>]())
              }
            }

            /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
            pub unsafe fn [<as_ $t _unchecked>](&self) -> &$t {
              unsafe {
                &*(self.data as *mut $t)
              }
            }

            /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
            pub unsafe fn [<as_ $t _mut_unchecked>](&self) -> &mut $t {
              unsafe {
                &mut *(self.data as *mut $t)
              }
            }
          }
      )*
    }
  };
}

implement! {
  0 => u8,
  1 => u16,
  2 => u32,
  3 => u64,
  4 => u128,
  5 => i8,
  6 => i16,
  7 => i32,
  8 => i64,
  9 => i128,
  10 => f32,
  11 => f64,
  12 => bool
  // 13 => FFISafeString
}

impl Into<ContainerV0> for String {
  fn into(self) -> ContainerV0 {
    FFISafeString::from_str(&self).into()
  }
}

impl Into<ContainerV0> for &str {
  fn into(self) -> ContainerV0 {
    FFISafeString::from_str(self).into()
  }
}

impl Into<ContainerV0> for FFISafeString {
  fn into(self) -> ContainerV0 {
    let data = Box::new(self);
    let d = Box::into_raw(data);

    ContainerV0 {
      data: d as *mut c_void,
      drop: general_drop::<FFISafeString>,
      id: 13
    }
  }
}

impl ContainerV0 {
  /// Returns `None` is types do not match
  pub fn as_string(&self) -> Option<&FFISafeString> {
    if self.id != 13 {
      return None;
    }

    unsafe {
      Some(self.as_string_unchecked())
    }
  }

  /// Returns `None` is types do not match
  pub fn as_string_mut(&self) -> Option<&mut FFISafeString> {
    if self.id != 13 {
      return None;
    }

    unsafe {
      Some(self.as_string_mut_unchecked())
    }
  }

  /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
  pub unsafe fn as_string_unchecked(&self) -> &FFISafeString {
    unsafe {
      &*(self.data as *mut FFISafeString)
    }
  }

  /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
  pub unsafe fn as_string_mut_unchecked(&self) -> &mut FFISafeString {
    unsafe {
      &mut *(self.data as *mut FFISafeString)
    }
  }
}


#[repr(C)]
#[derive(Default)]
pub struct FnStackV0 {
  /// Return value (identifier in MemoryMap)
  pub ret: Option<VariableDataV0>,
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
  pub unsafe fn get_ptr_mut<T>(&self) -> Option<&mut T> {
    unsafe {
      Some(match self._inner {
        RegValueV0::Moved(ptr) => &mut *ptr,
        RegValueV0::Mut(ptr) => &mut *ptr,
        _ => return None,
      }.get_mut())
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