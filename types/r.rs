#![feature(prelude_import)]
#![feature(trivial_bounds)]
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
            use std::fmt::Debug;
            use crate::common::FFIableObject;
            #[repr(C)]
            pub struct FnStackV0 {
                /// Return value (identifier in MemoryMap)
                pub ret: Option<FFIableObject>,
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
            #[automatically_derived]
            impl ::core::default::Default for FnStackV0 {
                #[inline]
                fn default() -> FnStackV0 {
                    FnStackV0 {
                        ret: ::core::default::Default::default(),
                        r1: ::core::default::Default::default(),
                        r2: ::core::default::Default::default(),
                        r3: ::core::default::Default::default(),
                        r4: ::core::default::Default::default(),
                        r5: ::core::default::Default::default(),
                        r6: ::core::default::Default::default(),
                        r7: ::core::default::Default::default(),
                        r8: ::core::default::Default::default(),
                    }
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
                _inner: RegValueV0,
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
        marker::PhantomData,
    };
    use crate::{
        commands::FFISafeContainer, common::others::{FFISafeString, boxes::Boxed},
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
        use std::ffi::CString;
        use std::fmt;
        use std::os::raw::c_char;
        use std::ptr;
        use std::slice;
        pub mod boxes {
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
                pub fn new(value: T) -> Self {
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
                        None => {
                            ::core::panicking::panic_fmt(
                                format_args!("Failed to allocate memory"),
                            );
                        }
                    };
                    unsafe {
                        ptr::write(ptr.as_ptr(), value);
                    }
                    Self { ptr, drop: true }
                }
                /// Consumes the FfiBox and returns the value inside.
                ///
                /// This will deallocate the memory used by the box but not the value itself.
                pub fn unbox(mut self) -> T {
                    let ptr = self.ptr.as_ptr();
                    self.drop = false;
                    let value = unsafe { ptr::read(ptr) };
                    if mem::size_of::<T>() > 0 {
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
                pub unsafe fn from_raw(ptr: *mut T) -> Self {
                    Self {
                        ptr: NonNull::new(ptr as *mut T)
                            .expect("Invalid Pointer provided"),
                        drop: true,
                    }
                }
                pub fn into_raw(mut val: Self) -> *mut c_void {
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
                        unsafe {
                            ptr::drop_in_place(self.ptr.as_ptr() as *mut c_void);
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
            use crate::{Maybe, Ref, common::FFIableObject, create};
            use libloading::{Library, Symbol, library_filename};
            use std::{env, mem::transmute, path::PathBuf, str::FromStr, sync::LazyLock};
            pub static FFIVECTOR: LazyLock<FfiVector> = LazyLock::new(|| {
                FfiVector::new()
            });
            pub type CallCreateFn = unsafe extern "C" fn() -> Ref;
            pub type CallGetAtFn = unsafe extern "C" fn(
                this: *mut Ref,
                index: usize,
            ) -> Maybe<*const FFIableObject>;
            pub type CallGetAtMutFn = unsafe extern "C" fn(
                this: *mut Ref,
                index: usize,
            ) -> Maybe<*mut FFIableObject>;
            pub type CallReplaceFn = unsafe extern "C" fn(
                this: *mut Ref,
                index: usize,
                with: FFIableObject,
            ) -> Maybe<FFIableObject>;
            pub type CallPopFn = unsafe extern "C" fn(
                this: *mut Ref,
            ) -> Maybe<FFIableObject>;
            pub type CallPushFn = unsafe extern "C" fn(
                this: *mut Ref,
                item: FFIableObject,
            ) -> ();
            pub type CallLengthFn = unsafe extern "C" fn(this: *mut Ref) -> usize;
            pub type CallCapacityFn = unsafe extern "C" fn(this: *mut Ref) -> usize;
            pub struct FfiVector {
                _lib: Library,
                ///Creates a new empty vector
                pub create: CallCreateFn,
                ///Gets a reference to the element at the specified index
                pub get_at: CallGetAtFn,
                ///Gets a mutable reference to the element at the specified index
                pub get_at_mut: CallGetAtMutFn,
                ///Replaces the element at the specified index
                pub replace: CallReplaceFn,
                ///Removes the last element from the vector and returns it
                pub pop: CallPopFn,
                ///Appends an element to the back of the vector
                pub push: CallPushFn,
                ///Returns the number of elements in the vector
                pub length: CallLengthFn,
                ///Returns the total number of elements the vector can hold without reallocating
                pub capacity: CallCapacityFn,
            }
            impl FfiVector {
                pub fn new() -> Self {
                    let lrt = env::var("LRT_HOME").expect("LRT Home not present");
                    let file = library_filename("vector");
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
                    let get_at = {
                        let ptr: Symbol<'_, CallGetAtFn> = unsafe {
                            lib.get("get_at".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallGetAtFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    let get_at_mut = {
                        let ptr: Symbol<'_, CallGetAtMutFn> = unsafe {
                            lib.get("get_at_mut".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallGetAtMutFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    let replace = {
                        let ptr: Symbol<'_, CallReplaceFn> = unsafe {
                            lib.get("replace".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallReplaceFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    let pop = {
                        let ptr: Symbol<'_, CallPopFn> = unsafe {
                            lib.get("pop".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallPopFn> = unsafe { transmute(ptr) };
                        out
                    };
                    let push = {
                        let ptr: Symbol<'_, CallPushFn> = unsafe {
                            lib.get("push".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallPushFn> = unsafe { transmute(ptr) };
                        out
                    };
                    let length = {
                        let ptr: Symbol<'_, CallLengthFn> = unsafe {
                            lib.get("length".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallLengthFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    let capacity = {
                        let ptr: Symbol<'_, CallCapacityFn> = unsafe {
                            lib.get("capacity".as_bytes()).unwrap()
                        };
                        let out: Symbol<'static, CallCapacityFn> = unsafe {
                            transmute(ptr)
                        };
                        out
                    };
                    Self {
                        _lib: lib,
                        create: *create,
                        get_at: *get_at,
                        get_at_mut: *get_at_mut,
                        replace: *replace,
                        pop: *pop,
                        push: *push,
                        length: *length,
                        capacity: *capacity,
                    }
                }
            }
        }
        #[macro_use]
        pub(crate) mod macros {}
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
                let len = cstring.as_bytes().len();
                let capacity = len + 1;
                let ptr = cstring.into_raw();
                Self { ptr, len, capacity }
            }
            /// Appends a string slice to this `FFISafeString`.
            ///
            /// This method reallocates memory if the new content exceeds the current capacity.
            pub fn push_str(&mut self, s: &str) {
                let additional_len = s.len();
                let new_len = self.len + additional_len;
                if new_len >= self.capacity {
                    let new_capacity = (new_len + 1).max(self.capacity * 2).max(16);
                    let new_ptr = if self.ptr.is_null() {
                        unsafe { libc::malloc(new_capacity) as *mut c_char }
                    } else {
                        unsafe {
                            libc::realloc(self.ptr as *mut libc::c_void, new_capacity)
                                as *mut c_char
                        }
                    };
                    if new_ptr.is_null() {
                        {
                            ::core::panicking::panic_fmt(
                                format_args!(
                                    "Failed to reallocate memory for FFISafeString",
                                ),
                            );
                        };
                    }
                    self.ptr = new_ptr;
                    self.capacity = new_capacity;
                }
                unsafe {
                    let dest = self.ptr.add(self.len) as *mut u8;
                    let src = s.as_ptr();
                    ptr::copy_nonoverlapping(src, dest, additional_len);
                }
                self.len = new_len;
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
                    Some(s) => f.write_fmt(format_args!("{0}", s)),
                    None => {
                        f.write_fmt(
                            format_args!("{0}", "[invalid UTF-8 or null string]"),
                        )
                    }
                }
            }
        }
        impl Drop for FFISafeString {
            fn drop(&mut self) {
                if !self.ptr.is_null() {
                    unsafe {
                        libc::free(self.ptr as *mut libc::c_void);
                    }
                    self.ptr = ptr::null_mut();
                    self.len = 0;
                    self.capacity = 0;
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
    #[repr(C)]
    pub struct WrappedFFIableObject<'a, T> {
        object: *mut FFIableObject,
        r#type: PhantomData<&'a T>,
    }
    impl<'a, T> WrappedFFIableObject<'a, T> {
        pub fn create_using_box<E: Debug + Display + 'static>(
            data: E,
        ) -> (Self, FFIableObject) {
            let mut object = FFIableObject::create_using_box(data);
            let data = Self::create_from_object(&mut object);
            (data, object)
        }
        pub fn create_using_box_no_display<E: Debug + 'static>(
            data: E,
        ) -> (Self, FFIableObject) {
            let mut object = FFIableObject::create_using_box_no_display(data);
            let data = Self::create_from_object(&mut object);
            (data, object)
        }
        pub fn create_using_box_non_static<E: Debug + Display>(
            data: E,
        ) -> (Self, FFIableObject) {
            let mut object = FFIableObject::create_using_box_non_static(data);
            let data = Self::create_from_object(&mut object);
            (data, object)
        }
        pub fn create_using_box_no_display_non_static<E: Debug + 'static>(
            data: E,
        ) -> (Self, FFIableObject) {
            let mut object = FFIableObject::create_using_box_no_display_non_static(data);
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
        pub unsafe fn get_mut(&'a mut self) -> &'a mut T {
            unsafe { self.get_ptr().get_mut() }
        }
    }
    extern "C" fn general_drop<T>(ptr: *mut c_void) {
        unsafe {
            drop(Boxed::from_raw(ptr as *mut T));
        }
    }
    extern "C" fn general_display<T: Display>(ptr: *mut c_void) -> FFISafeString {
        unsafe {
            let data = &*(ptr as *mut T);
            let fmt = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0}", data))
            });
            FFISafeString::from(fmt)
        }
    }
    extern "C" fn general_debug<T: Debug>(ptr: *mut c_void) -> FFISafeString {
        unsafe {
            let data = &*(ptr as *mut T);
            let fmt = ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("{0:?}", data))
            });
            FFISafeString::from(fmt)
        }
    }
    extern "C" fn no_display(_: *mut c_void) -> FFISafeString {
        FFISafeString::from(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("<cannot display type>"))
            }),
        )
    }
    impl FFIableObject {
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
        pub unsafe fn reconstruct<T: Debug>(mut self) -> T {
            if self.poisoned {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("FFIableObject is poisoned"),
                    );
                };
            }
            self.poisoned = true;
            (unsafe { Boxed::from_raw(self.data as *mut T) }).unbox()
        }
        /// Transfers the ownership to the new data and sets the `poisoned` field to `true` of this structure
        pub unsafe fn transfer_ownership(&mut self) -> FFIableObject {
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
        pub fn is_poisoned(&self) -> bool {
            self.poisoned
        }
        /// Get a mutable reference to the inner `FFIableObject`
        ///
        /// # Safety
        /// Do no use this is the struct is poisoned
        pub unsafe fn get_mut<'a, T>(&'a mut self) -> &'a mut T {
            unsafe { &mut *(self.data as *mut T) }
        }
        /// Get a mutable reference to the inner `FFIableObject`
        ///
        /// # Safety
        /// Do no use this is the struct is poisoned
        pub unsafe fn get<'a, T>(&'a self) -> &'a T {
            unsafe { &*(self.data as *mut T) }
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
            let data = data.as_str();
            let Some(data) = data else {
                return Err(std::fmt::Error::default());
            };
            f.write_str(data)
        }
    }
    impl Debug for FFIableObject {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let data = (self.fmt)(self.data);
            let data = data.as_str();
            let Some(data) = data else {
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
