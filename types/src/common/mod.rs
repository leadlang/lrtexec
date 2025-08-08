use std::{
  any::TypeId,
  ffi::c_void,
  fmt::{Debug, Display},
  marker::PhantomData, ptr::null_mut,
};

use crate::{
  commands::FFISafeContainer,
  common::others::{FFISafeString, boxes::Boxed},
};

pub mod r#async;
pub mod others;

#[repr(C)]
pub struct FFIableObject {
  data: *mut c_void,
  drop: extern "C" fn(*mut c_void),
  fmt: extern "C" fn(*mut c_void) -> FFISafeString,
  display: extern "C" fn(*mut c_void) -> FFISafeString,
  poisoned: bool,
  tag: u8,
}

impl FFISafeContainer for FFIableObject {}

#[repr(C)]
pub struct WrappedFFIableObject<'a, T> {
  object: *mut FFIableObject,
  r#type: PhantomData<&'a T>,
}

impl<'a, T> WrappedFFIableObject<'a, T> {
  pub fn create_using_box<E: Debug + Display + 'static>(data: E) -> (Self, FFIableObject) {
    let mut object = FFIableObject::create_using_box(data);

    let data = Self::create_from_object(&mut object);

    (data, object)
  }

  pub fn create_using_box_no_display<E: Debug + 'static>(data: E) -> (Self, FFIableObject) {
    let mut object = FFIableObject::create_using_box_no_display(data);

    let data = Self::create_from_object(&mut object);

    (data, object)
  }

  pub fn create_using_box_non_static<E: Debug + Display>(data: E) -> (Self, FFIableObject) {
    let mut object = FFIableObject::create_using_box_non_static(data);

    let data = Self::create_from_object(&mut object);

    (data, object)
  }

  pub fn create_using_box_no_display_non_static<E: Debug + 'static>(
    data: E,
  ) -> (Self, FFIableObject) {
    let mut object = FFIableObject::create_using_box_no_display_non_static(data);

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
    drop(Boxed::from_raw(ptr as *mut T));
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

macro_rules! implement {
  (
    $($num:expr => $t:ty),*
  ) => {
    pastey::paste! {
      $(
          impl Into<FFIableObject> for $t {
            fn into(self) -> FFIableObject {
              FFIableObject::create_using_box(self)
            }
          }

          impl FFIableObject {
            /// Returns `None` is types do not match
            pub fn [<as_ $t:lower>](&self) -> Option<&$t> {
              if self.tag != $num {
                return None;
              }

              unsafe {
                Some(self.[<as_ $t:lower _unchecked>]())
              }
            }

            /// Returns `None` is types do not match
            pub fn [<as_ $t:lower _mut>]<'a>(&'a mut self) -> Option<&'a mut $t> {
              if self.tag != $num {
                return None;
              }

              unsafe {
                Some(self.[<as_ $t:lower _mut_unchecked>]())
              }
            }

            /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
            pub unsafe fn [<as_ $t:lower _unchecked>]<'a>(&'a self) -> &'a $t {
              unsafe {
                self.get::<$t>()
              }
            }

            /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
            pub unsafe fn [<as_ $t:lower _mut_unchecked>]<'a>(&'a mut self) -> &'a mut $t {
              unsafe {
                self.get_mut::<$t>()
              }
            }
          }
      )*
    }

    fn get_tag<T: 'static>() -> u8 {
      let ty = TypeId::of::<T>();

      $(
        if ty == TypeId::of::<$t>() {
          return $num;
        }
      )*

      0
    }
  };
}

extern "C" fn drop_noop(_: *mut c_void) {

}

extern "C" fn fmt_noop(_: *mut c_void) -> FFISafeString {
  FFISafeString::from("")
}

impl FFIableObject {
  /// This might be a good way to create a dummy, invalid struct
  /// We still dont recommend it
  pub fn null() -> Self {
    Self {
      data: null_mut(),
      display: no_display,
      drop: drop_noop,
      fmt: fmt_noop,
      poisoned: true,
      tag: 0
    }
  }

  pub const fn is_null(&self) -> bool {
    self.data.is_null()
  }

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

