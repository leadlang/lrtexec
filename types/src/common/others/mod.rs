use std::ffi::CString;
use std::fmt;
use std::os::raw::c_char;
use std::ptr; // For ptr::null_mut() and ptr::copy()
use std::slice; // For slice::from_raw_parts_mut()

pub mod boxes;

/// A C-compatible string for FFI.
///
/// This struct manages a null-terminated C-style string (`*mut c_char`)
/// and its length (excluding the null terminator). It ensures that
/// memory is properly allocated, deallocated, and reallocated when
/// the string content changes.
#[repr(C)]
pub struct FFISafeString {
  ptr: *mut c_char,
  len: usize,
  capacity: usize,
}

impl FFISafeString {
  /// Creates a new, empty `FFISafeString`.
  pub fn new() -> Self {
    Self {
      ptr: ptr::null_mut(),
      len: 0,
      capacity: 0,
    }
  }

  /// Creates an `FFISafeString` from a Rust `&str`.
  ///
  /// This allocates a new C-compatible string and copies the content.
  pub fn from<T: Into<Vec<u8>>>(s: T) -> Self {
    let cstring = CString::new(s).expect("String contains null bytes");
    let len = cstring.as_bytes().len(); // Length without null terminator
    let capacity = len + 1; // Capacity including null terminator

    let ptr = cstring.into_raw(); // Consumes CString, gives raw pointer

    Self { ptr, len, capacity }
  }

  /// Appends a string slice to this `FFISafeString`.
  ///
  /// This method reallocates memory if the new content exceeds the current capacity.
  pub fn push_str(&mut self, s: &str) {
    let additional_len = s.len();
    let new_len = self.len + additional_len;

    if new_len >= self.capacity {
      // Reallocate: Choose a new capacity (e.g., double or more if needed)
      let new_capacity = (new_len + 1).max(self.capacity * 2).max(16); // +1 for null terminator, min capacity 16

      let new_ptr = if self.ptr.is_null() {
        // Initial allocation
        unsafe { libc::malloc(new_capacity) as *mut c_char }
      } else {
        // Reallocation
        unsafe { libc::realloc(self.ptr as *mut libc::c_void, new_capacity) as *mut c_char }
      };

      if new_ptr.is_null() {
        panic!("Failed to reallocate memory for FFISafeString");
      }

      self.ptr = new_ptr;
      self.capacity = new_capacity;
    }

    // Copy the new string data
    unsafe {
      let dest = self.ptr.add(self.len) as *mut u8;
      let src = s.as_ptr();
      ptr::copy_nonoverlapping(src, dest, additional_len);
    }

    self.len = new_len;

    // Ensure null termination
    unsafe {
      *self.ptr.add(self.len) = 0;
    }
  }

  /// Returns a `&str` slice of the `FFISafeString` content.
  ///
  /// This is a safe way to view the string within Rust.
  pub fn as_str(&self) -> Option<&str> {
    if self.ptr.is_null() || self.len == 0 {
      return None;
    }
    unsafe {
      let slice = slice::from_raw_parts(self.ptr as *const u8, self.len);
      std::str::from_utf8(slice).ok()
    }
  }

  /// Returns a `*const c_char` to the internal C string.
  ///
  /// This is typically what you'd pass to C functions.
  pub fn as_ptr(&self) -> *const c_char {
    self.ptr
  }

  /// Returns the length of the string in bytes, excluding the null terminator.
  pub fn len(&self) -> usize {
    self.len
  }

  /// Returns true if the string has a length of 0.
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }
}

// --- Trait Implementations ---

impl fmt::Debug for FFISafeString {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let s = self.as_str().unwrap_or("[invalid UTF-8 or null]");
    f.debug_struct("FFISafeString")
      .field("ptr", &self.ptr)
      .field("len", &self.len)
      .field("capacity", &self.capacity)
      .field("value", &s)
      .finish()
  }
}

impl fmt::Display for FFISafeString {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.as_str() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "{}", "[invalid UTF-8 or null string]"),
    }
  }
}

// --- Memory Management (Drop Trait) ---

impl Drop for FFISafeString {
  fn drop(&mut self) {
    if !self.ptr.is_null() {
      // Free the memory allocated by malloc/realloc
      unsafe {
        libc::free(self.ptr as *mut libc::c_void);
      }
      self.ptr = ptr::null_mut(); // Prevent double-free
      self.len = 0;
      self.capacity = 0;
    }
  }
}
