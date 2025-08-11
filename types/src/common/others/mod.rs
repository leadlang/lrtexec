use core::str;
use std::fmt::{self, Display};
use std::ops::{Deref, Index, Range};
use std::os::raw::c_void;
use std::ptr::{copy_nonoverlapping, null_mut};

use std::slice;

use libc::{malloc, realloc};

use crate::Maybe;

pub mod boxes;
pub mod hashmap;
pub mod vector;
pub mod rt_struct;

#[macro_use]
pub(crate) mod macros;

/// A string using our custom spec for FFI.
///
/// This struct manages a custom made String
/// It can store UTF-8 compatible Strings and is quite stable in design
/// 
/// Intended for Rust-Rust FFI
#[repr(C)]
#[derive(Debug)]
/// TODO: Rename to FFISafeString
pub struct FFISafeString {
  // Actual pointer to the string
  // 
  // Design Choice:
  // The pointer points to a data like this
  // [data, reserve]
  //
  // We're using our custom pointer type. 
  ptr: *mut u8,
  // Length without null terminator
  len: usize,

  // Reserve Size
  cap: usize
}

impl FFISafeString {
  pub fn null() -> Self {
    Self {
      ptr: null_mut(),
      cap: 0,
      len: 0
    }
  }

  pub fn new(slice: impl AsRef<str>) -> Maybe<Self> {
    let data: &str = slice.as_ref();

    let len = data.len();

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
    let ptr = unsafe { malloc(cap) as *mut u8 };

    if ptr.is_null() {
      return Maybe::None;
    }

    unsafe {
      let bytes = data.as_bytes();
      let src = bytes.as_ptr();

      copy_nonoverlapping(src, ptr, bytes.len());
    }
    
    Maybe::Some(Self {
      ptr,
      len,
      cap
    })
  }

  pub fn push_str(&mut self, slice: impl AsRef<str>) -> Maybe<()> {
    let data: &str = slice.as_ref();
    
    if data.len() == 0 {
      // No need, already done
      return Maybe::Some(());
    }

    let Some(new_len) = self.len.checked_add(data.len()) else {
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

        let new_ptr = libc::realloc(self.ptr as _, cap);

        if new_ptr.is_null() {
          return Maybe::None;
        }

        self.ptr = new_ptr as *mut u8;
      }
    }

    // Push from the index self.len upto self.new_len
    unsafe {
      let src_vect = data.as_bytes();
      let src = src_vect.as_ptr();

      copy_nonoverlapping(src, self.ptr.add(self.len), new_len - self.len);
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
  /// - **CRITICAL** You must ensure that this points to valid UTF8 data (which it should)
  /// - **CRITICAL** you must ensure that the bounds check is correct
  pub unsafe fn get_at_unchecked(&self, index: Range<usize>) -> &str {
    let dst = self.ptr;

    let slice = unsafe { slice::from_raw_parts(dst.add(index.start), index.len()) };

    unsafe {
      str::from_utf8_unchecked(slice)
    }
  }

  /// This does not realloc and returns Maybe::None, if the total size is less than cap or if its invalid utf8
  pub fn get_at(&self, index: Range<usize>) -> Maybe<&str> {
    let dst = self.ptr;

    let slice = unsafe { slice::from_raw_parts(dst.add(index.start), index.len()) };

    // Checks if utf8 or not
    str::from_utf8(slice).ok().into()
  }

  // This does not realloc and returns Maybe::None, if the total size is less than cap
  pub fn edit(&mut self, start: usize, src: impl AsRef<str>) -> Maybe<()> {
    unsafe {
      let src = src.as_ref().as_bytes();

      // Start is an index, so start + src.len() is also an index. So add `+1`
      let to_be_set_as_len = (start + src.len()).max(self.len);

      // Allow append via edit
      // Reallocate if necessary
      if self.cap < to_be_set_as_len {
        let Some(new_cap) = to_be_set_as_len.checked_mul(2) else {
          return Maybe::None;
        };

        // Reallocate
        {
          let ptr = realloc(self.ptr as _, new_cap);

          if ptr.is_null() {
            return Maybe::None;
          }

          self.ptr = ptr as _;
          self.cap = new_cap;
        }
      }

      self.len = to_be_set_as_len;

      copy_nonoverlapping(src.as_ptr(), self.ptr.add(start) as _, src.len());
    }

    Maybe::Some(())
  }

  pub fn to_str<'a>(&'a self) -> Maybe<&'a str> {
    if self.ptr.is_null() {
      return Maybe::Some("");
    }

    let bytes = unsafe { slice::from_raw_parts(self.ptr, self.len) };

    // Guaranteed to be valid UTF-8
    str::from_utf8(bytes).ok().into()
  }

  pub fn try_clone(&self) -> Maybe<Self> {
    Self::new(
      match self.to_str() {
        Maybe::Some(x) => x,
        _ => {
          return Maybe::None;
        }
      }
    )
  }

  pub unsafe fn to_str_unchecked<'a>(&'a self) -> &'a str {
    if self.ptr.is_null() {
      return "";
    }

    let bytes = unsafe { slice::from_raw_parts(self.ptr, self.len) };

    // Guaranteed to be valid UTF-8
    unsafe { str::from_utf8_unchecked(bytes) }
  }
}

impl Index<usize> for FFISafeString {
  type Output = str;

  fn index(&self, index: usize) -> &Self::Output {
    self.get_at(
      Range { start: index, end: index + 1 }
    ).unwrap()
  }
}

impl Index<Range<usize>> for FFISafeString {
  type Output = str;

  fn index(&self, index: Range<usize>) -> &Self::Output {
    self.get_at(index).unwrap()
  }
}

impl Display for FFISafeString {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    Display::fmt(self.deref(), f)
  }
}

impl Deref for FFISafeString {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.to_str().unwrap()
  }
}

impl Drop for FFISafeString {
  fn drop(&mut self) {
    if !self.ptr.is_null() {
      unsafe { libc::free(self.ptr as *mut c_void) }
    }
  }
}