use libc::{free, malloc};
use std::ffi::c_void;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

#[repr(C)]
pub struct Boxed<T> {
  ptr: NonNull<(T, extern "C" fn(data: *mut c_void))>,
  drop: bool,
}

extern "C" fn ptr_drop<T>(data: *mut c_void) {
  unsafe {
    ptr::drop_in_place(data as *mut T);
  }
}

impl<T> Boxed<T> {
  /// Creates a new Boxed containing the provided value.
  ///
  /// This function manually allocates memory on the heap using `libc::malloc`
  /// and moves the value into that newly allocated space.
  pub extern "C" fn new(value: T) -> Self {
    let drop = ptr_drop::<T>;
    let data: (T, extern "C" fn(*mut c_void)) = (value, drop);

    let size = size_of_val(&data);

    if size == 0 {
      return Boxed {
        ptr: NonNull::dangling(),
        drop: true,
      };
    }

    let ptr = unsafe { malloc(size) as *mut (T, extern "C" fn (_: *mut c_void)) };

    let ptr = match NonNull::new(ptr) {
      Some(p) => p,
      None => panic!("Failed to allocate memory"),
    };

    unsafe {
      ptr::write(ptr.as_ptr(), data);
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

    value.0
  }

  /// Creates an Boxed from a raw, non-null pointer.
  ///
  /// # Safety
  /// The caller must guarantee that the pointer is valid, non-null,
  /// and points to a value of type T that was allocated with `libc::malloc` i.e. by `Boxed::<T>::new`` or equivalent.
  /// The caller also takes responsibility for ensuring that the data is not
  /// freed elsewhere.
  pub unsafe extern "C" fn from_raw(ptr: *mut c_void) -> Self {
    Self {
      ptr: NonNull::new(ptr as *mut (T, extern "C" fn(_: *mut c_void))).expect("Invalid Pointer provided"),
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
    unsafe { &self.ptr.as_ref().0 }
  }
}

impl<T> DerefMut for Boxed<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut self.ptr.as_mut().0 }
  }
}

impl<T> Drop for Boxed<T> {
  fn drop(&mut self) {
    if mem::size_of::<T>() > 0 && self.drop {
      // Runs drop functions of the values
      unsafe {
        let ptr = &mut *self.ptr.as_ptr();

        (ptr.1)(&mut ptr.0 as *mut _ as *mut _)
      }

      // Free the pointers + value like C
      unsafe {
        free(self.ptr.as_ptr() as *mut _);
      }
    }
  }
}
