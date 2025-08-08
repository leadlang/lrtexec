use crate::{
  Maybe, Ref,
  common::FFIableObject,
  create,
};
use libloading::{Library, Symbol, library_filename};
use std::{env, mem::transmute, path::PathBuf, str::FromStr, sync::LazyLock};

create! {
  defines FfiVector
  loads "vector"

  extern C {
    doc = "Creates a new empty vector"
    fn create -> Ref {}

    doc = "Gets a reference to the element at the specified index"
    fn get_at -> Maybe<*const FFIableObject> { this: *mut Ref, index: usize }

    doc = "Gets a mutable reference to the element at the specified index"
    fn get_at_mut -> Maybe<*mut FFIableObject> { this: *mut Ref, index: usize }
    
    doc = "Replaces the element at the specified index"
    fn replace -> Maybe<FFIableObject> { this: *mut Ref, index: usize, with: FFIableObject }
    
    doc = "Removes the last element from the vector and returns it"
    fn pop -> Maybe<FFIableObject> { this: *mut Ref }
    
    doc = "Appends an element to the back of the vector"
    fn push -> () { this: *mut Ref, item: FFIableObject }
    
    doc = "Returns the number of elements in the vector"
    fn length -> usize { this: *mut Ref }
    
    doc = "Returns the total number of elements the vector can hold without reallocating"
    fn capacity -> usize { this: *mut Ref }
  }
}

pub struct Vect {

}