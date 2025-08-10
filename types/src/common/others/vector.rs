use crate::{
  common::{others::boxes::Boxed, FFIableObject}, 
  Maybe
};
use libc::malloc;
use std::{ffi::c_void, ops::{Index, Range}, ptr, slice};

#[repr(C)]
pub struct FFIVector {
  // Array of pointers
  ptr: *mut *mut c_void,
  len: usize,
  cap: usize
}

// Implementations support these
unsafe impl Send for FFIVector {}
unsafe impl Sync for FFIVector {}

impl FFIVector {
  pub fn null() -> Self {
    Self {
      ptr: ptr::null_mut(),
      cap: 0,
      len: 0
    }
  }

  pub fn new(slice: Vec<FFIableObject>) -> Maybe<Self> {
    let len = slice.len();

    if len == 0 {
      return Maybe::Some(Self::null());
    }

    // Let the capacity be calculated like this
    // size * 2
    // No need for null ptr since its our own format
    let Some(cap) = len.checked_mul(2) else {
      return Maybe::None;
    };

    // Allocate capacity
    let ptr = unsafe { malloc(cap * size_of::<*mut c_void>()) as *mut *mut c_void };

    if ptr.is_null() {
      return Maybe::None;
    }

    unsafe {
      for (index, item) in slice.into_iter().enumerate() {
        let data = Boxed::into_raw(Boxed::new(item));

        *ptr.add(index) = data;
      }
    }
    
    Maybe::Some(Self {
      ptr,
      len,
      cap
    })
  }

  pub fn push(&mut self, item: FFIableObject) -> Maybe<()> {
    let Some(new_len) = self.len.checked_add(1) else {
      return Maybe::None;
    };

    let mut cap = self.cap;

    if new_len > self.cap {
      // Reallocate
      unsafe {
        let Some(c) = new_len.checked_mul(2) else {
          return Maybe::None;
        };
        cap = c;

        let new_ptr = libc::realloc(self.ptr as _, cap * size_of::<*mut c_void>()) as *mut *mut c_void;

        if new_ptr.is_null() {
          return Maybe::None;
        }

        self.ptr = new_ptr as *mut *mut c_void;
      }
    }

    // Push from the index self.len upto self.new_len
    unsafe {
      *self.ptr.add(self.len) = Boxed::into_raw(Boxed::<FFIableObject>::new(item));
    }

    self.len = new_len;
    self.cap = cap;

    Maybe::Some(())
  }

  pub fn length(&self) -> usize {
    self.len
  }

  pub fn capacity(&self) -> usize {
    self.cap
  }

  /// This does not realloc and returns Maybe::None, if the total size is less than cap
  /// 
  /// # SAFETY
  /// - **CRITICAL** you must ensure that the bounds check is correct
  pub unsafe fn get_at<'a>(&'a self, index: Range<usize>) -> &'a [*mut c_void] {
    let dst = self.ptr;

    unsafe { slice::from_raw_parts(dst.add(index.start), index.len()) }
  }

  pub fn edit(&mut self, index: usize, data: FFIableObject) -> Maybe<()> {
    if index >= self.len {
      return Maybe::None;
    }

    unsafe {
      let data = Boxed::into_raw(Boxed::new(data));

      let old_ptr = *self.ptr.add(index);

      // Allocate the new data
      *self.ptr.add(index) = data;

      // Deallocate the older data
      drop(Boxed::<FFIableObject>::from_raw(old_ptr));

      Maybe::Some(())
    }
  }

  // // This does not realloc and returns Maybe::None, if the total size is less than cap
  // pub fn edit(&mut self, start: usize, src: Vec<FFIableObject>) -> Maybe<()> {
  //   todo!();

  //   unsafe {
  //     let src = src.as_ref().as_bytes();

  //     // Start is an index, so start + src.len() is also an index. So add `+1`
  //     let to_be_set_as_len = (start + src.len()).max(self.len);

  //     // Allow append via edit
  //     // Reallocate if necessary
  //     if self.cap < to_be_set_as_len {
  //       let Some(new_cap) = to_be_set_as_len.checked_mul(2) else {
  //         return Maybe::None;
  //       };

  //       // Reallocate
  //       {
  //         let ptr = realloc(self.ptr as _, new_cap);

  //         if ptr.is_null() {
  //           return Maybe::None;
  //         }

  //         self.ptr = ptr as _;
  //         self.cap = new_cap;
  //       }
  //     }

  //     self.len = to_be_set_as_len;

  //     ptr::copy_nonoverlapping(src.as_ptr(), self.ptr.add(start) as _, src.len());
  //   }

  //   Maybe::Some(())
  // }
}

impl Index<Range<usize>> for FFIVector {
  type Output = [*mut c_void];

  fn index(&self, index: Range<usize>) -> &Self::Output {
    unsafe { 
      self.get_at(
        index
      )
    }
  }
}

impl Drop for FFIVector {
  fn drop(&mut self) {
    if !self.ptr.is_null() {
      // Deallocate all the Boxed types
      unsafe { self.get_at(0..self.len) }
        .iter()
        .for_each(|x| {
          unsafe { drop(Boxed::<FFIableObject>::from_raw(*x)) };
        });

      // Free the pointer
      unsafe { libc::free(self.ptr as *mut c_void) }
    }
  }
}