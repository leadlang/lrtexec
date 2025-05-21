#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use std::{num::NonZeroU16, ptr};
use common::hashmap::RTVariableMap;
use stabby::str::Str as RStr;
use stabby::dynptr;
use stabby::{Any, boxed::Box as RBox};
use pastey::paste;
use critical::RustVariable;
pub mod common {
    use std::ffi::{CStr, CString, c_char};
    use crate::{CVariable, critical::RustVariable};
    pub mod hashmap {
        use std::collections::HashMap;
        use stabby::stabby;
        use crate::CVariable;
        #[deny(improper_ctypes_definitions)]
        pub trait RHashMap {
            extern "C" fn get<'a>(&'a self, key: &'a u16) -> Option<&'a CVariable>;
            extern "C" fn insert(&mut self, key: u16, value: CVariable);
        }
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        ///An stabby-generated item for [`RHashMap`]
        #[repr(C)]
        pub struct StabbyVtableRHashMap<'stabby_vt_lt> {
            ///An stabby-generated item for [`RHashMap`]
            pub get: stabby::abi::StableLike<
                for<'a> extern "C" fn(
                    stabby::abi::AnonymRef<'a>,
                    ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                    &'a u16,
                ) -> Option<&'a CVariable>,
                &'static (),
            >,
            ///An stabby-generated item for [`RHashMap`]
            pub insert: stabby::abi::StableLike<
                for<'stabby_receiver_lt> extern "C" fn(
                    stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                    ::core::marker::PhantomData<&'stabby_receiver_lt &'stabby_vt_lt ()>,
                    u16,
                    CVariable,
                ),
                &'static (),
            >,
        }
        #[automatically_derived]
        unsafe impl<'stabby_vt_lt> stabby::abi::IStable
        for StabbyVtableRHashMap<'stabby_vt_lt>
        where
            stabby::abi::Struct<
                stabby::abi::FieldPair<
                    stabby::abi::StableLike<
                        for<'a> extern "C" fn(
                            stabby::abi::AnonymRef<'a>,
                            ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            &'a u16,
                        ) -> Option<&'a CVariable>,
                        &'static (),
                    >,
                    stabby::abi::StableLike<
                        for<'stabby_receiver_lt> extern "C" fn(
                            stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                            ::core::marker::PhantomData<
                                &'stabby_receiver_lt &'stabby_vt_lt (),
                            >,
                            u16,
                            CVariable,
                        ),
                        &'static (),
                    >,
                >,
            >: stabby::abi::IStable,
            stabby::abi::StableLike<
                for<'stabby_receiver_lt> extern "C" fn(
                    stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                    ::core::marker::PhantomData<&'stabby_receiver_lt &'stabby_vt_lt ()>,
                    u16,
                    CVariable,
                ),
                &'static (),
            >: stabby::abi::IStable,
            stabby::abi::StableLike<
                for<'a> extern "C" fn(
                    stabby::abi::AnonymRef<'a>,
                    ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                    &'a u16,
                ) -> Option<&'a CVariable>,
                &'static (),
            >: stabby::abi::IStable,
        {
            type ForbiddenValues = <stabby::abi::Struct<
                stabby::abi::FieldPair<
                    stabby::abi::StableLike<
                        for<'a> extern "C" fn(
                            stabby::abi::AnonymRef<'a>,
                            ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            &'a u16,
                        ) -> Option<&'a CVariable>,
                        &'static (),
                    >,
                    stabby::abi::StableLike<
                        for<'stabby_receiver_lt> extern "C" fn(
                            stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                            ::core::marker::PhantomData<
                                &'stabby_receiver_lt &'stabby_vt_lt (),
                            >,
                            u16,
                            CVariable,
                        ),
                        &'static (),
                    >,
                >,
            > as stabby::abi::IStable>::ForbiddenValues;
            type UnusedBits = <stabby::abi::Struct<
                stabby::abi::FieldPair<
                    stabby::abi::StableLike<
                        for<'a> extern "C" fn(
                            stabby::abi::AnonymRef<'a>,
                            ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            &'a u16,
                        ) -> Option<&'a CVariable>,
                        &'static (),
                    >,
                    stabby::abi::StableLike<
                        for<'stabby_receiver_lt> extern "C" fn(
                            stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                            ::core::marker::PhantomData<
                                &'stabby_receiver_lt &'stabby_vt_lt (),
                            >,
                            u16,
                            CVariable,
                        ),
                        &'static (),
                    >,
                >,
            > as stabby::abi::IStable>::UnusedBits;
            type Size = <stabby::abi::Struct<
                stabby::abi::FieldPair<
                    stabby::abi::StableLike<
                        for<'a> extern "C" fn(
                            stabby::abi::AnonymRef<'a>,
                            ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            &'a u16,
                        ) -> Option<&'a CVariable>,
                        &'static (),
                    >,
                    stabby::abi::StableLike<
                        for<'stabby_receiver_lt> extern "C" fn(
                            stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                            ::core::marker::PhantomData<
                                &'stabby_receiver_lt &'stabby_vt_lt (),
                            >,
                            u16,
                            CVariable,
                        ),
                        &'static (),
                    >,
                >,
            > as stabby::abi::IStable>::Size;
            type Align = <stabby::abi::Struct<
                stabby::abi::FieldPair<
                    stabby::abi::StableLike<
                        for<'a> extern "C" fn(
                            stabby::abi::AnonymRef<'a>,
                            ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            &'a u16,
                        ) -> Option<&'a CVariable>,
                        &'static (),
                    >,
                    stabby::abi::StableLike<
                        for<'stabby_receiver_lt> extern "C" fn(
                            stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                            ::core::marker::PhantomData<
                                &'stabby_receiver_lt &'stabby_vt_lt (),
                            >,
                            u16,
                            CVariable,
                        ),
                        &'static (),
                    >,
                >,
            > as stabby::abi::IStable>::Align;
            type HasExactlyOneNiche = <stabby::abi::Struct<
                stabby::abi::FieldPair<
                    stabby::abi::StableLike<
                        for<'a> extern "C" fn(
                            stabby::abi::AnonymRef<'a>,
                            ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            &'a u16,
                        ) -> Option<&'a CVariable>,
                        &'static (),
                    >,
                    stabby::abi::StableLike<
                        for<'stabby_receiver_lt> extern "C" fn(
                            stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                            ::core::marker::PhantomData<
                                &'stabby_receiver_lt &'stabby_vt_lt (),
                            >,
                            u16,
                            CVariable,
                        ),
                        &'static (),
                    >,
                >,
            > as stabby::abi::IStable>::HasExactlyOneNiche;
            type ContainsIndirections = <stabby::abi::Struct<
                stabby::abi::FieldPair<
                    stabby::abi::StableLike<
                        for<'a> extern "C" fn(
                            stabby::abi::AnonymRef<'a>,
                            ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            &'a u16,
                        ) -> Option<&'a CVariable>,
                        &'static (),
                    >,
                    stabby::abi::StableLike<
                        for<'stabby_receiver_lt> extern "C" fn(
                            stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                            ::core::marker::PhantomData<
                                &'stabby_receiver_lt &'stabby_vt_lt (),
                            >,
                            u16,
                            CVariable,
                        ),
                        &'static (),
                    >,
                >,
            > as stabby::abi::IStable>::ContainsIndirections;
            const REPORT: &'static stabby::abi::report::TypeReport = &stabby::abi::report::TypeReport {
                name: stabby::abi::str::Str::new("StabbyVtableRHashMap"),
                module: stabby::abi::str::Str::new("lrtexec_types::common::hashmap"),
                fields: unsafe {
                    stabby::abi::StableLike::new(
                        Some(
                            &stabby::abi::report::FieldReport {
                                name: stabby::abi::str::Str::new("insert"),
                                ty: <stabby::abi::StableLike<
                                    for<'stabby_receiver_lt> extern "C" fn(
                                        stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                                        ::core::marker::PhantomData<
                                            &'stabby_receiver_lt &'stabby_vt_lt (),
                                        >,
                                        u16,
                                        CVariable,
                                    ),
                                    &'static (),
                                > as stabby::abi::IStable>::REPORT,
                                next_field: stabby::abi::StableLike::new(
                                    Some(
                                        &stabby::abi::report::FieldReport {
                                            name: stabby::abi::str::Str::new("get"),
                                            ty: <stabby::abi::StableLike<
                                                for<'a> extern "C" fn(
                                                    stabby::abi::AnonymRef<'a>,
                                                    ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                                                    &'a u16,
                                                ) -> Option<&'a CVariable>,
                                                &'static (),
                                            > as stabby::abi::IStable>::REPORT,
                                            next_field: stabby::abi::StableLike::new(None),
                                        },
                                    ),
                                ),
                            },
                        ),
                    )
                },
                version: 0u32,
                tyty: stabby::abi::report::TyTy::Struct,
            };
            const ID: u64 = {
                if core::mem::size_of::<Self>()
                    != <<Self as stabby::abi::IStable>::Size as stabby::abi::Unsigned>::USIZE
                {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "StabbyVtableRHashMap\'s size was mis-evaluated by stabby, this is definitely a bug and may cause UB, please file an issue",
                            ),
                        );
                    }
                }
                if core::mem::align_of::<Self>()
                    != <<Self as stabby::abi::IStable>::Align as stabby::abi::Unsigned>::USIZE
                {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "StabbyVtableRHashMap\'s align was mis-evaluated by stabby, this is definitely a bug and may cause UB, please file an issue",
                            ),
                        );
                    }
                }
                stabby::abi::report::gen_id(Self::REPORT)
            };
        }
        #[allow(dead_code, missing_docs)]
        struct OptimizedLayoutForStabbyVtableRHashMap<'stabby_vt_lt> {
            ///An stabby-generated item for [`RHashMap`]
            pub get: stabby::abi::StableLike<
                for<'a> extern "C" fn(
                    stabby::abi::AnonymRef<'a>,
                    ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                    &'a u16,
                ) -> Option<&'a CVariable>,
                &'static (),
            >,
            ///An stabby-generated item for [`RHashMap`]
            pub insert: stabby::abi::StableLike<
                for<'stabby_receiver_lt> extern "C" fn(
                    stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                    ::core::marker::PhantomData<&'stabby_receiver_lt &'stabby_vt_lt ()>,
                    u16,
                    CVariable,
                ),
                &'static (),
            >,
        }
        impl<'stabby_vt_lt> StabbyVtableRHashMap<'stabby_vt_lt>
        where
            stabby::abi::Struct<
                stabby::abi::FieldPair<
                    stabby::abi::StableLike<
                        for<'a> extern "C" fn(
                            stabby::abi::AnonymRef<'a>,
                            ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            &'a u16,
                        ) -> Option<&'a CVariable>,
                        &'static (),
                    >,
                    stabby::abi::StableLike<
                        for<'stabby_receiver_lt> extern "C" fn(
                            stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                            ::core::marker::PhantomData<
                                &'stabby_receiver_lt &'stabby_vt_lt (),
                            >,
                            u16,
                            CVariable,
                        ),
                        &'static (),
                    >,
                >,
            >: stabby::abi::IStable,
            stabby::abi::StableLike<
                for<'stabby_receiver_lt> extern "C" fn(
                    stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                    ::core::marker::PhantomData<&'stabby_receiver_lt &'stabby_vt_lt ()>,
                    u16,
                    CVariable,
                ),
                &'static (),
            >: stabby::abi::IStable,
            stabby::abi::StableLike<
                for<'a> extern "C" fn(
                    stabby::abi::AnonymRef<'a>,
                    ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                    &'a u16,
                ) -> Option<&'a CVariable>,
                &'static (),
            >: stabby::abi::IStable,
        {
            ///Returns true if the layout for [`StabbyVtableRHashMap`] is smaller or equal to that Rust would have generated for it.
            pub const fn has_optimal_layout() -> bool {
                core::mem::size_of::<Self>()
                    <= core::mem::size_of::<
                        OptimizedLayoutForStabbyVtableRHashMap<'stabby_vt_lt>,
                    >()
            }
        }
        impl<'stabby_vt_lt> Clone for StabbyVtableRHashMap<'stabby_vt_lt> {
            fn clone(&self) -> Self {
                *self
            }
        }
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        impl<'stabby_vt_lt> Copy for StabbyVtableRHashMap<'stabby_vt_lt> {}
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        impl<'stabby_vt_lt> core::cmp::PartialEq
        for StabbyVtableRHashMap<'stabby_vt_lt> {
            fn eq(&self, other: &Self) -> bool {
                core::ptr::eq(
                    (*unsafe { self.get.as_ref_unchecked() }) as *const (),
                    (*unsafe { other.get.as_ref_unchecked() }) as *const _,
                )
                    && core::ptr::eq(
                        (*unsafe { self.insert.as_ref_unchecked() }) as *const (),
                        (*unsafe { other.insert.as_ref_unchecked() }) as *const _,
                    ) && true
            }
        }
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        impl<'stabby_vt_lt> core::hash::Hash for StabbyVtableRHashMap<'stabby_vt_lt> {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.get.hash(state);
                self.insert.hash(state);
            }
        }
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        impl<'stabby_vt_lt> core::fmt::Debug for StabbyVtableRHashMap<'stabby_vt_lt> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let mut s = f.debug_struct("StabbyVtableRHashMap");
                s.field(
                    "get",
                    &format_args!("{0:p}", unsafe { self.get.as_ref_unchecked() }),
                );
                s.field(
                    "insert",
                    &format_args!("{0:p}", unsafe { self.insert.as_ref_unchecked() }),
                );
                s.finish()
            }
        }
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        impl<
            'stabby_vt_lt,
            StabbyArbitraryType,
        > stabby::abi::vtable::IConstConstructor<'stabby_vt_lt, StabbyArbitraryType>
        for StabbyVtableRHashMap<'stabby_vt_lt>
        where
            StabbyArbitraryType: RHashMap,
        {
            const VTABLE: StabbyVtableRHashMap<'stabby_vt_lt> = StabbyVtableRHashMap {
                get: unsafe {
                    stabby::abi::StableLike::new({
                        extern "C" fn ext_get<
                            'stabby_local_lt,
                            'a,
                            StabbyArbitraryType: 'stabby_local_lt,
                        >(
                            this: stabby::abi::AnonymRef<'a>,
                            _lt_proof: ::core::marker::PhantomData<
                                &'a &'stabby_local_lt (),
                            >,
                            _0: &'a u16,
                        ) -> Option<&'a CVariable>
                        where
                            StabbyArbitraryType: RHashMap,
                        {
                            unsafe {
                                <StabbyArbitraryType as RHashMap>::get(
                                    this.cast::<StabbyArbitraryType>().as_ref(),
                                    _0,
                                )
                            }
                        }
                        ext_get::<StabbyArbitraryType>
                            as for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                                &'a u16,
                            ) -> Option<&'a CVariable>
                    })
                },
                insert: unsafe {
                    stabby::abi::StableLike::new({
                        extern "C" fn ext_insert<
                            'stabby_local_lt,
                            'stabby_receiver_lt,
                            StabbyArbitraryType: 'stabby_local_lt,
                        >(
                            this: stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                            _lt_proof: ::core::marker::PhantomData<
                                &'stabby_receiver_lt &'stabby_local_lt (),
                            >,
                            _0: u16,
                            _1: CVariable,
                        )
                        where
                            StabbyArbitraryType: RHashMap,
                        {
                            unsafe {
                                <StabbyArbitraryType as RHashMap>::insert(
                                    this.cast::<StabbyArbitraryType>().as_mut(),
                                    _0,
                                    _1,
                                )
                            }
                        }
                        ext_insert::<StabbyArbitraryType>
                            as for<'stabby_receiver_lt> extern "C" fn(
                                stabby::abi::AnonymRefMut<'stabby_receiver_lt>,
                                ::core::marker::PhantomData<
                                    &'stabby_receiver_lt &'stabby_vt_lt (),
                                >,
                                u16,
                                CVariable,
                            )
                    })
                },
            };
        }
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        impl<'stabby_vt_lt> stabby::abi::vtable::CompoundVt<'stabby_vt_lt>
        for dyn RHashMap {
            ///An stabby-generated item for [`RHashMap`]
            type Vt<StabbyNextVtable> = stabby::abi::vtable::VTable<
                StabbyVtableRHashMap<'stabby_vt_lt>,
                StabbyNextVtable,
            >;
        }
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        ///An stabby-generated item for [`RHashMap`]
        pub trait RHashMapDyn<StabbyTransitiveDerefN> {
            ///An stabby-generated item for [`RHashMap`]
            extern "C" fn get<'a>(&'a self, _0: &'a u16) -> Option<&'a CVariable>;
        }
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        impl<
            'stabby_vt_lt,
            StabbyVtProvider: stabby::abi::vtable::TransitiveDeref<
                    StabbyVtableRHashMap<'stabby_vt_lt>,
                    StabbyTransitiveDerefN,
                > + Copy,
            StabbyTransitiveDerefN,
        > RHashMapDyn<StabbyTransitiveDerefN>
        for stabby::abi::DynRef<'_, StabbyVtProvider> {
            ///An stabby-generated item for [`RHashMap`]
            extern "C" fn get<'a>(&'a self, _0: &'a u16) -> Option<&'a CVariable> {
                unsafe {
                    (self
                        .vtable()
                        .tderef()
                        .get
                        .as_ref_unchecked())(self.ptr(), ::core::marker::PhantomData, _0)
                }
            }
        }
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        impl<
            'stabby_vt_lt,
            StabbyPtrProvider: stabby::abi::IPtrOwned + stabby::abi::IPtr,
            StabbyVtProvider: stabby::abi::vtable::HasDropVt + Copy
                + stabby::abi::vtable::TransitiveDeref<
                    StabbyVtableRHashMap<'stabby_vt_lt>,
                    StabbyTransitiveDerefN,
                >,
            StabbyTransitiveDerefN,
        > RHashMapDyn<StabbyTransitiveDerefN>
        for stabby::abi::Dyn<'_, StabbyPtrProvider, StabbyVtProvider> {
            ///An stabby-generated item for [`RHashMap`]
            extern "C" fn get<'a>(&'a self, _0: &'a u16) -> Option<&'a CVariable> {
                unsafe {
                    (self
                        .vtable()
                        .tderef()
                        .get
                        .as_ref_unchecked())(
                        self.ptr().as_ref(),
                        ::core::marker::PhantomData,
                        _0,
                    )
                }
            }
        }
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        ///An stabby-generated item for [`RHashMap`]
        pub trait RHashMapDynMut<
            StabbyTransitiveDerefN,
        >: RHashMapDyn<StabbyTransitiveDerefN> {
            ///An stabby-generated item for [`RHashMap`]
            extern "C" fn insert(&mut self, _0: u16, _1: CVariable);
        }
        #[allow(unknown_lints)]
        #[allow(clippy::multiple_bound_locations)]
        impl<
            'stabby_vt_lt,
            StabbyPtrProvider,
            StabbyVtProvider,
            StabbyTransitiveDerefN,
        > RHashMapDynMut<StabbyTransitiveDerefN>
        for stabby::abi::Dyn<'_, StabbyPtrProvider, StabbyVtProvider>
        where
            StabbyPtrProvider: stabby::abi::IPtrOwned + stabby::abi::IPtrMut,
            StabbyVtProvider: stabby::abi::vtable::HasDropVt + Copy
                + stabby::abi::vtable::TransitiveDeref<
                    StabbyVtableRHashMap<'stabby_vt_lt>,
                    StabbyTransitiveDerefN,
                >,
        {
            ///An stabby-generated item for [`RHashMap`]
            extern "C" fn insert(&mut self, _0: u16, _1: CVariable) {
                unsafe {
                    (self
                        .vtable()
                        .tderef()
                        .insert
                        .as_ref_unchecked())(
                        self.ptr_mut().as_mut(),
                        ::core::marker::PhantomData,
                        _0,
                        _1,
                    )
                }
            }
        }
        pub struct RTVariableMap {
            map: HashMap<u16, CVariable>,
        }
        impl RTVariableMap {
            pub fn new() -> Self {
                Self { map: HashMap::new() }
            }
        }
        impl RHashMap for RTVariableMap {
            extern "C" fn get<'a>(&'a self, key: &'a u16) -> Option<&'a CVariable> {
                self.map.get(key)
            }
            extern "C" fn insert(&mut self, key: u16, value: CVariable) {
                self.map.insert(key, value);
            }
        }
    }
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
pub mod commands {
    pub mod v0 {}
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
    pub itself: Option<NonZeroU16>,
    pub r1: Option<NonZeroU16>,
    pub r2: Option<NonZeroU16>,
    pub r3: Option<NonZeroU16>,
    pub r4: Option<NonZeroU16>,
    pub r5: Option<NonZeroU16>,
    pub r6: Option<NonZeroU16>,
    pub r7: Option<NonZeroU16>,
    pub r8: Option<NonZeroU16>,
}
impl FnStack {
    #[unsafe(no_mangle)]
    pub extern "C" fn create() -> Self {
        Self {
            ret: MaybeNone,
            itself: None,
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
    pub fn clear(&mut self) {
        self.itself = None;
        self.r1 = None;
        self.r2 = None;
        self.r3 = None;
        self.r4 = None;
        self.r5 = None;
        self.r6 = None;
        self.r7 = None;
        self.r8 = None;
    }
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
impl CVariable {
    pub unsafe fn get_u32(&self) -> u32 {
        let data: &RustVariable<Wrapper<u32>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
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
impl<T: Copy + Clone> Wrapper<T> {
    pub fn inner(&self) -> T {
        self.0
    }
}
impl CVariable {
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_u8(value: Wrapper<u8>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_u8(self) -> u8 {
        let data: &RustVariable<Wrapper<u8>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_u16(value: Wrapper<u16>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_u16(self) -> u16 {
        let data: &RustVariable<Wrapper<u16>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_u32(value: Wrapper<u32>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_u32(self) -> u32 {
        let data: &RustVariable<Wrapper<u32>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_u64(value: Wrapper<u64>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_u64(self) -> u64 {
        let data: &RustVariable<Wrapper<u64>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_usize(value: Wrapper<usize>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_usize(self) -> usize {
        let data: &RustVariable<Wrapper<usize>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_i8(value: Wrapper<i8>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_i8(self) -> i8 {
        let data: &RustVariable<Wrapper<i8>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_i16(value: Wrapper<i16>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_i16(self) -> i16 {
        let data: &RustVariable<Wrapper<i16>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_i32(value: Wrapper<i32>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_i32(self) -> i32 {
        let data: &RustVariable<Wrapper<i32>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_i64(value: Wrapper<i64>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_i64(self) -> i64 {
        let data: &RustVariable<Wrapper<i64>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_f32(value: Wrapper<f32>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_f32(self) -> f32 {
        let data: &RustVariable<Wrapper<f32>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_f64(value: Wrapper<f64>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_f64(self) -> f64 {
        let data: &RustVariable<Wrapper<f64>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_isize(value: Wrapper<isize>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_isize(self) -> isize {
        let data: &RustVariable<Wrapper<isize>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
    }
    #[unsafe(no_mangle)]
    pub extern "C" fn into_common_bool(value: Wrapper<bool>) -> Self {
        RustVariable::new(value).to_c()
    }
    /// This creates a Clone of the inner value
    #[unsafe(no_mangle)]
    pub extern "C" fn get_common_bool(self) -> bool {
        let data: &RustVariable<Wrapper<bool>> = unsafe {
            RustVariable::from_ptr(self.data)
        };
        data.inner()
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
impl From<Wrapper<usize>> for CVariable {
    fn from(value: Wrapper<usize>) -> Self {
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
impl From<Wrapper<isize>> for CVariable {
    fn from(value: Wrapper<isize>) -> Self {
        RustVariable::new(value).to_c()
    }
}
impl From<Wrapper<bool>> for CVariable {
    fn from(value: Wrapper<bool>) -> Self {
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
#[repr(C)]
pub struct MemoryMap {
    variables: RTVariableMap,
}
impl MemoryMap {}
#[repr(C)]
pub enum VariableData {
    Constant(RStr<'static>),
    Raw(CVariable),
    Abi(
        stabby::abi::Dyn<
            'static,
            RBox<()>,
            <dyn Any as stabby::abi::vtable::CompoundVt<
                'static,
            >>::Vt<stabby::abi::vtable::VtDrop>,
        >,
    ),
}
