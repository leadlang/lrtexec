use std::{ffi::c_void, ptr::write};

use libc::{free, malloc};

use crate::{common::{others::boxes::Boxed, FFIableObject}, Maybe};

/// Runtime Struct used by the Language to define structures
/// These are ungrowable Heap Allocated Objects
/// 
/// Structures are defined in lrt using the following spec:
/// - Each structure is an array of pointers
/// - Each pointer points to a data managed by the struct
/// - The Struct can only be accessed by the Interpreter
/// - LRT Packages (via FFI) cannot access this structure
/// - **(Worked On)** LRT Packages (written in LRT) might be allowed to use this structure, but no bandwidth yet
/// 
/// TODO:
/// - Allow LRT Packages in LRT to use Structs
#[repr(C)]
pub struct LRTStruct {
  // Pointers to FFI Boxes
  ptr: *mut *mut c_void,
  len: usize
}

impl LRTStruct {
  pub fn new_struct<const N: usize>(
    values: [FFIableObject; N],
  ) -> Maybe<Self> {
    let len = values.len();

    let ptr = unsafe {
      malloc(len * size_of::<*mut c_void>()) as *mut *mut c_void
    };

    if ptr.is_null() {
      return Maybe::None;
    }

    values.into_iter()
      .enumerate()
      .for_each(|(index, x)| {
        let src = Boxed::into_raw(Boxed::new(x));

        unsafe { write(ptr.add(index), src) };
      });

    Maybe::Some(Self {
      ptr,
      len
    })
  }

  pub fn new_from_vector(
    values: Vec<FFIableObject>,
  ) -> Maybe<Self> {
    let len = values.len();

    let ptr = unsafe {
      malloc(len * size_of::<*mut c_void>()) as *mut *mut c_void
    };

    if ptr.is_null() {
      return Maybe::None;
    }

    values.into_iter()
      .enumerate()
      .for_each(|(index, x)| {
        let src = Boxed::into_raw(Boxed::new(x));

        unsafe { write(ptr.add(index), src) };
      });

    Maybe::Some(Self {
      ptr,
      len
    })
  }

  #[unsafe(no_mangle)]
  // Let rust do its thing
  pub extern "C" fn free(self) {}
}

impl Drop for LRTStruct {
  fn drop(&mut self) {
    unsafe {
      let ptr = self.ptr;

      for index in 0..self.len {
        drop(Boxed::<FFIableObject>::from_raw(*self.ptr.add(index)));
      }
      
      // Release the memory back
      free(ptr as *mut c_void);
    }
  }
}