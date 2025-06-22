use std::{
  ffi::{CStr, CString, c_char, c_void},
  fmt::{Debug, Display},
  marker::PhantomData,
};

pub mod variables;

#[repr(C)]
pub struct CommonString {
  data: *mut c_char,
  drop: extern "C" fn(*mut c_char),
}

extern "C" fn common_string_drop(ptr: *mut c_char) {
  unsafe {
    drop(CString::from_raw(ptr));
  }
}

impl Into<CommonString> for String {
  fn into(self) -> CommonString {
    let cstring = CString::new(self).unwrap();

    let data = cstring.into_raw();

    CommonString {
      data,
      drop: common_string_drop,
    }
  }
}

impl AsRef<CStr> for CommonString {
  fn as_ref(&self) -> &CStr {
    unsafe { CStr::from_ptr(self.data) }
  }
}

impl Drop for CommonString {
  fn drop(&mut self) {
    (self.drop)(self.data)
  }
}

#[repr(C)]
pub struct FFIableObject {
  data: *mut c_void,
  drop: extern "C" fn(*mut c_void),
  fmt: extern "C" fn(*mut c_void) -> CommonString,
  display: extern "C" fn(*mut c_void) -> CommonString,
}

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

  pub unsafe fn get_mut(&'a mut self) -> &'a T {
    unsafe { self.get_ptr().get_mut() }
  }
}

extern "C" fn general_drop<T>(ptr: *mut c_void) {
  unsafe {
    drop(Box::from_raw(ptr as *mut T));
  }
}

extern "C" fn general_display<T: Display>(ptr: *mut c_void) -> CommonString {
  unsafe {
    let data = &*(ptr as *mut T);

    let fmt = format!("{}", data);

    fmt.into()
  }
}

extern "C" fn general_debug<T: Debug>(ptr: *mut c_void) -> CommonString {
  unsafe {
    let data = &*(ptr as *mut T);

    let fmt = format!("{:?}", data);

    fmt.into()
  }
}

extern "C" fn no_display(_: *mut c_void) -> CommonString {
  format!("<cannot display type>").into()
}

impl FFIableObject {
  pub unsafe fn get_mut<'a, T>(&'a mut self) -> &'a mut T {
    unsafe { &mut *(self.data as *mut T) }
  }

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
    }
  }
}

impl Display for FFIableObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let data = (self.display)(self.data);

    let data = data.as_ref();
    let data = data.to_str();

    let Ok(data) = data else {
      return Err(std::fmt::Error::default());
    };

    f.write_str(data)
  }
}

impl Debug for FFIableObject {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let data = (self.fmt)(self.data);

    let data = data.as_ref();
    let data = data.to_str();

    let Ok(data) = data else {
      return Err(std::fmt::Error::default());
    };

    f.write_str(data)
  }
}

impl Drop for FFIableObject {
  fn drop(&mut self) {
    (self.drop)(self.data)
  }
}
