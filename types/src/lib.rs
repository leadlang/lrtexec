#![feature(trivial_bounds)]

use std::os::raw::c_void;

pub mod commands;
pub mod common;

#[cfg(feature = "sudo")]
#[repr(C)]
pub struct Ref {
  pub ptr: *mut c_void,
  pub drop: extern "C" fn(ptr: *mut c_void),
}

#[cfg(not(feature = "sudo"))]
#[repr(C)]
pub struct Ref {
  ptr: *mut c_void,
  drop: extern "C" fn(ptr: *mut c_void),
}

impl Drop for Ref {
  fn drop(&mut self) {
    (self.drop)(self.ptr)
  }
}

#[repr(C)]
pub enum Maybe<T> {
  Some(T),
  None
}

impl<T> From<Option<&T>> for Maybe<*const T> {
  fn from(value: Option<&T>) -> Self {
    match value {
      Some(x) => Maybe::Some(x),
      None => Maybe::None
    }
  }
}

impl<T> From<Option<&mut T>> for Maybe<*mut T> {
  fn from(value: Option<&mut T>) -> Self {
    match value {
      Some(x) => Maybe::Some(x),
      None => Maybe::None
    }
  }
}

impl<T> From<Option<T>> for Maybe<T> {
  fn from(value: Option<T>) -> Self {
    match value {
      Some(x) => Maybe::Some(x),
      None => Maybe::None
    }
  }
}

#[cfg(test)]
mod tests;