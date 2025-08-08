use crate::{
  Maybe, Ref,
  common::{FFIableObject, others::FFISafeString},
  create,
};
use libloading::{Library, Symbol, library_filename};
use std::{env, mem::transmute, path::PathBuf, str::FromStr, sync::LazyLock};

create! {
  defines FfiHashMap
  loads "hashmap"

  extern C {
    doc = "Creates a new HashMap"
    fn create -> Ref { use_str_map: bool }
    
    doc = "Inserts a key-value pair using an integer key"
    fn insert_int -> Maybe<FFIableObject> { map: *const Ref, k: usize, v: FFIableObject }
    
    doc = "Gets a reference to the value associated with an integer key"
    fn get_int -> Maybe<*const FFIableObject> { map: *const Ref, k: *const usize }
    
    doc = "Removes and returns the value associated with an integer key"
    fn remove_int -> Maybe<FFIableObject> { map: *const Ref, k: *const usize }
    
    doc = "Inserts a key-value pair using a string key"
    fn insert_str -> Maybe<FFIableObject> { map: *const Ref, k: FFISafeString, v: FFIableObject }
    
    doc = "Gets a reference to the value associated with a string key"
    fn get_str -> Maybe<*const FFIableObject> { map: *const Ref, k: *const FFISafeString }
    
    doc = "Removes and returns the value associated with a string key"
    fn remove_str -> Maybe<FFIableObject> { map: *const Ref, k: *const FFISafeString }
  }
}
