use libc::{free, malloc};
use std::ffi::c_void;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

#[repr(C)]
pub struct Boxed<T> {
  ptr: NonNull<T>,
  drop: bool,
}

impl<T> Boxed<T> {
  /// Creates a new Boxed containing the provided value.
  ///
  /// This function manually allocates memory on the heap using `libc::malloc`
  /// and moves the value into that newly allocated space.
  pub extern "C" fn new(value: T) -> Self {
    let size = mem::size_of::<T>();

    if size == 0 {
      return Boxed {
        ptr: NonNull::dangling(),
        drop: true,
      };
    }

    let ptr = unsafe { malloc(size) as *mut T };

    let ptr = match NonNull::new(ptr) {
      Some(p) => p,
      None => panic!("Failed to allocate memory"),
    };

    unsafe {
      ptr::write(ptr.as_ptr(), value);
    }

    Self { ptr, drop: true }
  }

  /// Consumes the FfiBox and returns the value inside.
  ///
  /// This will deallocate the memory used by the box but not the value itself.
  pub extern "C" fn unbox(mut self) -> T {
    let ptr = self.ptr.as_ptr();
    self.drop = false;

    // # Safety
    // Its valid
    let value = unsafe { ptr::read(ptr) };

    // Now, we must manually deallocate the memory.
    if mem::size_of::<T>() > 0 {
      // # Safety
      // This is only freeing the pointer struct
      unsafe {
        free(ptr as *mut c_void);
      }
    }

    value
  }

  /// Creates an Boxed from a raw, non-null pointer.
  ///
  /// # Safety
  /// The caller must guarantee that the pointer is valid, non-null,
  /// and points to a value of type T that was allocated with `libc::malloc` i.e. by `Boxed::<T>::new`` or equivalent.
  /// The caller also takes responsibility for ensuring that the data is not
  /// freed elsewhere.
  pub unsafe extern "C" fn from_raw(ptr: *mut T) -> Self {
    Self {
      ptr: NonNull::new(ptr as *mut T).expect("Invalid Pointer provided"),
      drop: true,
    }
  }

  pub extern "C" fn into_raw(mut val: Self) -> *mut c_void {
    val.drop = false;

    val.ptr.as_ptr() as _
  }
}

impl<T> Deref for Boxed<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    unsafe { self.ptr.as_ref() }
  }
}

impl<T> DerefMut for Boxed<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { self.ptr.as_mut() }
  }
}

impl<T> Drop for Boxed<T> {
  fn drop(&mut self) {
    if mem::size_of::<T>() > 0 && self.drop {
      // Runs drop functions of the values
      unsafe {
        ptr::drop_in_place(self.ptr.as_ptr() as *mut c_void);
      }

      // Free the pointers + value like C
      unsafe {
        free(self.ptr.as_ptr() as *mut _);
      }
    }
  }
}
