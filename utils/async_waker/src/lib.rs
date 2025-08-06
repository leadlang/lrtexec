use std::os::raw::c_void;

use lrtexec_lib::common::FFIableObject;

#[repr(C)]
pub struct Waker {
  _inner_waker: Option<FFIableObject>,
  call: extern "C" fn(waker: FFIableObject) -> ()
}

#[unsafe(no_mangle)]
pub extern "C" fn create_waker(waker: FFIableObject, call: extern "C" fn(waker: FFIableObject) -> ()) -> *mut c_void {
    Box::into_raw(Box::new(Waker {
        _inner_waker: Some(waker),
        call
    })) as *mut _
}

/// This function consumes the pointer, should be called only once
#[unsafe(no_mangle)]
pub extern "C" fn call_waker_consume_ptr(waker: *mut c_void) {
    let mut waker: Box<Waker> = unsafe {
      Box::from_raw(waker as *mut Waker)
    };

    let ffi_waker = waker._inner_waker.take().unwrap();

    (waker.call)(ffi_waker);

    drop(waker)
}