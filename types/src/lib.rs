use std::{collections::HashMap, num::NonZeroU16, ptr};
use pastey::paste;
use critical::RustVariable;

pub mod common;
/// If we mess up any of these structs, we're dead
pub mod critical;

#[macro_use]
mod macros;

pub const VERSION: u16 = 0;

#[repr(C)]
pub enum Maybe<T> {
  Some(T),
  None
}

use Maybe::None as MaybeNone;
use Maybe::Some as MaybeSome;

#[repr(C)]
pub struct FnStack {
  /// Return value (identifier in MemoryMap)
  pub ret: Maybe<CVariable>,
  // General Purpose (identifiers in MemoryMap)
  pub(crate) r1: Option<NonZeroU16>,
  pub(crate) r2: Option<NonZeroU16>,
  pub(crate) r3: Option<NonZeroU16>,
  pub(crate) r4: Option<NonZeroU16>,
  pub(crate) r5: Option<NonZeroU16>,
  pub(crate) r6: Option<NonZeroU16>,
  pub(crate) r7: Option<NonZeroU16>,
  pub(crate) r8: Option<NonZeroU16>,
}

impl FnStack {
  #[unsafe(no_mangle)]
  pub extern "C" fn create() -> Self {
    Self {
      ret: MaybeNone,
      r1: None,
      r2: None,
      r3: None,
      r4: None,
      r5: None,
      r6: None,
      r7: None,
      r8: None,
    }
  }

  pub fn clear(&mut self) {}

  #[unsafe(no_mangle)]
  pub extern "C" fn setOutput(&mut self, data: CVariable) {
    self.ret = MaybeSome(data);
  }
}

#[repr(C)]
pub struct CVariable {
  pub data: *mut (),
  pub _drop: extern "C" fn(*mut ()),
  pub _clone: extern "C" fn(*mut ()) -> CVariable,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Wrapper<T: Copy + Clone>(T);

generate! {
  CVariable;
  generate: u8,u16,u32,u64,i8,i16,i32,i64,f32,f64
}

#[repr(C)]
pub struct RustClosure {
  _closure: extern "C" fn(*mut ()),
}

extern "C" fn drop_noop(_: *mut ()) {}
extern "C" fn clone_noop(_: *mut ()) -> CVariable {
  unsafe { CVariable::null() }
}

impl Drop for CVariable {
  fn drop(&mut self) {
    (self._drop)(self.data)
  }
}

impl Clone for CVariable {
  fn clone(&self) -> Self {
    (self._clone)(self.data)
  }
}

impl CVariable {
  pub unsafe fn null() -> Self {
    Self {
      data: ptr::null_mut(),
      _drop: drop_noop,
      _clone: clone_noop,
    }
  }

  pub unsafe fn into_rust(self) -> Option<VariableData> {
    Some(VariableData::Raw(self))
  }
}

pub struct MemoryMap {
  pub variables: HashMap<u16, VariableData>,
}

pub enum VariableData {
  Constant(&'static str),
  Raw(CVariable),
}
