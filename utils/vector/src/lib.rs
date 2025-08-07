use std::{ffi::c_void, mem::replace as mem_replace};

use lrtexec_lib::{common::FFIableObject, Maybe, Ref};

type AbiUnsafeVector = Vec<FFIableObject>;

extern "C" fn dealloc<T>(ptr: *mut c_void) {
  unsafe { drop(Box::from_raw(ptr as *mut T)) }
}

#[unsafe(no_mangle)]
pub extern "C" fn create() -> Ref {
  let vect: *mut c_void = Box::into_raw(Box::new(Vec::new() as AbiUnsafeVector)) as *mut _;

  Ref {
    ptr: vect,
    drop: dealloc::<AbiUnsafeVector>
  }
}

macro_rules! get {
  ($x:ident) => {
    unsafe { Box::from_raw((&*$x).ptr as *mut AbiUnsafeVector) }
  };
}

macro_rules! forget {
  ($x:ident) => {
    _ = Box::into_raw($x);
  };
}

#[unsafe(no_mangle)]
pub extern "C" fn get_at(this: *mut Ref, index: usize) -> Maybe<*const FFIableObject> {
  let vect = get!(this);

  let resp: Maybe<*const FFIableObject> = vect.get(index).into();

  forget!(vect);

  resp
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_at_mut(this: *mut Ref, index: usize) -> Maybe<*mut FFIableObject> {
  let mut vect = get!(this);

  let resp: Maybe<*mut FFIableObject> = vect.get_mut(index).into();

  forget!(vect);

  resp
}

#[unsafe(no_mangle)]
pub extern "C" fn replace(this: *mut Ref, index: usize, with: FFIableObject) -> Maybe<FFIableObject> {
  let mut vect = get!(this);

  let resp = vect.get_mut(index);

  let mut out = Maybe::None;

  if let Some(x) = resp {
    out = Maybe::Some(mem_replace(x, with));
  }

  forget!(vect);

  out
}

#[unsafe(no_mangle)]
pub extern "C" fn pop(this: *mut Ref) -> Maybe<FFIableObject> {
  let mut vect = get!(this);

  let resp = vect.pop().into();

  forget!(vect);

  resp
}

#[unsafe(no_mangle)]
pub extern "C" fn push(this: *mut Ref, item: FFIableObject) {
  let mut vect = get!(this);

  vect.push(item);

  forget!(vect);
}

#[unsafe(no_mangle)]
pub extern "C" fn length(this: *mut Ref) -> usize {
  let vect = get!(this);

  let resp = vect.len();

  forget!(vect);

  resp
}

#[unsafe(no_mangle)]
pub extern "C" fn capacity(this: *mut Ref) -> usize {
  let vect = get!(this);

  let resp = vect.capacity();

  forget!(vect);

  resp
}