#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use std::{collections::HashMap, num::NonZeroU16, ptr};
use pastey::paste;
use critical::RustVariable;
pub mod common {
    use std::ffi::{CStr, CString, c_char};
    use crate::{CVariable, critical::RustVariable};
    #[repr(C)]
    pub struct CommonString {
        ptr: *mut c_char,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CommonString {
        #[inline]
        fn clone(&self) -> CommonString {
            CommonString {
                ptr: ::core::clone::Clone::clone(&self.ptr),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CommonString {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "CommonString",
                "ptr",
                &&self.ptr,
            )
        }
    }
    impl CommonString {
        #[unsafe(no_mangle)]
        pub extern "C" fn create(
            data: *mut c_char,
            _clone: extern "C" fn(data: *mut ()) -> CVariable,
            _drop: extern "C" fn(data: *mut ()),
        ) -> Self {
            let data = unsafe { CStr::from_ptr(data) };
            let data = CString::from(data);
            let ptr = data.into_raw();
            Self { ptr }
        }
        pub unsafe fn to_cstring_ref(&self) -> &CStr {
            unsafe { CStr::from_ptr(self.ptr) }
        }
        pub unsafe fn to_cstring_owned(self) -> CString {
            unsafe { CString::from_raw(self.ptr) }
        }
        pub unsafe fn unsafe_from_mut<'a>(c: *mut ()) -> &'a mut CommonString {
            unsafe { RustVariable::from_ptr_mut(c) }
        }
        pub unsafe fn unsafe_from<'a>(c: *mut ()) -> &'a CommonString {
            unsafe { RustVariable::from_ptr(c) }
        }
    }
    impl Into<CVariable> for CommonString {
        fn into(self) -> CVariable {
            RustVariable::new(self).to_c()
        }
    }
}
/// If we mess up any of these structs, we're dead
pub mod critical {
    use std::{
        mem::ManuallyDrop, ops::{Deref, DerefMut},
        ptr,
    };
    use crate::CVariable;
    pub struct RustVariable<T: Clone>(T);
    #[automatically_derived]
    impl<T: ::core::clone::Clone + Clone> ::core::clone::Clone for RustVariable<T> {
        #[inline]
        fn clone(&self) -> RustVariable<T> {
            RustVariable(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<T: Clone> Deref for RustVariable<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T: Clone> DerefMut for RustVariable<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<T: Clone> RustVariable<T> {
        pub fn new(t: T) -> Self {
            Self(t)
        }
        pub unsafe fn from_ptr_mut<'a>(ptr: *mut ()) -> &'a mut Self {
            unsafe { &mut *(ptr as *mut RustVariable<T>) }
        }
        pub unsafe fn from_ptr<'a>(ptr: *mut ()) -> &'a Self {
            unsafe { &*(ptr as *mut RustVariable<T>) }
        }
        pub fn to_c(self) -> CVariable {
            let mut data = ManuallyDrop::new(self);
            let ptr: *mut RustVariable<T> = &mut *data;
            CVariable {
                data: ptr as *mut (),
                _drop: _drop::<RustVariable<T>>,
                _clone: _clone::<RustVariable<T>>,
            }
        }
    }
    extern "C" fn _drop<T>(data: *mut ()) {
        unsafe {
            let data = &mut *(data as *mut T);
            ptr::drop_in_place(data);
        }
    }
    extern "C" fn _clone<T: Clone>(data: *mut ()) -> CVariable {
        unsafe {
            let data = &mut *(data as *mut T);
            let cloned = data.clone();
            RustVariable::new(cloned).to_c()
        }
    }
}
#[macro_use]
mod macros {}
pub const VERSION: u16 = 0;
#[repr(C)]
pub enum Maybe<T> {
    Some(T),
    None,
}
use Maybe::None as MaybeNone;
use Maybe::Some as MaybeSome;
#[repr(C)]
pub struct FnStack {
    /// Return value (identifier in MemoryMap)
    pub ret: Maybe<CVariable>,
    pub(crate) r1: Option<NonZeroU16>,
    pub(crate) r2: Option<NonZeroU16>,
    pub(crate) r3: Option<NonZeroU16>,
    pub(crate) r4: Option<NonZeroU16>,
    pub(crate) r5: Option<NonZeroU16>,
    pub(crate) r6: Option<NonZeroU16>,
    pub(crate) r7: Option<NonZeroU16>,
    pub(crate) r8: Option<NonZeroU16>,
}
impl FnStack {
    #[unsafe(no_mangle)]
    pub extern "C" fn create() -> Self {
        Self {
            ret: MaybeNone,
            r1: None,
            r2: None,
            r3: None,
            r4: None,
            r5: None,
            r6: None,
            r7: None,
            r8: None,
        }
    }
    pub fn clear(&mut self) {}
    #[unsafe(no_mangle)]
    pub extern "C" fn setOutput(&mut self, data: CVariable) {
        self.ret = MaybeSome(data);
    }
}
#[repr(C)]
pub struct CVariable {
    pub data: *mut (),
    pub _drop: extern "C" fn(*mut ()),
    pub _clone: extern "C" fn(*mut ()) -> CVariable,
}
#[repr(C)]
pub struct Wrapper<T: Copy + Clone>(T);
#[automatically_derived]
impl<T: ::core::fmt::Debug + Copy + Clone> ::core::fmt::Debug for Wrapper<T> {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Wrapper", &&self.0)
    }
}
#[automatically_derived]
impl<T: ::core::clone::Clone + Copy + Clone> ::core::clone::Clone for Wrapper<T> {
    #[inline]
    fn clone(&self) -> Wrapper<T> {
        Wrapper(::core::clone::Clone::clone(&self.0))
    }
}
impl CVariable {
    #[unsafe(no_mangle)]
    pub extern "C" fn from_c_to_rustu8(value: Wrapper<u8>) -> Self {
        RustVariable::new(value).to_c()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn from_c_to_rustu16(value: Wrapper<u16>) -> Self {
        RustVariable::new(value).to_c()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn from_c_to_rustu32(value: Wrapper<u32>) -> Self {
        RustVariable::new(value).to_c()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn from_c_to_rustu64(value: Wrapper<u64>) -> Self {
        RustVariable::new(value).to_c()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn from_c_to_rusti8(value: Wrapper<i8>) -> Self {
        RustVariable::new(value).to_c()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn from_c_to_rusti16(value: Wrapper<i16>) -> Self {
        RustVariable::new(value).to_c()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn from_c_to_rusti32(value: Wrapper<i32>) -> Self {
        RustVariable::new(value).to_c()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn from_c_to_rusti64(value: Wrapper<i64>) -> Self {
        RustVariable::new(value).to_c()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn from_c_to_rustf32(value: Wrapper<f32>) -> Self {
        RustVariable::new(value).to_c()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn from_c_to_rustf64(value: Wrapper<f64>) -> Self {
        RustVariable::new(value).to_c()
    }
}
impl From<Wrapper<u8>> for CVariable {
    fn from(value: Wrapper<u8>) -> Self {
        RustVariable::new(value).to_c()
    }
}
impl From<Wrapper<u16>> for CVariable {
    fn from(value: Wrapper<u16>) -> Self {
        RustVariable::new(value).to_c()
    }
}
impl From<Wrapper<u32>> for CVariable {
    fn from(value: Wrapper<u32>) -> Self {
        RustVariable::new(value).to_c()
    }
}
impl From<Wrapper<u64>> for CVariable {
    fn from(value: Wrapper<u64>) -> Self {
        RustVariable::new(value).to_c()
    }
}
impl From<Wrapper<i8>> for CVariable {
    fn from(value: Wrapper<i8>) -> Self {
        RustVariable::new(value).to_c()
    }
}
impl From<Wrapper<i16>> for CVariable {
    fn from(value: Wrapper<i16>) -> Self {
        RustVariable::new(value).to_c()
    }
}
impl From<Wrapper<i32>> for CVariable {
    fn from(value: Wrapper<i32>) -> Self {
        RustVariable::new(value).to_c()
    }
}
impl From<Wrapper<i64>> for CVariable {
    fn from(value: Wrapper<i64>) -> Self {
        RustVariable::new(value).to_c()
    }
}
impl From<Wrapper<f32>> for CVariable {
    fn from(value: Wrapper<f32>) -> Self {
        RustVariable::new(value).to_c()
    }
}
impl From<Wrapper<f64>> for CVariable {
    fn from(value: Wrapper<f64>) -> Self {
        RustVariable::new(value).to_c()
    }
}
#[repr(C)]
pub struct RustClosure {
    _closure: extern "C" fn(*mut ()),
}
extern "C" fn drop_noop(_: *mut ()) {}
extern "C" fn clone_noop(_: *mut ()) -> CVariable {
    unsafe { CVariable::null() }
}
impl Drop for CVariable {
    fn drop(&mut self) {
        (self._drop)(self.data)
    }
}
impl Clone for CVariable {
    fn clone(&self) -> Self {
        (self._clone)(self.data)
    }
}
impl CVariable {
    pub unsafe fn null() -> Self {
        Self {
            data: ptr::null_mut(),
            _drop: drop_noop,
            _clone: clone_noop,
        }
    }
    pub unsafe fn into_rust(self) -> Option<VariableData> {
        Some(VariableData::Raw(self))
    }
}
pub struct MemoryMap {
    pub variables: HashMap<u16, VariableData>,
}
pub enum VariableData {
    Constant(&'static str),
    Raw(CVariable),
}
