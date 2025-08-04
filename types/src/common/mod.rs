use std::{
  ffi::c_void,
  fmt::{Debug, Display},
  marker::PhantomData,
};

use crate::{commands::FFISafeContainer, common::others::FFISafeString};

pub mod r#async;
pub mod others;

#[repr(C)]
pub struct FFIableObject {
  data: *mut c_void,
  drop: extern "C" fn(*mut c_void),
  fmt: extern "C" fn(*mut c_void) -> FFISafeString,
  display: extern "C" fn(*mut c_void) -> FFISafeString,
  poisoned: bool,
}

impl FFISafeContainer for FFIableObject {}

#[repr(C)]
pub struct WrappedFFIableObject<'a, T> {
  object: *mut FFIableObject,
  r#type: PhantomData<&'a T>,
}

impl<'a, T> WrappedFFIableObject<'a, T> {
  pub fn create_using_box<E: Debug + Display>(data: E) -> (Self, FFIableObject) {
    let mut object = FFIableObject::create_using_box(data);

    let data = Self::create_from_object(&mut object);

    (data, object)
  }

  pub fn create_using_box_no_display<E: Debug>(data: E) -> (Self, FFIableObject) {
    let mut object = FFIableObject::create_using_box_no_display(data);

    let data = Self::create_from_object(&mut object);

    (data, object)
  }

  pub fn create_from_object<'e>(object: &'e mut FFIableObject) -> Self {
    Self {
      object,
      r#type: PhantomData,
    }
  }

  fn get_ptr(&self) -> &mut FFIableObject {
    unsafe { &mut *self.object }
  }

  pub unsafe fn get(&'a self) -> &'a T {
    unsafe { self.get_ptr().get() }
  }

  pub unsafe fn get_mut(&'a mut self) -> &'a mut T {
    unsafe { self.get_ptr().get_mut() }
  }
}

extern "C" fn general_drop<T>(ptr: *mut c_void) {
  unsafe {
    drop(Box::from_raw(ptr as *mut T));
  }
}

extern "C" fn general_display<T: Display>(ptr: *mut c_void) -> FFISafeString {
  unsafe {
    let data = &*(ptr as *mut T);

    let fmt = format!("{}", data);

    FFISafeString::from(fmt)
  }
}

extern "C" fn general_debug<T: Debug>(ptr: *mut c_void) -> FFISafeString {
  unsafe {
    let data = &*(ptr as *mut T);

    let fmt = format!("{:?}", data);

    FFISafeString::from(fmt)
  }
}

extern "C" fn no_display(_: *mut c_void) -> FFISafeString {
  FFISafeString::from(format!("<cannot display type>"))
}

impl FFIableObject {
  /// (Un)safely consumes the FFIableObject and returns the original owned `T`.
  ///
  /// This method transfers ownership of the raw data pointer from this FFIableObject
  /// to the returned `T`. It sets this FFIableObject's `poisoned` flag to `true`
  /// to prevent its `drop` implementation from freeing the memory it no longer owns.
  ///
  /// # Panics
  /// Panics if this FFIableObject is poisoned.
  ///
  /// # Safety
  ///
  /// This function is unsafe because the caller must ensure that:
  /// 1. This `FFIableObject` instance currently owns the data (i.e., `self.is_poisoned()` is `false`).
  ///    Calling this on a poisoned object will lead to a panic.
  /// 2. The `FFIableObject` actually contains a value of type `T`. Mis-casting `T` will lead to Undefined Behavior.
  /// 3. This `FFIableObject` is not used further after this call, as its internal pointer
  ///    will effectively be consumed.
  pub unsafe fn reconstruct<T: Debug>(mut self) -> T {
    if self.poisoned {
      panic!("FFIableObject is poisoned");
    }

    self.poisoned = true;

    *(unsafe { Box::from_raw(self.data as *mut T) })
  }

  /// Transfers the ownership to the new data and sets the `poisoned` field to `true` of this structure
  pub unsafe fn transfer_ownership(&mut self) -> FFIableObject {
    let data = self.data;
    self.poisoned = true;

    FFIableObject {
      data,
      drop: self.drop,
      fmt: self.fmt,
      display: self.display,
      poisoned: false,
    }
  }

  /// Returns whether this FFIableObject is poisoned or not. This is usually used to check whether
  /// `reconstruct` or `transfer_ownership` has been called on this instance before calling any other methods.
  pub fn is_poisoned(&self) -> bool {
    self.poisoned
  }

  /// Get a mutable reference to the inner `FFIableObject`
  ///
  /// # Safety
  /// Do no use this is the struct is poisoned
  pub unsafe fn get_mut<'a, T>(&'a mut self) -> &'a mut T {
    unsafe { &mut *(self.data as *mut T) }
  }

  /// Get a mutable reference to the inner `FFIableObject`
  ///
  /// # Safety
  /// Do no use this is the struct is poisoned
  pub unsafe fn get<'a, T>(&'a self) -> &'a T {
    unsafe { &*(self.data as *mut T) }
  }

  pub fn create_using_box<T: Debug + Display>(data: T) -> Self {
    let data = Box::new(data);
    let data = Box::into_raw(data);

    Self {
      data: data as *mut c_void,
      display: general_display::<T>,
      drop: general_drop::<T>,
      fmt: general_debug::<T>,
      poisoned: false,
    }
  }

  pub fn create_using_box_no_display<T: Debug>(data: T) -> Self {
    let data = Box::new(data);
    let data = Box::into_raw(data);

    Self {
      data: data as *mut c_void,
      display: no_display,
      drop: general_drop::<T>,
      fmt: general_debug::<T>,
      poisoned: false,
    }
  }
}

impl Display for FFIableObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let data = (self.display)(self.data);

    let data = data.as_str();

    let Some(data) = data else {
      return Err(std::fmt::Error::default());
    };

    f.write_str(data)
  }
}

impl Debug for FFIableObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let data = (self.fmt)(self.data);

    let data = data.as_str();

    let Some(data) = data else {
      return Err(std::fmt::Error::default());
    };

    f.write_str(data)
  }
}

impl Drop for FFIableObject {
  fn drop(&mut self) {
    if !self.poisoned {
      (self.drop)(self.data)
    }
  }
}