    (unsafe { Boxed::from_raw(self.data as *mut T) }).unbox()
  }

  /// Transfers the ownership to the new data and sets the `poisoned` field to `true` of this structure
  pub unsafe fn transfer_ownership(&mut self) -> FFIableObject {
    if self.poisoned {
      panic!("The object is already poisoned.");
    }

    let data = self.data;
    self.poisoned = true;

    FFIableObject {
      data,
      drop: self.drop,
      fmt: self.fmt,
      display: self.display,
      poisoned: false,
      tag: self.tag,
    }
  }

  /// Returns whether this FFIableObject is poisoned or not. This is usually used to check whether
  /// `reconstruct` or `transfer_ownership` has been called on this instance before calling any other methods.
  pub const fn is_poisoned(&self) -> bool {
    self.poisoned
  }

  /// Get a mutable reference to the inner `FFIableObject`
  ///
  /// # Safety
  /// We assume that you're absolutely sure that the strucure is the `T` that you've provided
  pub unsafe fn get_mut<'a, T>(&'a mut self) -> &'a mut T {
    if self.poisoned {
      panic!("The object is poisoned.");
    }

    unsafe { self.get_mut_unchecked() }
  }

  /// Get a mutable reference to the inner `FFIableObject` like `get_mut`
  ///
  /// # 🚨 Safety 
  /// ## **CRITICAL CAUTION REQUIRED**
  /// - We assume that you're absolutely sure that the strucure is the `T` that you've provided
  /// - **CRITICAL** This function does not check if the data is poisoned!
  pub unsafe fn get_mut_unchecked<'a, T>(&'a mut self) -> &'a mut T {
    unsafe { &mut *(self.data as *mut T) }
  }

  /// Get a mutable reference to the inner `FFIableObject`
  ///
  /// # Safety
  /// We assume that you're absolutely sure that the strucure is the `T` that you've provided
  pub unsafe fn get<'a, T>(&'a self) -> &'a T {
    if self.poisoned {
      panic!("The object is poisoned.");
    }

    unsafe { self.get_unchecked() }
  }

  /// Get a mutable reference to the inner `FFIableObject` like `get`
  ///
  /// # 🚨 Safety 
  /// ## **CRITICAL CAUTION REQUIRED**
  /// - We assume that you're absolutely sure that the strucure is the `T` that you've provided
  /// - **CRITICAL** This function does not check if the data is poisoned!
  pub unsafe fn get_unchecked<'a, T>(&'a self) -> &'a T {
    unsafe { &*(self.data as *mut T) }
  }

  pub fn create_using_box<T: Debug + Display + 'static>(data: T) -> Self {
    let data = Boxed::new(data);
    let data = Boxed::into_raw(data);

    Self {
      data: data as *mut c_void,
      display: general_display::<T>,
      drop: general_drop::<T>,
      fmt: general_debug::<T>,
      poisoned: false,
      tag: get_tag::<T>(),
    }
  }

  pub fn create_using_box_no_display<T: Debug + 'static>(data: T) -> Self {
    let data = Boxed::new(data);
    let data = Boxed::into_raw(data);

    Self {
      data: data as *mut c_void,
      display: no_display,
      drop: general_drop::<T>,
      fmt: general_debug::<T>,
      poisoned: false,
      tag: get_tag::<T>(),
    }
  }

  pub fn create_using_box_non_static<T: Debug + Display>(data: T) -> Self {
    let data = Boxed::new(data);
    let data = Boxed::into_raw(data);

    Self {
      data: data as *mut c_void,
      display: general_display::<T>,
      drop: general_drop::<T>,
      fmt: general_debug::<T>,
      poisoned: false,
      tag: 0,
    }
  }

  pub fn create_using_box_no_display_non_static<T: Debug>(data: T) -> Self {
    let data = Boxed::new(data);
    let data = Boxed::into_raw(data);

    Self {
      data: data as *mut c_void,
      display: no_display,
      drop: general_drop::<T>,
      fmt: general_debug::<T>,
      poisoned: false,
      tag: 0,
    }
  }
}

implement! {
  1 => u8,
  2 => u16,
  3 => u32,
  4 => u64,
  5 => u128,
  6 => i8,
  7 => i16,
  8 => i32,
  9 => i64,
  10 => i128,
  11 => f32,
  12 => f64,
  13 => bool,
  14 => FFISafeString
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
