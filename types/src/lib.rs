use std::{num::NonZeroU16, ptr};

use common::hashmap::RTVariableMap;
use stabby::str::Str as RStr;
use stabby::dynptr;
use stabby::{Any, boxed::Box as RBox};

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
  pub itself: Option<NonZeroU16>,
  pub r1: Option<NonZeroU16>,
  pub r2: Option<NonZeroU16>,
  pub r3: Option<NonZeroU16>,
  pub r4: Option<NonZeroU16>,
  pub r5: Option<NonZeroU16>,
  pub r6: Option<NonZeroU16>,
  pub r7: Option<NonZeroU16>,
  pub r8: Option<NonZeroU16>,
}

impl FnStack {
  #[unsafe(no_mangle)]
  pub extern "C" fn create() -> Self {
    Self {
      ret: MaybeNone,
      itself: None,
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

  pub fn clear(&mut self) {
    self.itself = None;
    self.r1 = None;
    self.r2 = None;
    self.r3 = None;
    self.r4 = None;
    self.r5 = None;
    self.r6 = None;
    self.r7 = None;
    self.r8 = None;
  }

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

impl CVariable {
  pub unsafe fn get_u32(&self) -> u32 {
    let data: &RustVariable<Wrapper<u32>> = unsafe { RustVariable::from_ptr(self.data) };

    data.inner()
  }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Wrapper<T: Copy + Clone>(T);

impl<T: Copy + Clone> Wrapper<T> {
  pub fn inner(&self) -> T {
    self.0
  }
}

generate! {
  CVariable;
  generate: u8,u16,u32,u64,usize,i8,i16,i32,i64,f32,f64,isize,bool
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

#[repr(C)]
pub struct MemoryMap {
  variables: RTVariableMap,
}

impl MemoryMap {
  #[cfg(feature = "memory")]
  pub fn create_map() -> Self {
    Self {
      variables: HashMap::new(),
    }
  }
}

#[repr(C)]
pub enum VariableData {
  Constant(RStr<'static>),
  Raw(CVariable),
  Abi(dynptr!(RBox<dyn Any + 'static>))
}