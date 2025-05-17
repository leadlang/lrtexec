use std::{
  mem::ManuallyDrop,
  ops::{Deref, DerefMut},
  ptr,
};

use crate::CVariable;

#[derive(Clone)]
pub struct RustVariable<T: Clone>(T);

impl<T: Clone> Deref for RustVariable<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T: Clone> DerefMut for RustVariable<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T: Clone> RustVariable<T> {
  pub fn new(t: T) -> Self {
    Self(t)
  }

  pub unsafe fn from_ptr_mut<'a>(ptr: *mut ()) -> &'a mut Self {
    unsafe { &mut *(ptr as *mut RustVariable<T>) }
  }

  pub unsafe fn from_ptr<'a>(ptr: *mut ()) -> &'a Self {
    unsafe { &*(ptr as *mut RustVariable<T>) }
  }

  pub fn to_c(self) -> CVariable {
    let mut data = ManuallyDrop::new(self);

    let ptr: *mut RustVariable<T> = &mut *data;

    CVariable {
      data: ptr as *mut (),
      _drop: _drop::<RustVariable<T>>,
      _clone: _clone::<RustVariable<T>>,
    }
  }
}

extern "C" fn _drop<T>(data: *mut ()) {
  unsafe {
    let data = &mut *(data as *mut T);
    ptr::drop_in_place(data);
  }
}

extern "C" fn _clone<T: Clone>(data: *mut ()) -> CVariable {
  unsafe {
    let data = &mut *(data as *mut T);

    let cloned = data.clone();

    RustVariable::new(cloned).to_c()
  }
}
