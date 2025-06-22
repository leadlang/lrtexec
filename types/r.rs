#![feature(prelude_import)]
#![feature(trivial_bounds)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod commands {
    use lrt_macros::ver;
    pub mod v0 {
        use lrt_macros::declare;
        pub mod compat {
            //! **v0** Provides no compatibility
        }
        pub mod structs {
            //! Structs related to v0 commands
            use std::os::raw::c_void;
            use crate::common::FFIableObject;
            #[repr(C)]
            pub enum VariableDataV0 {
                Inbuilt(Container),
                Object(FFIableObject),
            }
            #[repr(C)]
            pub struct Container {
                pub data: *mut c_void,
                pub drop: extern "C" fn(*mut c_void),
                pub id: u8,
            }
            impl Drop for Container {
                fn drop(&mut self) {
                    (self.drop)(self.data)
                }
            }
            extern "C" fn general_drop<T>(ptrr: *mut c_void) {
                unsafe {
                    _ = Box::from_raw(ptrr);
                }
            }
            impl Into<Container> for u8 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<u8>,
                        id: 0,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_u8(&self) -> Option<&u8> {
                    if self.id != 0 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut u8)) }
                }
                /// Returns `None` is types do not match
                pub fn as_u8_mut(&self) -> Option<&mut u8> {
                    if self.id != 0 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut u8)) }
                }
            }
            impl Into<Container> for u16 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<u16>,
                        id: 1,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_u16(&self) -> Option<&u16> {
                    if self.id != 1 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut u16)) }
                }
                /// Returns `None` is types do not match
                pub fn as_u16_mut(&self) -> Option<&mut u16> {
                    if self.id != 1 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut u16)) }
                }
            }
            impl Into<Container> for u32 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<u32>,
                        id: 2,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_u32(&self) -> Option<&u32> {
                    if self.id != 2 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut u32)) }
                }
                /// Returns `None` is types do not match
                pub fn as_u32_mut(&self) -> Option<&mut u32> {
                    if self.id != 2 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut u32)) }
                }
            }
            impl Into<Container> for u64 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<u64>,
                        id: 3,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_u64(&self) -> Option<&u64> {
                    if self.id != 3 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut u64)) }
                }
                /// Returns `None` is types do not match
                pub fn as_u64_mut(&self) -> Option<&mut u64> {
                    if self.id != 3 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut u64)) }
                }
            }
            impl Into<Container> for u128 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<u128>,
                        id: 4,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_u128(&self) -> Option<&u128> {
                    if self.id != 4 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut u128)) }
                }
                /// Returns `None` is types do not match
                pub fn as_u128_mut(&self) -> Option<&mut u128> {
                    if self.id != 4 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut u128)) }
                }
            }
            impl Into<Container> for i8 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<i8>,
                        id: 5,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_i8(&self) -> Option<&i8> {
                    if self.id != 5 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut i8)) }
                }
                /// Returns `None` is types do not match
                pub fn as_i8_mut(&self) -> Option<&mut i8> {
                    if self.id != 5 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut i8)) }
                }
            }
            impl Into<Container> for i16 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<i16>,
                        id: 6,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_i16(&self) -> Option<&i16> {
                    if self.id != 6 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut i16)) }
                }
                /// Returns `None` is types do not match
                pub fn as_i16_mut(&self) -> Option<&mut i16> {
                    if self.id != 6 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut i16)) }
                }
            }
            impl Into<Container> for i32 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<i32>,
                        id: 7,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_i32(&self) -> Option<&i32> {
                    if self.id != 7 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut i32)) }
                }
                /// Returns `None` is types do not match
                pub fn as_i32_mut(&self) -> Option<&mut i32> {
                    if self.id != 7 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut i32)) }
                }
            }
            impl Into<Container> for i64 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<i64>,
                        id: 8,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_i64(&self) -> Option<&i64> {
                    if self.id != 8 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut i64)) }
                }
                /// Returns `None` is types do not match
                pub fn as_i64_mut(&self) -> Option<&mut i64> {
                    if self.id != 8 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut i64)) }
                }
            }
            impl Into<Container> for i128 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<i128>,
                        id: 9,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_i128(&self) -> Option<&i128> {
                    if self.id != 9 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut i128)) }
                }
                /// Returns `None` is types do not match
                pub fn as_i128_mut(&self) -> Option<&mut i128> {
                    if self.id != 9 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut i128)) }
                }
            }
            impl Into<Container> for f32 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<f32>,
                        id: 10,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_f32(&self) -> Option<&f32> {
                    if self.id != 10 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut f32)) }
                }
                /// Returns `None` is types do not match
                pub fn as_f32_mut(&self) -> Option<&mut f32> {
                    if self.id != 10 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut f32)) }
                }
            }
            impl Into<Container> for f64 {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<f64>,
                        id: 11,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_f64(&self) -> Option<&f64> {
                    if self.id != 11 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut f64)) }
                }
                /// Returns `None` is types do not match
                pub fn as_f64_mut(&self) -> Option<&mut f64> {
                    if self.id != 11 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut f64)) }
                }
            }
            impl Into<Container> for bool {
                fn into(self) -> Container {
                    let data = Box::new(self);
                    let d = Box::into_raw(data);
                    Container {
                        data: d as *mut c_void,
                        drop: general_drop::<bool>,
                        id: 12,
                    }
                }
            }
            impl Container {
                /// Returns `None` is types do not match
                pub fn as_bool(&self) -> Option<&bool> {
                    if self.id != 12 {
                        return None;
                    }
                    unsafe { Some(&*(self.data as *mut bool)) }
                }
                /// Returns `None` is types do not match
                pub fn as_bool_mut(&self) -> Option<&mut bool> {
                    if self.id != 12 {
                        return None;
                    }
                    unsafe { Some(&mut *(self.data as *mut bool)) }
                }
            }
        }
        pub struct ScriptV0 {}
        pub type Script = ScriptV0;
        pub fn cmd_to_int(cmd: &str, vect: &mut Vec<u8>) -> Option<()> {
            match cmd {
                "extend" => {
                    vect.push(1u8);
                    Some(())
                }
                "set" => {
                    vect.push(2u8);
                    Some(())
                }
                "loadfromreg" => {
                    vect.push(3u8);
                    Some(())
                }
                "regset" => {
                    vect.push(4u8);
                    Some(())
                }
                "regload" => {
                    vect.push(5u8);
                    Some(())
                }
                "dlopen" => {
                    vect.push(6u8);
                    Some(())
                }
                "drop" => {
                    vect.push(7u8);
                    Some(())
                }
                "hello" => {
                    vect.push(0u8);
                    vect.push(1u8);
                    vect.push(0u8);
                    Some(())
                }
                _ => None,
            }
        }
        pub fn handle_line(s: &mut Script) {}
    }
}
pub mod common {
    use std::{
        ffi::{CStr, CString, c_char, c_void},
        fmt::{Debug, Display},
        marker::PhantomData,
    };
    pub mod variables {
        pub mod v0 {}
    }
    #[repr(C)]
    pub struct CommonString {
        data: *mut c_char,
        drop: extern "C" fn(*mut c_char),
    }
    extern "C" fn common_string_drop(ptr: *mut c_char) {
        unsafe {
            drop(CString::from_raw(ptr));
        }
    }
    impl Into<CommonString> for String {
        fn into(self) -> CommonString {
            let cstring = CString::new(self).unwrap();
            let data = cstring.into_raw();
            CommonString {
                data,
                drop: common_string_drop,
            }
        }
    }
    impl AsRef<CStr> for CommonString {
        fn as_ref(&self) -> &CStr {
            unsafe { CStr::from_ptr(self.data) }
        }
    }
    impl Drop for CommonString {
        fn drop(&mut self) {
            (self.drop)(self.data)
        }
    }
    #[repr(C)]
    pub struct FFIableObject {
        data: *mut c_void,
        drop: extern "C" fn(*mut c_void),
        fmt: extern "C" fn(*mut c_void) -> CommonString,
        display: extern "C" fn(*mut c_void) -> CommonString,
    }
    #[repr(C)]
    pub struct WrappedFFIableObject<'a, T> {
        object: *mut FFIableObject,
        r#type: PhantomData<&'a T>,
    }
    impl<'a, T> WrappedFFIableObject<'a, T> {
        pub fn create_using_box<E: Debug + Display>(data: E) -> (Self, FFIableObject) {
            let mut object = FFIableObject::create_using_box(data);
            let data = Self::create_from_object(&mut object);
            (data, object)
        }
        pub fn create_using_box_no_display<E: Debug>(data: E) -> (Self, FFIableObject) {
            let mut object = FFIableObject::create_using_box_no_display(data);
            let data = Self::create_from_object(&mut object);
            (data, object)
        }
        pub fn create_from_object<'e>(object: &'e mut FFIableObject) -> Self {
            Self {
                object,
                r#type: PhantomData,
            }
        }
        fn get_ptr(&self) -> &mut FFIableObject {
            unsafe { &mut *self.object }
        }
        pub unsafe fn get(&'a self) -> &'a T {
            unsafe { self.get_ptr().get() }
        }
        pub unsafe fn get_mut(&'a mut self) -> &'a T {
            unsafe { self.get_ptr().get_mut() }
        }
    }
    extern "C" fn general_drop<T>(ptr: *mut c_void) {
        unsafe {
            drop(Box::from_raw(ptr as *mut T));
        }
    }
    extern "C" fn general_display<T: Display>(ptr: *mut c_void) -> CommonString {
        unsafe {
            let data = &*(ptr as *mut T);
            let fmt = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", data))
            });
            fmt.into()
        }
    }
    extern "C" fn general_debug<T: Debug>(ptr: *mut c_void) -> CommonString {
        unsafe {
            let data = &*(ptr as *mut T);
            let fmt = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0:?}", data))
            });
            fmt.into()
        }
    }
    extern "C" fn no_display(_: *mut c_void) -> CommonString {
        ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("<cannot display type>"))
            })
            .into()
    }
    impl FFIableObject {
        pub unsafe fn get_mut<'a, T>(&'a mut self) -> &'a mut T {
            unsafe { &mut *(self.data as *mut T) }
        }
        pub unsafe fn get<'a, T>(&'a self) -> &'a T {
            unsafe { &*(self.data as *mut T) }
        }
        pub fn create_using_box<T: Debug + Display>(data: T) -> Self {
            let data = Box::new(data);
            let data = Box::into_raw(data);
            Self {
                data: data as *mut c_void,
                display: general_display::<T>,
                drop: general_drop::<T>,
                fmt: general_debug::<T>,
            }
        }
        pub fn create_using_box_no_display<T: Debug>(data: T) -> Self {
            let data = Box::new(data);
            let data = Box::into_raw(data);
            Self {
                data: data as *mut c_void,
                display: no_display,
                drop: general_drop::<T>,
                fmt: general_debug::<T>,
            }
        }
    }
    impl Display for FFIableObject {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let data = (self.display)(self.data);
            let data = data.as_ref();
            let data = data.to_str();
            let Ok(data) = data else {
                return Err(std::fmt::Error::default());
            };
            f.write_str(data)
        }
    }
    impl Debug for FFIableObject {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let data = (self.fmt)(self.data);
            let data = data.as_ref();
            let data = data.to_str();
            let Ok(data) = data else {
                return Err(std::fmt::Error::default());
            };
            f.write_str(data)
        }
    }
    impl Drop for FFIableObject {
        fn drop(&mut self) {
            (self.drop)(self.data)
        }
    }
}
