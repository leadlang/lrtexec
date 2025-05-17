use std::
  ffi::{CStr, CString, c_char};

use crate::{CVariable, critical::RustVariable};

#[repr(C)]
#[derive(Clone, Debug)]
pub struct CommonString {
  ptr: *mut c_char,
}

impl CommonString {
  #[unsafe(no_mangle)]
  pub extern "C" fn create(
    data: *mut c_char,
    _clone: extern "C" fn(data: *mut ()) -> CVariable,
    _drop: extern "C" fn(data: *mut ()),
  ) -> Self {
    let data = unsafe { CStr::from_ptr(data) };
    let data = CString::from(data);
    let ptr = data.into_raw(); // This leaks ownership to C

    Self { ptr }
  }

  pub unsafe fn to_cstring_ref(&self) -> &CStr {
    unsafe { CStr::from_ptr(self.ptr) }
  }

  pub unsafe fn to_cstring_owned(self) -> CString {
    // Take back ownership
    unsafe { CString::from_raw(self.ptr) }
  }

  pub unsafe fn unsafe_from_mut<'a>(c: *mut ()) -> &'a mut CommonString {
    unsafe { RustVariable::from_ptr_mut(c) }
  }

  pub unsafe fn unsafe_from<'a>(c: *mut ()) -> &'a CommonString {
    unsafe { RustVariable::from_ptr(c) }
  }
}

impl Into<CVariable> for CommonString {
  fn into(self) -> CVariable {
    RustVariable::new(self).to_c()
  }
}
