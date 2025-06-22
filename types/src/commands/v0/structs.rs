//! Structs related to v0 commands

use std::os::raw::c_void;

use crate::common::FFIableObject;

#[repr(C)]
pub enum VariableDataV0 {
  Inbuilt(Container),
  Object(FFIableObject)
}

#[repr(C)]
pub struct Container {
  pub data: *mut c_void,
  pub drop: extern "C" fn(*mut c_void),
  pub id: u8
}

impl Drop for Container {
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
          impl Into<Container> for $t {
            fn into(self) -> Container {
              let data = Box::new(self);
              let d = Box::into_raw(data);

              Container {
                data: d as *mut c_void,
                drop: general_drop::<$t>,
                id: $num
              }
            }
          }

          impl Container {
            /// Returns `None` is types do not match
            pub fn [<as_ $t>](&self) -> Option<&$t> {
              if self.id != $num {
                return None;
              }

              unsafe {
                Some(&*(self.data as *mut $t))
              }
            }

            /// Returns `None` is types do not match
            pub fn [<as_ $t _mut>](&self) -> Option<&mut $t> {
              if self.id != $num {
                return None;
              }

              unsafe {
                Some(&mut *(self.data as *mut $t))
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
}

// use std::{fmt::Debug, num::NonZeroU16};

// pub struct FnStackV0 {
//   /// Return value (identifier in MemoryMap)
//   pub ret: Option<VariableDataV0>,
//   // General Purpose (identifiers in MemoryMap)
//   pub itself: Option<NonZeroU16>,
//   pub r1: Option<NonZeroU16>,
//   pub r2: Option<NonZeroU16>,
//   pub r3: Option<NonZeroU16>,
//   pub r4: Option<NonZeroU16>,
//   pub r5: Option<NonZeroU16>,
//   pub r6: Option<NonZeroU16>,
//   pub r7: Option<NonZeroU16>,
//   pub r8: Option<NonZeroU16>,
// }

// pub enum FmtOutput<'a> {
//   String(BoxedStr),
//   Str(Str<'a>)
// }


// pub trait ObjectV0 {
//   extern "C" fn debug_fmt<'a>(&'a self) -> Option<FmtOutput<'a>>;
//   extern "C" fn display_fmt<'a>(&'a self) -> Option<FmtOutput<'a>>;
// }

// pub enum VariableDataV0 {
//   Arc(Arc<dynptr!(RBox<dyn ObjectV0 + 'static>)>),
//   Object(dynptr!(RBox<dyn ObjectV0 + 'static>))
// }

// pub fn create_v0<T: ObjectV0>(data: T) -> VariableDataV0 {
//   VariableDataV0::Object(RBox::new(data))
// }

// impl<T: Debug> ObjectV0 for T {
//   extern "C" fn debug_fmt<'a>(&'a self) -> Option<FmtOutput<'a> > {
//     Some(FmtOutput::String(format!("{:?}", self).into_boxed_str()))
//   }

//   extern "C" fn display_fmt<'a>(&'a self) -> Option<FmtOutput<'a> > {
//     None
//   }
// }