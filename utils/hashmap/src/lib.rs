use std::{collections::HashMap, ffi::c_void};

use lrtexec_lib::{
  common::{others::FFISafeString, FFIableObject}, Maybe, Ref
};

struct AbiUnsafeNumMap(HashMap<usize, FFIableObject>);
struct AbiUnsafeStrMap(HashMap<String, FFIableObject>);

fn to_str(data: FFISafeString) -> String {
  data.to_string()
}

extern "C" fn dealloc<T>(ptr: *mut c_void) {
  unsafe { drop(Box::from_raw(ptr as *mut T)) }
}

#[unsafe(no_mangle)]
pub extern "C" fn create(use_str_map: bool) -> Ref {
  let (ptr, drop): (*mut c_void, extern "C" fn(ptr: *mut c_void)) = if use_str_map {
    (
      Box::into_raw(Box::new(AbiUnsafeStrMap(HashMap::new()))) as *mut _,
      dealloc::<AbiUnsafeStrMap>,
    )
  } else {
    (
      Box::into_raw(Box::new(AbiUnsafeNumMap(HashMap::new()))) as *mut _,
      dealloc::<AbiUnsafeNumMap>,
    )
  };

  Ref { ptr, drop }
}

macro_rules! get {
  ($x:ident as $y:ty) => {
    unsafe { Box::from_raw((&*$x).ptr as *mut $y) }
  };
}

pub unsafe extern "C" fn insert_int(map: *const Ref, k: usize, v: FFIableObject) -> Maybe<FFIableObject> {
  let mut hmap = get!(map as AbiUnsafeNumMap);

  let data = hmap.as_mut().0.insert(k, v).into();

  _ = Box::into_raw(hmap);

  data
}

pub unsafe extern "C" fn get_int(map: *const Ref, k: *const usize) -> Maybe<*const FFIableObject> {
  let mut hmap = get!(map as AbiUnsafeNumMap);

  let data = hmap.as_mut().0.get(unsafe { &*k }).into();

  _ = Box::into_raw(hmap);

  data
}

pub unsafe extern "C" fn remove_int(map: *const Ref, k: *const usize) -> Maybe<FFIableObject> {
  let mut hmap = get!(map as AbiUnsafeNumMap);

  let data = hmap.as_mut().0.remove(unsafe { &*k }).into();

  _ = Box::into_raw(hmap);

  data
}

pub unsafe extern "C" fn insert_str(map: *const Ref, k: FFISafeString, v: FFIableObject) -> Maybe<FFIableObject> {
  let mut hmap = get!(map as AbiUnsafeStrMap);

  let data = hmap.as_mut().0.insert(to_str(k), v).into();

  _ = Box::into_raw(hmap);

  data
}

pub unsafe extern "C" fn get_str(
  map: *const Ref,
  k: *const FFISafeString,
) -> Maybe<*const FFIableObject> {
  let mut hmap = get!(map as AbiUnsafeStrMap);

  let data = hmap
    .as_mut()
    .0
    .get(unsafe { &*k }.as_str().expect("Impossible"))
    .into();

  _ = Box::into_raw(hmap);

  data
}

pub unsafe extern "C" fn remove_str(
  map: *const Ref,
  k: *const FFISafeString,
) -> Maybe<FFIableObject> {
  let mut hmap = get!(map as AbiUnsafeStrMap);

  let data = hmap
    .as_mut()
    .0
    .remove(unsafe { &*k }.as_str().expect("Impossible"))
    .into();

  _ = Box::into_raw(hmap);

  data
}
