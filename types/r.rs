#![feature(prelude_import)]
#![feature(trivial_bounds, unsafe_cell_access)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::os::raw::c_void;
pub mod commands {
    use lrt_macros::ver;
    pub trait FFISafeContainer {}
    pub mod v0 {
        use lrt_macros::declare;
        pub mod compat {
            //! **v0** Provides no compatibility
        }
        pub mod structs {
            //! # v0
            //! This crate defines the `v0` of the Assembly Syntax of LRTEXEC Bytecode (also mentioned as assembly in many places)
            //!
            //! Structs related to v0 Assembly Syntax
            use std::{cell::UnsafeCell, fmt::Debug, mem::replace};
            use crate::common::{
                r#async::{AsyncInterface, ReturnReg},
                FFIableObject,
            };
            #[repr(C)]
            pub struct FnStackV0 {
                /// Return value (identifier in MemoryMap)
                pub ret: UnsafeCell<ReturnReg<FFIableObject>>,
                /// Registers
                pub r1: WrapperRegValueV0,
                pub r2: WrapperRegValueV0,
                pub r3: WrapperRegValueV0,
                pub r4: WrapperRegValueV0,
                pub r5: WrapperRegValueV0,
                pub r6: WrapperRegValueV0,
                pub r7: WrapperRegValueV0,
                pub r8: WrapperRegValueV0,
            }
            impl FnStackV0 {
                /// Set either the output or the async task, doing both will override one of the results
                pub fn set_output(&self, out: FFIableObject) {
                    *unsafe { self.ret.as_mut_unchecked() } = ReturnReg::Output(out);
                }
                /// Set either the output or the async task, doing both will override one of the results
                pub fn set_async_task(&self, out: AsyncInterface<FFIableObject>) {
                    *unsafe { self.ret.as_mut_unchecked() } = ReturnReg::Async(out);
                }
                pub fn postrun(&mut self) -> Option<ReturnReg<FFIableObject>> {
                    let ret_to_replace = ReturnReg::<FFIableObject>::Null;
                    let ret = replace(self.ret.get_mut(), ret_to_replace);
                    {
                        self.r1._inner = RegValueV0::Null;
                        self.r2._inner = RegValueV0::Null;
                        self.r3._inner = RegValueV0::Null;
                        self.r4._inner = RegValueV0::Null;
                        self.r5._inner = RegValueV0::Null;
                        self.r6._inner = RegValueV0::Null;
                        self.r7._inner = RegValueV0::Null;
                        self.r8._inner = RegValueV0::Null;
                    }
                    if #[allow(non_exhaustive_omitted_patterns)]
                    match ret {
                        ReturnReg::Null => true,
                        _ => false,
                    } {
                        return None;
                    }
                    Some(ret)
                }
            }
            pub static REGISTER_R1: u16 = 0;
            pub static REGISTER_R2: u16 = 1;
            pub static REGISTER_R3: u16 = 2;
            pub static REGISTER_R4: u16 = 3;
            pub static REGISTER_R5: u16 = 4;
            pub static REGISTER_R6: u16 = 5;
            pub static REGISTER_R7: u16 = 6;
            pub static REGISTER_R8: u16 = 7;
            pub static REGISTER_RET: u16 = 8;
            #[repr(C)]
            pub enum RegValueV0 {
                Moved(*mut FFIableObject),
                Mut(*mut FFIableObject),
                Ref(*const FFIableObject),
                #[default]
                Null,
            }
            #[automatically_derived]
            impl ::core::default::Default for RegValueV0 {
                #[inline]
                fn default() -> RegValueV0 {
                    Self::Null
                }
            }
            #[repr(C)]
            pub struct WrapperRegValueV0 {
                pub(crate) _inner: RegValueV0,
            }
            #[automatically_derived]
            impl ::core::default::Default for WrapperRegValueV0 {
                #[inline]
                fn default() -> WrapperRegValueV0 {
                    WrapperRegValueV0 {
                        _inner: ::core::default::Default::default(),
                    }
                }
            }
            impl WrapperRegValueV0 {
                /// Get a shared reference to the inner `FFIableObject` if it isn't null.
                ///
                /// # Safety
                ///
                /// This function is unsafe because it relies on the correctness of the `RegValueV0` enum
                /// instance and the provided type `T` by the caller. If the enum instance is `Null`, this function will return `None`. If it is
                /// not `Null`, this function will return a shared reference to the `FFIableObject`
                /// stored in the enum instance.
                pub unsafe fn get_ptr<T>(&self) -> Option<&T> {
                    unsafe {
                        Some(
                            match self._inner {
                                RegValueV0::Moved(ptr) => (&*ptr).get(),
                                RegValueV0::Mut(ptr) => (&*ptr).get(),
                                RegValueV0::Ref(ptr) => (&*ptr).get(),
                                RegValueV0::Null => return None,
                            },
                        )
                    }
                }
                /// Get a mutable reference to the inner `FFIableObject` if it isn't null.
                ///
                /// # Safety
                ///
                /// This function is unsafe because it relies on the correctness of the `RegValueV0` enum
                /// instance and the provided type `T` by the caller. If the enum instance is `Null`, this function will return `None`. If it is
                /// not `Null`, this function will return a mutable reference to the `FFIableObject`
                /// stored in the enum instance.
                pub unsafe fn get_ptr_mut<'a, T>(&'a self) -> Option<&'a mut T> {
                    unsafe {
                        Some(
                            match self._inner {
                                RegValueV0::Moved(ptr) => &mut *ptr,
                                RegValueV0::Mut(ptr) => &mut *ptr,
                                _ => return None,
                            }
                                .get_mut::<T>(),
                        )
                    }
                }
                /// Get a mutable reference to the inner `FFIableObject` if it isn't null.
                ///
                /// # Note
                /// You are given a `&mut T` instance of the `FFIableObject` stored in the enum instance.
                ///
                /// # Safety
                ///
                /// This function is unsafe because it relies on the correctness of the `RegValueV0` enum
                /// instance and the provided type `T` by the caller. If the enum instance is `Null`, this function will return `None`. If it is
                /// not `Null`, this function will return a mutable reference to the `FFIableObject`
                /// stored in the enum instance.
                pub unsafe fn remove_ptr(&self) -> Option<FFIableObject> {
                    unsafe {
                        Some(
                            match self._inner {
                                RegValueV0::Moved(ptr) => &mut *ptr,
                                _ => return None,
                            }
                                .transfer_ownership(),
                        )
                    }
                }
                /// Get a mutable reference to the inner `FFIableObject` if it isn't null.
                ///
                /// # Note
                /// You are given a `&mut T` instance of the `FFIableObject` stored in the enum instance.
                ///
                /// # Safety
                ///
                /// This function is unsafe because it relies on the correctness of the `RegValueV0` enum
                /// instance and the provided type `T` by the caller. If the enum instance is `Null`, this function will return `None`. If it is
                /// not `Null`, this function will return a mutable reference to the `FFIableObject`
                /// stored in the enum instance.
                pub unsafe fn remove_ptr_reconstruct<T: Debug>(&self) -> Option<T> {
                    unsafe { Some(self.remove_ptr()?.reconstruct()) }
                }
            }
        }
        pub use super::FFISafeContainer;
        pub fn cmd_to_int(cmd: &str, vect: &mut Vec<u8>) -> Option<()> {
            match cmd {
                "set" => {
                    vect.push(1u8);
                    Some(())
                }
                "drop" => {
                    vect.push(2u8);
                    Some(())
                }
                "regload" => {
                    vect.push(3u8);
                    Some(())
                }
                "regdrop" => {
                    vect.push(4u8);
                    Some(())
                }
                "set" => {
                    vect.push(5u8);
                    Some(())
                }
                "loadfromreg" => {
                    vect.push(6u8);
                    Some(())
                }
                "regset" => {
                    vect.push(7u8);
                    Some(())
                }
                "dlopen" => {
                    vect.push(8u8);
                    Some(())
                }
                "drop" => {
                    vect.push(9u8);
                    Some(())
                }
                "hi" => {
                    vect.push(0u8);
                    vect.push(1u8);
                    vect.push(0u8);
                    Some(())
                }
                _ => None,
            }
        }
    }
}
pub mod common {
    use std::{
        any::TypeId, ffi::c_void, fmt::{Debug, Display},
        ptr::null_mut,
    };
    use crate::{
        commands::FFISafeContainer, common::others::{boxes::Boxed, FFISafeString},
        Maybe,
    };
    pub mod r#async {
        use std::os::raw::c_void;
        use std::{
            fmt::Debug, future::Future, ops::{Deref, DerefMut},
            pin::Pin, task::{Context, Poll},
        };
        use tokio::task::{JoinHandle, spawn_blocking};
        use crate::{commands::FFISafeContainer, common::FFIableObject};
        mod waker {
            use std::{
                env, mem::transmute, os::raw::c_void, path::PathBuf, str::FromStr,
                sync::LazyLock,
            };
            use libloading::{Library, Symbol, library_filename};
            use crate::common::FFIableObject;
            pub static WAKER: LazyLock<Waker> = LazyLock::new(|| Waker::new());
            /// This function consumes the pointer, should be called only once
            pub extern "C" fn call_waker_consume_ptr(waker: *mut c_void) {
                (*WAKER.call_waker_consume_ptr)(waker)
            }
            pub type CreateWaker = extern "C" fn(
                waker: FFIableObject,
                call: extern "C" fn(waker: FFIableObject) -> (),
            ) -> *mut c_void;
            pub type CallWakerConsume = extern "C" fn(waker: *mut c_void);
            pub struct Waker {
                _lib: Library,
                pub(crate) create_waker: Symbol<'static, CreateWaker>,
                pub call_waker_consume_ptr: Symbol<'static, CallWakerConsume>,
            }
            impl Waker {
                pub fn new() -> Self {
                    let lrt = env::var("LRT_HOME").expect("LRT Home not present");
                    let file = library_filename("async_waker");
                    let mut path = PathBuf::from_str(&lrt).unwrap();
                    path.push("libs");
                    path.push("waker");
                    path.push(file);
                    let lib = unsafe {
                        Library::new(path).expect("Unable to load async_waker")
                    };
                    let create: Symbol<'_, CreateWaker> = unsafe {
                        lib.get(b"").unwrap()
                    };
                    let create = unsafe { transmute(create) };
                    let call_consume_ptr: Symbol<
                        '_,
                        extern "C" fn(*mut std::ffi::c_void),
                    > = unsafe { lib.get(b"").unwrap() };
                    let call_consume_ptr: Symbol<
                        'static,
                        extern "C" fn(*mut std::ffi::c_void),
                    > = unsafe { transmute(call_consume_ptr) };
                    Self {
                        _lib: lib,
                        create_waker: create,
                        call_waker_consume_ptr: call_consume_ptr,
                    }
                }
            }
        }
        pub use waker::{WAKER, call_waker_consume_ptr};
        #[repr(C)]
        #[allow(deprecated)]
        pub enum ReturnReg<T: FFISafeContainer + 'static> {
            Output(T),
            Async(AsyncInterface<T>),
            Null,
        }
        #[repr(C)]
        #[allow(deprecated)]
        pub enum AsyncInterface<T: FFISafeContainer + 'static> {
            Threaded(ThreadedTask<T>),
            Lazy(LazyableTask<T>),
            LazyWithWaker(LazyableTaskWithWaker<T>),
        }
        impl<T: FFISafeContainer + 'static> Future for AsyncInterface<T> {
            type Output = SafeWrapped<T>;
            fn poll(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Self::Output> {
                match self.as_mut().get_mut() {
                    AsyncInterface::Lazy(lazy) => {
                        Pin::new(lazy).poll(cx).map(|x| SafeWrapped(x))
                    }
                    AsyncInterface::Threaded(threaded) => Pin::new(threaded).poll(cx),
                    AsyncInterface::LazyWithWaker(lazy) => {
                        Pin::new(lazy).poll(cx).map(|x| SafeWrapped(x))
                    }
                }
            }
        }
        #[repr(C)]
        pub struct ThreadedTask<T: FFISafeContainer + 'static> {
            pub computation: extern "C" fn() -> T,
            handle: Option<JoinHandle<SafeWrapped<T>>>,
        }
        impl<T: FFISafeContainer + 'static> ThreadedTask<T> {
            pub fn new(task: extern "C" fn() -> T) -> Self {
                Self {
                    computation: task,
                    handle: None,
                }
            }
            fn start_once(&mut self) {
                if self.handle.is_none() {
                    let computation = UnsafeWrapped(self.computation);
                    let handle = spawn_blocking(move || {
                        let computation_fn_ptr = computation.0;
                        SafeWrapped((computation_fn_ptr)())
                    });
                    self.handle = Some(handle);
                }
            }
        }
        impl<T: FFISafeContainer + 'static> Future for ThreadedTask<T> {
            type Output = SafeWrapped<T>;
            fn poll(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Self::Output> {
                self.start_once();
                let handle = Pin::new(self.as_mut().get_mut().handle.as_mut().unwrap());
                handle
                    .poll(cx)
                    .map(|result| {
                        result.expect("Threaded task panicked during execution")
                    })
            }
        }
        pub(crate) struct UnsafeWrapped<T: 'static>(T);
        unsafe impl<T> Send for UnsafeWrapped<T> {}
        pub struct SafeWrapped<T: 'static + FFISafeContainer>(T);
        unsafe impl<T: FFISafeContainer> Send for SafeWrapped<T> {}
        impl<T: Debug + FFISafeContainer + 'static> Debug for SafeWrapped<T> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
        impl<T: 'static + FFISafeContainer> SafeWrapped<T> {
            pub fn get_pure(self) -> T {
                self.0
            }
        }
        impl<T: 'static + FFISafeContainer> Deref for SafeWrapped<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl<T: 'static + FFISafeContainer> DerefMut for SafeWrapped<T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        #[repr(C)]
        pub struct LazyableTask<T: FFISafeContainer> {
            pub state: FFIableObject,
            pub poll: extern "C" fn(state: *mut FFIableObject) -> PollResult<T>,
        }
        #[repr(C)]
        pub enum PollResult<T: FFISafeContainer> {
            Ready(T),
            Poll,
        }
        unsafe impl<T: FFISafeContainer> Send for LazyableTask<T> {}
        impl<T: FFISafeContainer + 'static> Future for LazyableTask<T> {
            type Output = T;
            fn poll(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Self::Output> {
                let data = (self.poll)(&mut self.state);
                match data {
                    PollResult::Ready(r) => Poll::Ready(r),
                    PollResult::Poll => {
                        let waker = cx.waker().clone();
                        tokio::spawn(async move {
                            tokio::time::sleep(tokio::time::Duration::from_millis(20))
                                .await;
                            waker.wake();
                        });
                        Poll::Pending
                    }
                }
            }
        }
        #[repr(C)]
        pub struct LazyableTaskWithWaker<T: FFISafeContainer> {
            pub state: FFIableObject,
            pub waker: Option<*mut c_void>,
            pub poll: extern "C" fn(state: *mut FFIableObject) -> PollResult<T>,
            pub append_waker: extern "C" fn(waker: *mut c_void) -> (),
        }
        unsafe impl<T: FFISafeContainer> Send for LazyableTaskWithWaker<T> {}
        extern "C" fn call_waker_inner(waker: FFIableObject) {
            use std::task::Waker;
            let waker: Waker = unsafe { waker.reconstruct() };
            waker.wake();
        }
        impl<T: FFISafeContainer + 'static> Future for LazyableTaskWithWaker<T> {
            type Output = T;
            fn poll(
                mut self: Pin<&mut Self>,
                cx: &mut Context<'_>,
            ) -> Poll<Self::Output> {
                let data = (self.poll)(&mut self.state);
                match data {
                    PollResult::Ready(r) => Poll::Ready(r),
                    PollResult::Poll => {
                        if self.waker.is_none() {
                            let waker: std::task::Waker = cx.waker().clone();
                            self.waker = Some({
                                (WAKER
                                    .create_waker)(
                                    FFIableObject::create_using_box_no_display(waker),
                                    call_waker_inner,
                                )
                            });
                            (self.append_waker)(self.waker.clone().unwrap())
                        }
                        Poll::Pending
                    }
                }
            }
        }
    }
    pub mod others {
        use core::str;
        use std::fmt::{self, Display};
        use std::ops::{Deref, Index, Range};
        use std::os::raw::c_void;
        use std::ptr::{copy_nonoverlapping, null_mut};
        use std::slice;
        use libc::{malloc, realloc};
        use crate::Maybe;
        pub mod boxes {
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
                    let ptr = unsafe {
                        malloc(size) as *mut (T, extern "C" fn(_: *mut c_void))
                    };
                    let ptr = match NonNull::new(ptr) {
                        Some(p) => p,
                        None => {
                            ::core::panicking::panic_fmt(
                                format_args!("Failed to allocate memory"),
                            );
                        }
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
                    let value = unsafe { ptr::read(ptr) };
                    if mem::size_of::<T>() > 0 {
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
                        ptr: NonNull::new(ptr as *mut (T, extern "C" fn(_: *mut c_void)))
                            .expect("Invalid Pointer provided"),
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
                        unsafe {
                            let ptr = &mut *self.ptr.as_ptr();
                            (ptr.1)(&mut ptr.0 as *mut _ as *mut _)
                        }
                        unsafe {
                            free(self.ptr.as_ptr() as *mut _);
                        }
                    }
                }
            }
        }
        pub mod hashmap {
            use crate::{
                Maybe, Ref, common::{FFIableObject, others::FFISafeString},
                create,
            };
            use libloading::{Library, Symbol, library_filename};
            use std::{env, mem::transmute, path::PathBuf, str::FromStr, sync::LazyLock};
            pub static FFIHASHMAP: LazyLock<FfiHashMap> = LazyLock::new(|| {
                FfiHashMap::new()
            });
            pub type CallCreateFn = unsafe extern "C" fn(use_str_map: bool) -> Ref;
            pub type CallInsertIntFn = unsafe extern "C" fn(
                map: *const Ref,
                k: usize,
                v: FFIableObject,
            ) -> Maybe<FFIableObject>;
            pub type CallGetIntFn = unsafe extern "C" fn(
                map: *const Ref,
                k: *const usize,
            ) -> Maybe<*const FFIableObject>;
            pub type CallRemoveIntFn = unsafe extern "C" fn(
                map: *const Ref,
                k: *const usize,
            ) -> Maybe<FFIableObject>;
            pub type CallInsertStrFn = unsafe extern "C" fn(
                map: *const Ref,
                k: FFISafeString,
                v: FFIableObject,
            ) -> Maybe<FFIableObject>;
            pub type CallGetStrFn = unsafe extern "C" fn(
                map: *const Ref,
                k: *const FFISafeString,
            ) -> Maybe<*const FFIableObject>;
            pub type CallRemoveStrFn = unsafe extern "C" fn(
                map: *const Ref,
                k: *const FFISafeString,
            ) -> Maybe<FFIableObject>;
            pub struct FfiHashMap {
                _lib: Library,
                ///Creates a new HashMap
                pub create: CallCreateFn,
                ///Inserts a key-value pair using an integer key
                pub insert_int: CallInsertIntFn,
                ///Gets a reference to the value associated with an integer key
                pub get_int: CallGetIntFn,
                ///Removes and returns the value associated with an integer key
                pub remove_int: CallRemoveIntFn,
                ///Inserts a key-value pair using a string key
                pub insert_str: CallInsertStrFn,
                ///Gets a reference to the value associated with a string key
                pub get_str: CallGetStrFn,
                ///Removes and returns the value associated with a string key
                pub remove_str: CallRemoveStrFn,
            }
            impl FfiHashMap {
                pub fn new() -> Self {
                    let lrt = env::var("LRT_HOME").expect("LRT Home not present");
                    let file = library_filename("hashmap");
                    let mut path = PathBuf::from_str(&lrt).unwrap();
                    path.push("libs");
                    path.push("waker");
                    path.push(file);
                    let lib = unsafe {
                        Library::new(path).expect("Unable to load async_waker")
                    };
                    let create = {
                        let ptr: Symbol<'_, CallCreateFn> = unsafe {
                            lib.get("create".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallCreateFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    let insert_int = {
                        let ptr: Symbol<'_, CallInsertIntFn> = unsafe {
                            lib.get("insert_int".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallInsertIntFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    let get_int = {
                        let ptr: Symbol<'_, CallGetIntFn> = unsafe {
                            lib.get("get_int".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallGetIntFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    let remove_int = {
                        let ptr: Symbol<'_, CallRemoveIntFn> = unsafe {
                            lib.get("remove_int".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallRemoveIntFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    let insert_str = {
                        let ptr: Symbol<'_, CallInsertStrFn> = unsafe {
                            lib.get("insert_str".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallInsertStrFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    let get_str = {
                        let ptr: Symbol<'_, CallGetStrFn> = unsafe {
                            lib.get("get_str".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallGetStrFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    let remove_str = {
                        let ptr: Symbol<'_, CallRemoveStrFn> = unsafe {
                            lib.get("remove_str".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallRemoveStrFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    Self {
                        _lib: lib,
                        create: *create,
                        insert_int: *insert_int,
                        get_int: *get_int,
                        remove_int: *remove_int,
                        insert_str: *insert_str,
                        get_str: *get_str,
                        remove_str: *remove_str,
                    }
                }
            }
        }
        pub mod vector {
            use crate::{
                common::{others::boxes::Boxed, FFIableObject},
                Maybe,
            };
            use libc::malloc;
            use std::{
                ffi::c_void, ops::{Index, Range},
                ptr, slice,
            };
            #[repr(C)]
            pub struct FFIVector {
                ptr: *mut *mut c_void,
                len: usize,
                cap: usize,
            }
            unsafe impl Send for FFIVector {}
            unsafe impl Sync for FFIVector {}
            impl FFIVector {
                pub fn null() -> Self {
                    Self {
                        ptr: ptr::null_mut(),
                        cap: 0,
                        len: 0,
                    }
                }
                pub fn new(slice: Vec<FFIableObject>) -> Maybe<Self> {
                    let len = slice.len();
                    if len == 0 {
                        return Maybe::Some(Self::null());
                    }
                    let Some(cap) = len.checked_mul(2) else {
                        return Maybe::None;
                    };
                    let ptr = unsafe {
                        malloc(cap * size_of::<*mut c_void>()) as *mut *mut c_void
                    };
                    if ptr.is_null() {
                        return Maybe::None;
                    }
                    unsafe {
                        for (index, item) in slice.into_iter().enumerate() {
                            let data = Boxed::into_raw(Boxed::new(item));
                            *ptr.add(index) = data;
                        }
                    }
                    Maybe::Some(Self { ptr, len, cap })
                }
                pub fn push(&mut self, item: FFIableObject) -> Maybe<()> {
                    let Some(new_len) = self.len.checked_add(1) else {
                        return Maybe::None;
                    };
                    let mut cap = self.cap;
                    if new_len > self.cap {
                        unsafe {
                            let Some(c) = new_len.checked_mul(2) else {
                                return Maybe::None;
                            };
                            cap = c;
                            let new_ptr = libc::realloc(
                                self.ptr as _,
                                cap * size_of::<*mut c_void>(),
                            ) as *mut *mut c_void;
                            if new_ptr.is_null() {
                                return Maybe::None;
                            }
                            self.ptr = new_ptr as *mut *mut c_void;
                        }
                    }
                    unsafe {
                        *self.ptr.add(self.len) = Boxed::into_raw(
                            Boxed::<FFIableObject>::new(item),
                        );
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
                pub unsafe fn get_at<'a>(
                    &'a self,
                    index: Range<usize>,
                ) -> &'a [*mut c_void] {
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
                        *self.ptr.add(index) = data;
                        drop(Boxed::<FFIableObject>::from_raw(old_ptr));
                        Maybe::Some(())
                    }
                }
            }
            impl Index<Range<usize>> for FFIVector {
                type Output = [*mut c_void];
                fn index(&self, index: Range<usize>) -> &Self::Output {
                    unsafe { self.get_at(index) }
                }
            }
            impl Drop for FFIVector {
                fn drop(&mut self) {
                    if !self.ptr.is_null() {
                        unsafe { self.get_at(0..self.len) }
                            .iter()
                            .for_each(|x| {
                                unsafe { drop(Boxed::<FFIableObject>::from_raw(*x)) };
                            });
                        unsafe { libc::free(self.ptr as *mut c_void) }
                    }
                }
            }
        }
        #[macro_use]
        pub(crate) mod macros {}
        /// A string using our custom spec for FFI.
        ///
        /// This struct manages a custom made String
        /// It can store UTF-8 compatible Strings and is quite stable in design
        ///
        /// Intended for Rust-Rust FFI
        #[repr(C)]
        /// TODO: Rename to FFISafeString
        pub struct FFISafeString {
            ptr: *mut u8,
            len: usize,
            cap: usize,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for FFISafeString {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "FFISafeString",
                    "ptr",
                    &self.ptr,
                    "len",
                    &self.len,
                    "cap",
                    &&self.cap,
                )
            }
        }
        impl FFISafeString {
            pub fn null() -> Self {
                Self {
                    ptr: null_mut(),
                    cap: 0,
                    len: 0,
                }
            }
            pub fn new(slice: impl AsRef<str>) -> Maybe<Self> {
                let data: &str = slice.as_ref();
                let len = data.len();
                if len == 0 {
                    return Maybe::Some(Self::null());
                }
                let Some(cap) = len.checked_mul(2) else {
                    return Maybe::None;
                };
                let ptr = unsafe { malloc(cap) as *mut u8 };
                if ptr.is_null() {
                    return Maybe::None;
                }
                unsafe {
                    let bytes = data.as_bytes();
                    let src = bytes.as_ptr();
                    copy_nonoverlapping(src, ptr, bytes.len());
                }
                Maybe::Some(Self { ptr, len, cap })
            }
            pub fn push_str(&mut self, slice: impl AsRef<str>) -> Maybe<()> {
                let data: &str = slice.as_ref();
                if data.len() == 0 {
                    return Maybe::Some(());
                }
                let Some(new_len) = self.len.checked_add(data.len()) else {
                    return Maybe::None;
                };
                let mut cap = self.cap;
                if new_len > self.cap {
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
                let slice = unsafe {
                    slice::from_raw_parts(dst.add(index.start), index.len())
                };
                unsafe { str::from_utf8_unchecked(slice) }
            }
            /// This does not realloc and returns Maybe::None, if the total size is less than cap or if its invalid utf8
            pub fn get_at(&self, index: Range<usize>) -> Maybe<&str> {
                let dst = self.ptr;
                let slice = unsafe {
                    slice::from_raw_parts(dst.add(index.start), index.len())
                };
                str::from_utf8(slice).ok().into()
            }
            pub fn edit(&mut self, start: usize, src: impl AsRef<str>) -> Maybe<()> {
                unsafe {
                    let src = src.as_ref().as_bytes();
                    let to_be_set_as_len = (start + src.len()).max(self.len);
                    if self.cap < to_be_set_as_len {
                        let Some(new_cap) = to_be_set_as_len.checked_mul(2) else {
                            return Maybe::None;
                        };
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
                    copy_nonoverlapping(
                        src.as_ptr(),
                        self.ptr.add(start) as _,
                        src.len(),
                    );
                }
                Maybe::Some(())
            }
            pub fn to_str<'a>(&'a self) -> Maybe<&'a str> {
                if self.ptr.is_null() {
                    return Maybe::Some("");
                }
                let bytes = unsafe { slice::from_raw_parts(self.ptr, self.len) };
                str::from_utf8(bytes).ok().into()
            }
            pub fn try_clone(&self) -> Maybe<Self> {
                Self::new(
                    match self.to_str() {
                        Maybe::Some(x) => x,
                        _ => {
                            return Maybe::None;
                        }
                    },
                )
            }
            pub unsafe fn to_str_unchecked<'a>(&'a self) -> &'a str {
                if self.ptr.is_null() {
                    return "";
                }
                let bytes = unsafe { slice::from_raw_parts(self.ptr, self.len) };
                unsafe { str::from_utf8_unchecked(bytes) }
            }
        }
        impl Index<usize> for FFISafeString {
            type Output = str;
            fn index(&self, index: usize) -> &Self::Output {
                self.get_at(Range {
                        start: index,
                        end: index + 1,
                    })
                    .unwrap()
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
    }
    #[repr(C)]
    pub struct FFIableObject {
        data: *mut c_void,
        drop: extern "C" fn(*mut c_void),
        fmt: extern "C" fn(*mut c_void) -> FFISafeString,
        display: extern "C" fn(*mut c_void) -> FFISafeString,
        poisoned: bool,
        tag: u8,
    }
    impl FFISafeContainer for FFIableObject {}
    extern "C" fn general_drop<T>(ptr: *mut c_void) {
        unsafe {
            drop(Boxed::<T>::from_raw(ptr));
        }
    }
    extern "C" fn general_display<T: Display>(ptr: *mut c_void) -> FFISafeString {
        unsafe {
            let data = &*(ptr as *mut (T, extern "C" fn(_: *mut c_void)));
            let data = &data.0;
            let fmt = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", data))
            });
            FFISafeString::new(fmt).unwrap()
        }
    }
    extern "C" fn general_debug<T: Debug>(ptr: *mut c_void) -> FFISafeString {
        unsafe {
            let data = &*(ptr as *mut (T, extern "C" fn(_: *mut c_void)));
            let data = &data.0;
            let fmt = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0:?}", data))
            });
            FFISafeString::new(fmt).unwrap()
        }
    }
    extern "C" fn no_display(_: *mut c_void) -> FFISafeString {
        FFISafeString::new(
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("<cannot display type>"))
                }),
            )
            .unwrap()
    }
    extern "C" fn drop_noop(_: *mut c_void) {}
    extern "C" fn fmt_noop(_: *mut c_void) -> FFISafeString {
        FFISafeString::new("").unwrap()
    }
    impl FFIableObject {
        /// This might be a good way to create a dummy, invalid struct
        /// We still dont recommend it
        pub fn null() -> Self {
            Self {
                data: null_mut(),
                display: no_display,
                drop: drop_noop,
                fmt: fmt_noop,
                poisoned: true,
                tag: 0,
            }
        }
        pub const fn is_null(&self) -> bool {
            self.data.is_null()
        }
        /// (Un)safely consumes the FFIableObject and returns the original owned `T`.
        ///
        /// This method transfers ownership of the raw data pointer from this FFIableObject
        /// to the returned `T`. It sets this FFIableObject's `poisoned` flag to `true`
        /// to prevent its `drop` implementation from freeing the memory it no longer owns.
        ///
        /// # Panics
        /// Panics if this FFIableObject is poisoned.
        ///
        /// # Safety
        ///
        /// This function is unsafe because the caller must ensure that:
        /// 1. This `FFIableObject` instance currently owns the data (i.e., `self.is_poisoned()` is `false`).
        ///    Calling this on a poisoned object will lead to a panic.
        /// 2. The `FFIableObject` actually contains a value of type `T`. Mis-casting `T` will lead to Undefined Behavior.
        /// 3. This `FFIableObject` is not used further after this call, as its internal pointer
        ///    will effectively be consumed.
        /// 4. This binary has created this object or else it can happily lead for undefined behaviour
        pub unsafe fn reconstruct<T: Debug>(mut self) -> T {
            if self.poisoned {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("FFIableObject is poisoned"),
                    );
                };
            }
            self.poisoned = true;
            (unsafe { Boxed::<T>::from_raw(self.data) }).unbox()
        }
        /// Transfers the ownership to the new data and sets the `poisoned` field to `true` of this structure
        pub unsafe fn transfer_ownership(&mut self) -> FFIableObject {
            if self.poisoned {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("The object is already poisoned."),
                    );
                };
            }
            let data = self.data;
            self.poisoned = true;
            FFIableObject {
                data,
                drop: self.drop,
                fmt: self.fmt,
                display: self.display,
                poisoned: false,
                tag: self.tag,
            }
        }
        /// Returns whether this FFIableObject is poisoned or not. This is usually used to check whether
        /// `reconstruct` or `transfer_ownership` has been called on this instance before calling any other methods.
        pub const fn is_poisoned(&self) -> bool {
            self.poisoned
        }
        /// Get a mutable reference to the inner `FFIableObject`
        ///
        /// # Safety
        /// We assume that you're absolutely sure that the strucure is the `T` that you've provided
        pub unsafe fn get_mut<'a, T>(&'a mut self) -> &'a mut T {
            if self.poisoned {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("The object is poisoned."),
                    );
                };
            }
            unsafe { self.get_mut_unchecked() }
        }
        /// Get a mutable reference to the inner `FFIableObject` like `get_mut`
        ///
        /// # 🚨 Safety
        /// ## **CRITICAL CAUTION REQUIRED**
        /// - We assume that you're absolutely sure that the strucure is the `T` that you've provided
        /// - **CRITICAL** This function does not check if the data is poisoned!
        pub unsafe fn get_mut_unchecked<'a, T>(&'a mut self) -> &'a mut T {
            unsafe { &mut (*(self.data as *mut (T, extern "C" fn(_: *mut c_void)))).0 }
        }
        /// Get a mutable reference to the inner `FFIableObject`
        ///
        /// # Safety
        /// We assume that you're absolutely sure that the strucure is the `T` that you've provided
        pub unsafe fn get<'a, T>(&'a self) -> &'a T {
            if self.poisoned {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("The object is poisoned."),
                    );
                };
            }
            unsafe { self.get_unchecked() }
        }
        /// Get a mutable reference to the inner `FFIableObject` like `get`
        ///
        /// # 🚨 Safety
        /// ## **CRITICAL CAUTION REQUIRED**
        /// - We assume that you're absolutely sure that the strucure is the `T` that you've provided
        /// - **CRITICAL** This function does not check if the data is poisoned!
        pub unsafe fn get_unchecked<'a, T>(&'a self) -> &'a T {
            unsafe { &(*(self.data as *mut (T, extern "C" fn(_: *mut c_void)))).0 }
        }
        pub fn create_using_box<T: Debug + Display + 'static>(data: T) -> Self {
            let data = Boxed::new(data);
            let data = Boxed::into_raw(data);
            Self {
                data: data as *mut c_void,
                display: general_display::<T>,
                drop: general_drop::<T>,
                fmt: general_debug::<T>,
                poisoned: false,
                tag: get_tag::<T>(),
            }
        }
        pub fn create_using_box_no_display<T: Debug + 'static>(data: T) -> Self {
            let data = Boxed::new(data);
            let data = Boxed::into_raw(data);
            Self {
                data: data as *mut c_void,
                display: no_display,
                drop: general_drop::<T>,
                fmt: general_debug::<T>,
                poisoned: false,
                tag: get_tag::<T>(),
            }
        }
        pub fn create_using_box_non_static<T: Debug + Display>(data: T) -> Self {
            let data = Boxed::new(data);
            let data = Boxed::into_raw(data);
            Self {
                data: data as *mut c_void,
                display: general_display::<T>,
                drop: general_drop::<T>,
                fmt: general_debug::<T>,
                poisoned: false,
                tag: 0,
            }
        }
        pub fn create_using_box_no_display_non_static<T: Debug>(data: T) -> Self {
            let data = Boxed::new(data);
            let data = Boxed::into_raw(data);
            Self {
                data: data as *mut c_void,
                display: no_display,
                drop: general_drop::<T>,
                fmt: general_debug::<T>,
                poisoned: false,
                tag: 0,
            }
        }
    }
    impl Into<FFIableObject> for u8 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_u8(&self) -> Option<&u8> {
            if self.tag != 1 {
                return None;
            }
            unsafe { Some(self.as_u8_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_u8_mut<'a>(&'a mut self) -> Option<&'a mut u8> {
            if self.tag != 1 {
                return None;
            }
            unsafe { Some(self.as_u8_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_u8_unchecked<'a>(&'a self) -> &'a u8 {
            unsafe { self.get::<u8>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_u8_mut_unchecked<'a>(&'a mut self) -> &'a mut u8 {
            unsafe { self.get_mut::<u8>() }
        }
    }
    impl Into<FFIableObject> for u16 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_u16(&self) -> Option<&u16> {
            if self.tag != 2 {
                return None;
            }
            unsafe { Some(self.as_u16_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_u16_mut<'a>(&'a mut self) -> Option<&'a mut u16> {
            if self.tag != 2 {
                return None;
            }
            unsafe { Some(self.as_u16_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_u16_unchecked<'a>(&'a self) -> &'a u16 {
            unsafe { self.get::<u16>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_u16_mut_unchecked<'a>(&'a mut self) -> &'a mut u16 {
            unsafe { self.get_mut::<u16>() }
        }
    }
    impl Into<FFIableObject> for u32 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_u32(&self) -> Option<&u32> {
            if self.tag != 3 {
                return None;
            }
            unsafe { Some(self.as_u32_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_u32_mut<'a>(&'a mut self) -> Option<&'a mut u32> {
            if self.tag != 3 {
                return None;
            }
            unsafe { Some(self.as_u32_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_u32_unchecked<'a>(&'a self) -> &'a u32 {
            unsafe { self.get::<u32>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_u32_mut_unchecked<'a>(&'a mut self) -> &'a mut u32 {
            unsafe { self.get_mut::<u32>() }
        }
    }
    impl Into<FFIableObject> for u64 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_u64(&self) -> Option<&u64> {
            if self.tag != 4 {
                return None;
            }
            unsafe { Some(self.as_u64_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_u64_mut<'a>(&'a mut self) -> Option<&'a mut u64> {
            if self.tag != 4 {
                return None;
            }
            unsafe { Some(self.as_u64_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_u64_unchecked<'a>(&'a self) -> &'a u64 {
            unsafe { self.get::<u64>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_u64_mut_unchecked<'a>(&'a mut self) -> &'a mut u64 {
            unsafe { self.get_mut::<u64>() }
        }
    }
    impl Into<FFIableObject> for u128 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_u128(&self) -> Option<&u128> {
            if self.tag != 5 {
                return None;
            }
            unsafe { Some(self.as_u128_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_u128_mut<'a>(&'a mut self) -> Option<&'a mut u128> {
            if self.tag != 5 {
                return None;
            }
            unsafe { Some(self.as_u128_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_u128_unchecked<'a>(&'a self) -> &'a u128 {
            unsafe { self.get::<u128>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_u128_mut_unchecked<'a>(&'a mut self) -> &'a mut u128 {
            unsafe { self.get_mut::<u128>() }
        }
    }
    impl Into<FFIableObject> for i8 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_i8(&self) -> Option<&i8> {
            if self.tag != 6 {
                return None;
            }
            unsafe { Some(self.as_i8_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_i8_mut<'a>(&'a mut self) -> Option<&'a mut i8> {
            if self.tag != 6 {
                return None;
            }
            unsafe { Some(self.as_i8_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_i8_unchecked<'a>(&'a self) -> &'a i8 {
            unsafe { self.get::<i8>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_i8_mut_unchecked<'a>(&'a mut self) -> &'a mut i8 {
            unsafe { self.get_mut::<i8>() }
        }
    }
    impl Into<FFIableObject> for i16 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_i16(&self) -> Option<&i16> {
            if self.tag != 7 {
                return None;
            }
            unsafe { Some(self.as_i16_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_i16_mut<'a>(&'a mut self) -> Option<&'a mut i16> {
            if self.tag != 7 {
                return None;
            }
            unsafe { Some(self.as_i16_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_i16_unchecked<'a>(&'a self) -> &'a i16 {
            unsafe { self.get::<i16>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_i16_mut_unchecked<'a>(&'a mut self) -> &'a mut i16 {
            unsafe { self.get_mut::<i16>() }
        }
    }
    impl Into<FFIableObject> for i32 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_i32(&self) -> Option<&i32> {
            if self.tag != 8 {
                return None;
            }
            unsafe { Some(self.as_i32_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_i32_mut<'a>(&'a mut self) -> Option<&'a mut i32> {
            if self.tag != 8 {
                return None;
            }
            unsafe { Some(self.as_i32_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_i32_unchecked<'a>(&'a self) -> &'a i32 {
            unsafe { self.get::<i32>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_i32_mut_unchecked<'a>(&'a mut self) -> &'a mut i32 {
            unsafe { self.get_mut::<i32>() }
        }
    }
    impl Into<FFIableObject> for i64 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_i64(&self) -> Option<&i64> {
            if self.tag != 9 {
                return None;
            }
            unsafe { Some(self.as_i64_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_i64_mut<'a>(&'a mut self) -> Option<&'a mut i64> {
            if self.tag != 9 {
                return None;
            }
            unsafe { Some(self.as_i64_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_i64_unchecked<'a>(&'a self) -> &'a i64 {
            unsafe { self.get::<i64>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_i64_mut_unchecked<'a>(&'a mut self) -> &'a mut i64 {
            unsafe { self.get_mut::<i64>() }
        }
    }
    impl Into<FFIableObject> for i128 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_i128(&self) -> Option<&i128> {
            if self.tag != 10 {
                return None;
            }
            unsafe { Some(self.as_i128_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_i128_mut<'a>(&'a mut self) -> Option<&'a mut i128> {
            if self.tag != 10 {
                return None;
            }
            unsafe { Some(self.as_i128_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_i128_unchecked<'a>(&'a self) -> &'a i128 {
            unsafe { self.get::<i128>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_i128_mut_unchecked<'a>(&'a mut self) -> &'a mut i128 {
            unsafe { self.get_mut::<i128>() }
        }
    }
    impl Into<FFIableObject> for f32 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_f32(&self) -> Option<&f32> {
            if self.tag != 11 {
                return None;
            }
            unsafe { Some(self.as_f32_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_f32_mut<'a>(&'a mut self) -> Option<&'a mut f32> {
            if self.tag != 11 {
                return None;
            }
            unsafe { Some(self.as_f32_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_f32_unchecked<'a>(&'a self) -> &'a f32 {
            unsafe { self.get::<f32>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_f32_mut_unchecked<'a>(&'a mut self) -> &'a mut f32 {
            unsafe { self.get_mut::<f32>() }
        }
    }
    impl Into<FFIableObject> for f64 {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_f64(&self) -> Option<&f64> {
            if self.tag != 12 {
                return None;
            }
            unsafe { Some(self.as_f64_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_f64_mut<'a>(&'a mut self) -> Option<&'a mut f64> {
            if self.tag != 12 {
                return None;
            }
            unsafe { Some(self.as_f64_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_f64_unchecked<'a>(&'a self) -> &'a f64 {
            unsafe { self.get::<f64>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_f64_mut_unchecked<'a>(&'a mut self) -> &'a mut f64 {
            unsafe { self.get_mut::<f64>() }
        }
    }
    impl Into<FFIableObject> for bool {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_bool(&self) -> Option<&bool> {
            if self.tag != 13 {
                return None;
            }
            unsafe { Some(self.as_bool_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_bool_mut<'a>(&'a mut self) -> Option<&'a mut bool> {
            if self.tag != 13 {
                return None;
            }
            unsafe { Some(self.as_bool_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_bool_unchecked<'a>(&'a self) -> &'a bool {
            unsafe { self.get::<bool>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_bool_mut_unchecked<'a>(&'a mut self) -> &'a mut bool {
            unsafe { self.get_mut::<bool>() }
        }
    }
    impl Into<FFIableObject> for FFISafeString {
        fn into(self) -> FFIableObject {
            FFIableObject::create_using_box(self)
        }
    }
    impl FFIableObject {
        /// Returns `None` is types do not match
        pub fn as_ffisafestring(&self) -> Option<&FFISafeString> {
            if self.tag != 14 {
                return None;
            }
            unsafe { Some(self.as_ffisafestring_unchecked()) }
        }
        /// Returns `None` is types do not match
        pub fn as_ffisafestring_mut<'a>(&'a mut self) -> Option<&'a mut FFISafeString> {
            if self.tag != 14 {
                return None;
            }
            unsafe { Some(self.as_ffisafestring_mut_unchecked()) }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_ffisafestring_unchecked<'a>(&'a self) -> &'a FFISafeString {
            unsafe { self.get::<FFISafeString>() }
        }
        /// In NO Case; Should this be used unless you're absolutely sure it is exactly the type you're casting it as
        pub unsafe fn as_ffisafestring_mut_unchecked<'a>(
            &'a mut self,
        ) -> &'a mut FFISafeString {
            unsafe { self.get_mut::<FFISafeString>() }
        }
    }
    fn get_tag<T: 'static>() -> u8 {
        let ty = TypeId::of::<T>();
        if ty == TypeId::of::<u8>() {
            return 1;
        }
        if ty == TypeId::of::<u16>() {
            return 2;
        }
        if ty == TypeId::of::<u32>() {
            return 3;
        }
        if ty == TypeId::of::<u64>() {
            return 4;
        }
        if ty == TypeId::of::<u128>() {
            return 5;
        }
        if ty == TypeId::of::<i8>() {
            return 6;
        }
        if ty == TypeId::of::<i16>() {
            return 7;
        }
        if ty == TypeId::of::<i32>() {
            return 8;
        }
        if ty == TypeId::of::<i64>() {
            return 9;
        }
        if ty == TypeId::of::<i128>() {
            return 10;
        }
        if ty == TypeId::of::<f32>() {
            return 11;
        }
        if ty == TypeId::of::<f64>() {
            return 12;
        }
        if ty == TypeId::of::<bool>() {
            return 13;
        }
        if ty == TypeId::of::<FFISafeString>() {
            return 14;
        }
        0
    }
    impl Display for FFIableObject {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let data = (self.display)(self.data);
            let data = data.to_str();
            let Maybe::Some(data) = data else {
                return Err(std::fmt::Error::default());
            };
            f.write_str(data)
        }
    }
    impl Debug for FFIableObject {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let data = (self.fmt)(self.data);
            let data = data.to_str();
            let Maybe::Some(data) = data else {
                return Err(std::fmt::Error::default());
            };
            f.write_str(data)
        }
    }
    impl Drop for FFIableObject {
        fn drop(&mut self) {
            if !self.poisoned {
                (self.drop)(self.data)
            }
        }
    }
}
#[repr(C)]
pub struct Ref {
    ptr: *mut c_void,
    drop: extern "C" fn(ptr: *mut c_void),
}
impl Drop for Ref {
    fn drop(&mut self) {
        (self.drop)(self.ptr)
    }
}
#[repr(C)]
pub enum Maybe<T> {
    Some(T),
    None,
}
impl<T> Maybe<T> {
    pub fn unwrap(self) -> T {
        match self {
            Maybe::Some(x) => x,
            _ => {
                ::core::panicking::panic_fmt(format_args!("Panicked during unwrap"));
            }
        }
    }
}
impl<T> From<Option<&T>> for Maybe<*const T> {
    fn from(value: Option<&T>) -> Self {
        match value {
            Some(x) => Maybe::Some(x),
            None => Maybe::None,
        }
    }
}
impl<T> From<Option<&mut T>> for Maybe<*mut T> {
    fn from(value: Option<&mut T>) -> Self {
        match value {
            Some(x) => Maybe::Some(x),
            None => Maybe::None,
        }
    }
}
impl<T> From<Option<T>> for Maybe<T> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(x) => Maybe::Some(x),
            None => Maybe::None,
        }
    }
}
