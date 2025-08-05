use std::{env, mem::transmute, os::raw::c_void, path::PathBuf, str::FromStr, sync::LazyLock};

use libloading::{library_filename, Library, Symbol};

use crate::common::FFIableObject;

pub static WAKER: LazyLock<Waker> = LazyLock::new(|| {
  Waker::new()
});

/// This function consumes the pointer, should be called only once
pub extern "C" fn call_waker_consume_ptr(waker: *mut c_void) {
  // Load from the lazy library
  (*WAKER.call_waker_consume_ptr)(waker)
}

pub type CreateWaker = extern "C" fn(waker: FFIableObject, call: extern "C" fn(waker: FFIableObject) -> ()) -> *mut c_void;
pub type CallWakerConsume = extern "C" fn(waker: *mut c_void);

pub struct Waker {
  _lib: Library,
  pub(crate) create_waker: Symbol<'static, CreateWaker>,
  pub call_waker_consume_ptr: Symbol<'static, CallWakerConsume>
}

impl Waker {
  pub fn new() -> Self {
    let lrt = env::var("LRT_HOME").expect("LRT Home not present");

    let file = library_filename("async_waker");

    let mut path = PathBuf::from_str(&lrt).unwrap();

    path.push("libs");
    path.push("waker");
    path.push(file);

    let lib = unsafe { Library::new(path).expect("Unable to load async_waker") }; 

    let create: Symbol<'_, CreateWaker> = unsafe { lib.get(b"").unwrap() };
    let create = unsafe{ transmute(create) };

    let call_consume_ptr: Symbol<'_, extern "C" fn(*mut std::ffi::c_void)> = unsafe { lib.get(b"").unwrap() };
    let call_consume_ptr: Symbol<'static, extern "C" fn(*mut std::ffi::c_void)> = unsafe{ transmute(call_consume_ptr) };

    Self {
      _lib: lib,
      create_waker: create,
      call_waker_consume_ptr: call_consume_ptr
    }
  }
}