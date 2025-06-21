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
            use std::{fmt::Debug, num::NonZeroU16};
            use stabby::{
                boxed::{Box as RBox, BoxedStr},
                dynptr, stabby, str::Str, sync::Arc,
            };
            #[repr(C)]
            pub struct FnStackV0 {
                /// Return value (identifier in MemoryMap)
                pub ret: Option<VariableDataV0>,
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
            #[automatically_derived]
            unsafe impl stabby::abi::IStable for FnStackV0
            where
                stabby::abi::Struct<
                    stabby::abi::FieldPair<
                        stabby::abi::FieldPair<
                            stabby::abi::FieldPair<
                                stabby::abi::FieldPair<
                                    stabby::abi::FieldPair<
                                        stabby::abi::FieldPair<
                                            stabby::abi::FieldPair<
                                                stabby::abi::FieldPair<
                                                    stabby::abi::FieldPair<
                                                        Option<VariableDataV0>,
                                                        Option<NonZeroU16>,
                                                    >,
                                                    Option<NonZeroU16>,
                                                >,
                                                Option<NonZeroU16>,
                                            >,
                                            Option<NonZeroU16>,
                                        >,
                                        Option<NonZeroU16>,
                                    >,
                                    Option<NonZeroU16>,
                                >,
                                Option<NonZeroU16>,
                            >,
                            Option<NonZeroU16>,
                        >,
                        Option<NonZeroU16>,
                    >,
                >: stabby::abi::IStable,
                Option<NonZeroU16>: stabby::abi::IStable,
                Option<VariableDataV0>: stabby::abi::IStable,
            {
                type ForbiddenValues = <stabby::abi::Struct<
                    stabby::abi::FieldPair<
                        stabby::abi::FieldPair<
                            stabby::abi::FieldPair<
                                stabby::abi::FieldPair<
                                    stabby::abi::FieldPair<
                                        stabby::abi::FieldPair<
                                            stabby::abi::FieldPair<
                                                stabby::abi::FieldPair<
                                                    stabby::abi::FieldPair<
                                                        Option<VariableDataV0>,
                                                        Option<NonZeroU16>,
                                                    >,
                                                    Option<NonZeroU16>,
                                                >,
                                                Option<NonZeroU16>,
                                            >,
                                            Option<NonZeroU16>,
                                        >,
                                        Option<NonZeroU16>,
                                    >,
                                    Option<NonZeroU16>,
                                >,
                                Option<NonZeroU16>,
                            >,
                            Option<NonZeroU16>,
                        >,
                        Option<NonZeroU16>,
                    >,
                > as stabby::abi::IStable>::ForbiddenValues;
                type UnusedBits = <stabby::abi::Struct<
                    stabby::abi::FieldPair<
                        stabby::abi::FieldPair<
                            stabby::abi::FieldPair<
                                stabby::abi::FieldPair<
                                    stabby::abi::FieldPair<
                                        stabby::abi::FieldPair<
                                            stabby::abi::FieldPair<
                                                stabby::abi::FieldPair<
                                                    stabby::abi::FieldPair<
                                                        Option<VariableDataV0>,
                                                        Option<NonZeroU16>,
                                                    >,
                                                    Option<NonZeroU16>,
                                                >,
                                                Option<NonZeroU16>,
                                            >,
                                            Option<NonZeroU16>,
                                        >,
                                        Option<NonZeroU16>,
                                    >,
                                    Option<NonZeroU16>,
                                >,
                                Option<NonZeroU16>,
                            >,
                            Option<NonZeroU16>,
                        >,
                        Option<NonZeroU16>,
                    >,
                > as stabby::abi::IStable>::UnusedBits;
                type Size = <stabby::abi::Struct<
                    stabby::abi::FieldPair<
                        stabby::abi::FieldPair<
                            stabby::abi::FieldPair<
                                stabby::abi::FieldPair<
                                    stabby::abi::FieldPair<
                                        stabby::abi::FieldPair<
                                            stabby::abi::FieldPair<
                                                stabby::abi::FieldPair<
                                                    stabby::abi::FieldPair<
                                                        Option<VariableDataV0>,
                                                        Option<NonZeroU16>,
                                                    >,
                                                    Option<NonZeroU16>,
                                                >,
                                                Option<NonZeroU16>,
                                            >,
                                            Option<NonZeroU16>,
                                        >,
                                        Option<NonZeroU16>,
                                    >,
                                    Option<NonZeroU16>,
                                >,
                                Option<NonZeroU16>,
                            >,
                            Option<NonZeroU16>,
                        >,
                        Option<NonZeroU16>,
                    >,
                > as stabby::abi::IStable>::Size;
                type Align = <stabby::abi::Struct<
                    stabby::abi::FieldPair<
                        stabby::abi::FieldPair<
                            stabby::abi::FieldPair<
                                stabby::abi::FieldPair<
                                    stabby::abi::FieldPair<
                                        stabby::abi::FieldPair<
                                            stabby::abi::FieldPair<
                                                stabby::abi::FieldPair<
                                                    stabby::abi::FieldPair<
                                                        Option<VariableDataV0>,
                                                        Option<NonZeroU16>,
                                                    >,
                                                    Option<NonZeroU16>,
                                                >,
                                                Option<NonZeroU16>,
                                            >,
                                            Option<NonZeroU16>,
                                        >,
                                        Option<NonZeroU16>,
                                    >,
                                    Option<NonZeroU16>,
                                >,
                                Option<NonZeroU16>,
                            >,
                            Option<NonZeroU16>,
                        >,
                        Option<NonZeroU16>,
                    >,
                > as stabby::abi::IStable>::Align;
                type HasExactlyOneNiche = <stabby::abi::Struct<
                    stabby::abi::FieldPair<
                        stabby::abi::FieldPair<
                            stabby::abi::FieldPair<
                                stabby::abi::FieldPair<
                                    stabby::abi::FieldPair<
                                        stabby::abi::FieldPair<
                                            stabby::abi::FieldPair<
                                                stabby::abi::FieldPair<
                                                    stabby::abi::FieldPair<
                                                        Option<VariableDataV0>,
                                                        Option<NonZeroU16>,
                                                    >,
                                                    Option<NonZeroU16>,
                                                >,
                                                Option<NonZeroU16>,
                                            >,
                                            Option<NonZeroU16>,
                                        >,
                                        Option<NonZeroU16>,
                                    >,
                                    Option<NonZeroU16>,
                                >,
                                Option<NonZeroU16>,
                            >,
                            Option<NonZeroU16>,
                        >,
                        Option<NonZeroU16>,
                    >,
                > as stabby::abi::IStable>::HasExactlyOneNiche;
                type ContainsIndirections = <stabby::abi::Struct<
                    stabby::abi::FieldPair<
                        stabby::abi::FieldPair<
                            stabby::abi::FieldPair<
                                stabby::abi::FieldPair<
                                    stabby::abi::FieldPair<
                                        stabby::abi::FieldPair<
                                            stabby::abi::FieldPair<
                                                stabby::abi::FieldPair<
                                                    stabby::abi::FieldPair<
                                                        Option<VariableDataV0>,
                                                        Option<NonZeroU16>,
                                                    >,
                                                    Option<NonZeroU16>,
                                                >,
                                                Option<NonZeroU16>,
                                            >,
                                            Option<NonZeroU16>,
                                        >,
                                        Option<NonZeroU16>,
                                    >,
                                    Option<NonZeroU16>,
                                >,
                                Option<NonZeroU16>,
                            >,
                            Option<NonZeroU16>,
                        >,
                        Option<NonZeroU16>,
                    >,
                > as stabby::abi::IStable>::ContainsIndirections;
                const REPORT: &'static stabby::abi::report::TypeReport = &stabby::abi::report::TypeReport {
                    name: stabby::abi::str::Str::new("FnStackV0"),
                    module: stabby::abi::str::Str::new(
                        "lrtexec_lib::commands::v0::structs",
                    ),
                    fields: unsafe {
                        stabby::abi::StableLike::new(
                            Some(
                                &stabby::abi::report::FieldReport {
                                    name: stabby::abi::str::Str::new("r8"),
                                    ty: <Option<NonZeroU16> as stabby::abi::IStable>::REPORT,
                                    next_field: stabby::abi::StableLike::new(
                                        Some(
                                            &stabby::abi::report::FieldReport {
                                                name: stabby::abi::str::Str::new("r7"),
                                                ty: <Option<NonZeroU16> as stabby::abi::IStable>::REPORT,
                                                next_field: stabby::abi::StableLike::new(
                                                    Some(
                                                        &stabby::abi::report::FieldReport {
                                                            name: stabby::abi::str::Str::new("r6"),
                                                            ty: <Option<NonZeroU16> as stabby::abi::IStable>::REPORT,
                                                            next_field: stabby::abi::StableLike::new(
                                                                Some(
                                                                    &stabby::abi::report::FieldReport {
                                                                        name: stabby::abi::str::Str::new("r5"),
                                                                        ty: <Option<NonZeroU16> as stabby::abi::IStable>::REPORT,
                                                                        next_field: stabby::abi::StableLike::new(
                                                                            Some(
                                                                                &stabby::abi::report::FieldReport {
                                                                                    name: stabby::abi::str::Str::new("r4"),
                                                                                    ty: <Option<NonZeroU16> as stabby::abi::IStable>::REPORT,
                                                                                    next_field: stabby::abi::StableLike::new(
                                                                                        Some(
                                                                                            &stabby::abi::report::FieldReport {
                                                                                                name: stabby::abi::str::Str::new("r3"),
                                                                                                ty: <Option<NonZeroU16> as stabby::abi::IStable>::REPORT,
                                                                                                next_field: stabby::abi::StableLike::new(
                                                                                                    Some(
                                                                                                        &stabby::abi::report::FieldReport {
                                                                                                            name: stabby::abi::str::Str::new("r2"),
                                                                                                            ty: <Option<NonZeroU16> as stabby::abi::IStable>::REPORT,
                                                                                                            next_field: stabby::abi::StableLike::new(
                                                                                                                Some(
                                                                                                                    &stabby::abi::report::FieldReport {
                                                                                                                        name: stabby::abi::str::Str::new("r1"),
                                                                                                                        ty: <Option<NonZeroU16> as stabby::abi::IStable>::REPORT,
                                                                                                                        next_field: stabby::abi::StableLike::new(
                                                                                                                            Some(
                                                                                                                                &stabby::abi::report::FieldReport {
                                                                                                                                    name: stabby::abi::str::Str::new("itself"),
                                                                                                                                    ty: <Option<NonZeroU16> as stabby::abi::IStable>::REPORT,
                                                                                                                                    next_field: stabby::abi::StableLike::new(
                                                                                                                                        Some(
                                                                                                                                            &stabby::abi::report::FieldReport {
                                                                                                                                                name: stabby::abi::str::Str::new("ret"),
                                                                                                                                                ty: <Option<
                                                                                                                                                    VariableDataV0,
                                                                                                                                                > as stabby::abi::IStable>::REPORT,
                                                                                                                                                next_field: stabby::abi::StableLike::new(None),
                                                                                                                                            },
                                                                                                                                        ),
                                                                                                                                    ),
                                                                                                                                },
                                                                                                                            ),
                                                                                                                        ),
                                                                                                                    },
                                                                                                                ),
                                                                                                            ),
                                                                                                        },
                                                                                                    ),
                                                                                                ),
                                                                                            },
                                                                                        ),
                                                                                    ),
                                                                                },
                                                                            ),
                                                                        ),
                                                                    },
                                                                ),
                                                            ),
                                                        },
                                                    ),
                                                ),
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
                                    "FnStackV0\'s size was mis-evaluated by stabby, this is definitely a bug and may cause UB, please file an issue",
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
                                    "FnStackV0\'s align was mis-evaluated by stabby, this is definitely a bug and may cause UB, please file an issue",
                                ),
                            );
                        }
                    }
                    stabby::abi::report::gen_id(Self::REPORT)
                };
            }
            #[allow(dead_code, missing_docs)]
            struct OptimizedLayoutForFnStackV0 {
                /// Return value (identifier in MemoryMap)
                pub ret: Option<VariableDataV0>,
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
            const _: () = {
                if !<FnStackV0>::has_optimal_layout() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "FnStackV0\'s layout is sub-optimal, reorder fields or use `#[stabby::stabby(no_opt)]`",
                            ),
                        );
                    }
                }
            };
            impl FnStackV0
            where
                stabby::abi::Struct<
                    stabby::abi::FieldPair<
                        stabby::abi::FieldPair<
                            stabby::abi::FieldPair<
                                stabby::abi::FieldPair<
                                    stabby::abi::FieldPair<
                                        stabby::abi::FieldPair<
                                            stabby::abi::FieldPair<
                                                stabby::abi::FieldPair<
                                                    stabby::abi::FieldPair<
                                                        Option<VariableDataV0>,
                                                        Option<NonZeroU16>,
                                                    >,
                                                    Option<NonZeroU16>,
                                                >,
                                                Option<NonZeroU16>,
                                            >,
                                            Option<NonZeroU16>,
                                        >,
                                        Option<NonZeroU16>,
                                    >,
                                    Option<NonZeroU16>,
                                >,
                                Option<NonZeroU16>,
                            >,
                            Option<NonZeroU16>,
                        >,
                        Option<NonZeroU16>,
                    >,
                >: stabby::abi::IStable,
                Option<NonZeroU16>: stabby::abi::IStable,
                Option<VariableDataV0>: stabby::abi::IStable,
            {
                ///Returns true if the layout for [`FnStackV0`] is smaller or equal to that Rust would have generated for it.
                pub const fn has_optimal_layout() -> bool {
                    core::mem::size_of::<Self>()
                        <= core::mem::size_of::<OptimizedLayoutForFnStackV0>()
                }
            }
            pub struct FmtOutput<'a>(
                stabby::abi::Result<BoxedStr, Str<'a>>,
            )
            where
                Str<'a>: stabby::abi::IStable,
                BoxedStr: stabby::abi::IStable,
                BoxedStr: stabby::abi::IDeterminantProvider<Str<'a>>,
                Str<'a>: stabby::abi::IStable;
            #[allow(dead_code)]
            #[repr(u8)]
            enum ReprCLayoutForFmtOutput<'a>
            where
                BoxedStr: stabby::abi::IDeterminantProvider<Str<'a>>,
                Str<'a>: stabby::abi::IStable,
            {
                String(BoxedStr),
                Str(Str<'a>),
            }
            impl<'a> FmtOutput<'a>
            where
                Str<'a>: stabby::abi::IStable,
                BoxedStr: stabby::abi::IStable,
                BoxedStr: stabby::abi::IDeterminantProvider<Str<'a>>,
                Str<'a>: stabby::abi::IStable,
                stabby::abi::Result<BoxedStr, Str<'a>>: stabby::abi::IStable,
            {
                ///Returns true if the layout for [`FmtOutput`] is smaller than what `#[repr(C)]` would have generated for it.
                pub const fn has_optimal_layout() -> bool {
                    core::mem::size_of::<Self>()
                        < core::mem::size_of::<ReprCLayoutForFmtOutput<'a>>()
                }
            }
            #[automatically_derived]
            unsafe impl<'a> stabby::abi::IStable for FmtOutput<'a>
            where
                Str<'a>: stabby::abi::IStable,
                BoxedStr: stabby::abi::IStable,
                BoxedStr: stabby::abi::IDeterminantProvider<Str<'a>>,
                Str<'a>: stabby::abi::IStable,
                stabby::abi::Result<BoxedStr, Str<'a>>: stabby::abi::IStable,
            {
                type ForbiddenValues = <stabby::abi::Result<
                    BoxedStr,
                    Str<'a>,
                > as stabby::abi::IStable>::ForbiddenValues;
                type UnusedBits = <stabby::abi::Result<
                    BoxedStr,
                    Str<'a>,
                > as stabby::abi::IStable>::UnusedBits;
                type Size = <stabby::abi::Result<
                    BoxedStr,
                    Str<'a>,
                > as stabby::abi::IStable>::Size;
                type Align = <stabby::abi::Result<
                    BoxedStr,
                    Str<'a>,
                > as stabby::abi::IStable>::Align;
                type HasExactlyOneNiche = stabby::abi::B0;
                type ContainsIndirections = <stabby::abi::Result<
                    BoxedStr,
                    Str<'a>,
                > as stabby::abi::IStable>::ContainsIndirections;
                const REPORT: &'static stabby::abi::report::TypeReport = &stabby::abi::report::TypeReport {
                    name: stabby::abi::str::Str::new("FmtOutput"),
                    module: stabby::abi::str::Str::new(
                        "lrtexec_lib::commands::v0::structs",
                    ),
                    fields: unsafe {
                        stabby::abi::StableLike::new(
                            Some(
                                &stabby::abi::report::FieldReport {
                                    name: stabby::abi::str::Str::new("Str"),
                                    ty: <Str<'a> as stabby::abi::IStable>::REPORT,
                                    next_field: stabby::abi::StableLike::new(
                                        Some(
                                            &stabby::abi::report::FieldReport {
                                                name: stabby::abi::str::Str::new("String"),
                                                ty: <BoxedStr as stabby::abi::IStable>::REPORT,
                                                next_field: stabby::abi::StableLike::new(None),
                                            },
                                        ),
                                    ),
                                },
                            ),
                        )
                    },
                    version: 0u32,
                    tyty: stabby::abi::report::TyTy::Enum(
                        stabby::abi::str::Str::new("stabby"),
                    ),
                };
                const ID: u64 = stabby::abi::report::gen_id(Self::REPORT);
            }
            #[automatically_derived]
            impl<'a> FmtOutput<'a>
            where
                Str<'a>: stabby::abi::IStable,
                BoxedStr: stabby::abi::IStable,
                BoxedStr: stabby::abi::IDeterminantProvider<Str<'a>>,
                Str<'a>: stabby::abi::IStable,
            {
                #[allow(non_snake_case)]
                pub fn String(value: BoxedStr) -> Self {
                    Self(stabby::abi::Result::Ok(value))
                }
                #[allow(non_snake_case)]
                pub fn Str(value: Str<'a>) -> Self {
                    Self(stabby::abi::Result::Err(value))
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match self`.
                pub fn match_owned<
                    StabbyOut,
                    StringFn: FnOnce(BoxedStr) -> StabbyOut,
                    StrFn: FnOnce(Str<'a>) -> StabbyOut,
                >(self, String: StringFn, Str: StrFn) -> StabbyOut {
                    self.0.match_owned(String, Str)
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match &self`.
                pub fn match_ref<
                    'st_lt,
                    StabbyOut,
                    StringFn: FnOnce(&'st_lt BoxedStr) -> StabbyOut,
                    StrFn: FnOnce(&'st_lt Str<'a>) -> StabbyOut,
                >(&'st_lt self, String: StringFn, Str: StrFn) -> StabbyOut {
                    self.0.match_ref(String, Str)
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match &mut self`.
                pub fn match_mut<
                    'st_lt,
                    StabbyOut,
                    StringFn: FnOnce(&mut BoxedStr) -> StabbyOut,
                    StrFn: FnOnce(&mut Str<'a>) -> StabbyOut,
                >(&'st_lt mut self, String: StringFn, Str: StrFn) -> StabbyOut {
                    self.0.match_mut(|mut a| String(&mut *a), |mut a| Str(&mut *a))
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match self`, but allows you to pass common arguments to all closures to make the borrow checker happy.
                pub fn match_owned_ctx<
                    StabbyOut,
                    StabbyCtx,
                    StringFn: FnOnce(StabbyCtx, BoxedStr) -> StabbyOut,
                    StrFn: FnOnce(StabbyCtx, Str<'a>) -> StabbyOut,
                >(
                    self,
                    stabby_ctx: StabbyCtx,
                    String: StringFn,
                    Str: StrFn,
                ) -> StabbyOut {
                    self.0.match_owned_ctx(stabby_ctx, String, Str)
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match &self`, but allows you to pass common arguments to all closures to make the borrow checker happy.
                pub fn match_ref_ctx<
                    'st_lt,
                    StabbyCtx,
                    StabbyOut,
                    StringFn: FnOnce(StabbyCtx, &'st_lt BoxedStr) -> StabbyOut,
                    StrFn: FnOnce(StabbyCtx, &'st_lt Str<'a>) -> StabbyOut,
                >(
                    &'st_lt self,
                    stabby_ctx: StabbyCtx,
                    String: StringFn,
                    Str: StrFn,
                ) -> StabbyOut {
                    self.0.match_ref_ctx(stabby_ctx, String, Str)
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match &mut self`, but allows you to pass common arguments to all closures to make the borrow checker happy.
                pub fn match_mut_ctx<
                    'st_lt,
                    StabbyCtx,
                    StabbyOut,
                    StringFn: FnOnce(StabbyCtx, &mut BoxedStr) -> StabbyOut,
                    StrFn: FnOnce(StabbyCtx, &mut Str<'a>) -> StabbyOut,
                >(
                    &'st_lt mut self,
                    stabby_ctx: StabbyCtx,
                    String: StringFn,
                    Str: StrFn,
                ) -> StabbyOut {
                    self.0
                        .match_mut_ctx(
                            stabby_ctx,
                            |stabby_ctx, mut a| String(stabby_ctx, &mut *a),
                            |stabby_ctx, mut a| Str(stabby_ctx, &mut *a),
                        )
                }
            }
            #[deny(improper_ctypes_definitions)]
            pub trait ObjectV0 {
                extern "C" fn debug_fmt<'a>(&'a self) -> Option<FmtOutput<'a>>;
                extern "C" fn display_fmt<'a>(&'a self) -> Option<FmtOutput<'a>>;
            }
            #[allow(unknown_lints)]
            #[allow(clippy::multiple_bound_locations)]
            ///An stabby-generated item for [`ObjectV0`]
            #[repr(C)]
            pub struct StabbyVtableObjectV0<'stabby_vt_lt> {
                ///An stabby-generated item for [`ObjectV0`]
                pub debug_fmt: stabby::abi::StableLike<
                    for<'a> extern "C" fn(
                        stabby::abi::AnonymRef<'a>,
                        ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                    ) -> Option<FmtOutput<'a>>,
                    &'static (),
                >,
                ///An stabby-generated item for [`ObjectV0`]
                pub display_fmt: stabby::abi::StableLike<
                    for<'a> extern "C" fn(
                        stabby::abi::AnonymRef<'a>,
                        ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                    ) -> Option<FmtOutput<'a>>,
                    &'static (),
                >,
            }
            #[automatically_derived]
            unsafe impl<'stabby_vt_lt> stabby::abi::IStable
            for StabbyVtableObjectV0<'stabby_vt_lt>
            where
                stabby::abi::Struct<
                    stabby::abi::FieldPair<
                        stabby::abi::StableLike<
                            for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            ) -> Option<FmtOutput<'a>>,
                            &'static (),
                        >,
                        stabby::abi::StableLike<
                            for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            ) -> Option<FmtOutput<'a>>,
                            &'static (),
                        >,
                    >,
                >: stabby::abi::IStable,
                stabby::abi::StableLike<
                    for<'a> extern "C" fn(
                        stabby::abi::AnonymRef<'a>,
                        ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                    ) -> Option<FmtOutput<'a>>,
                    &'static (),
                >: stabby::abi::IStable,
            {
                type ForbiddenValues = <stabby::abi::Struct<
                    stabby::abi::FieldPair<
                        stabby::abi::StableLike<
                            for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            ) -> Option<FmtOutput<'a>>,
                            &'static (),
                        >,
                        stabby::abi::StableLike<
                            for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            ) -> Option<FmtOutput<'a>>,
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
                            ) -> Option<FmtOutput<'a>>,
                            &'static (),
                        >,
                        stabby::abi::StableLike<
                            for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            ) -> Option<FmtOutput<'a>>,
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
                            ) -> Option<FmtOutput<'a>>,
                            &'static (),
                        >,
                        stabby::abi::StableLike<
                            for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            ) -> Option<FmtOutput<'a>>,
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
                            ) -> Option<FmtOutput<'a>>,
                            &'static (),
                        >,
                        stabby::abi::StableLike<
                            for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            ) -> Option<FmtOutput<'a>>,
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
                            ) -> Option<FmtOutput<'a>>,
                            &'static (),
                        >,
                        stabby::abi::StableLike<
                            for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            ) -> Option<FmtOutput<'a>>,
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
                            ) -> Option<FmtOutput<'a>>,
                            &'static (),
                        >,
                        stabby::abi::StableLike<
                            for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            ) -> Option<FmtOutput<'a>>,
                            &'static (),
                        >,
                    >,
                > as stabby::abi::IStable>::ContainsIndirections;
                const REPORT: &'static stabby::abi::report::TypeReport = &stabby::abi::report::TypeReport {
                    name: stabby::abi::str::Str::new("StabbyVtableObjectV0"),
                    module: stabby::abi::str::Str::new(
                        "lrtexec_lib::commands::v0::structs",
                    ),
                    fields: unsafe {
                        stabby::abi::StableLike::new(
                            Some(
                                &stabby::abi::report::FieldReport {
                                    name: stabby::abi::str::Str::new("display_fmt"),
                                    ty: <stabby::abi::StableLike<
                                        for<'a> extern "C" fn(
                                            stabby::abi::AnonymRef<'a>,
                                            ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                                        ) -> Option<FmtOutput<'a>>,
                                        &'static (),
                                    > as stabby::abi::IStable>::REPORT,
                                    next_field: stabby::abi::StableLike::new(
                                        Some(
                                            &stabby::abi::report::FieldReport {
                                                name: stabby::abi::str::Str::new("debug_fmt"),
                                                ty: <stabby::abi::StableLike<
                                                    for<'a> extern "C" fn(
                                                        stabby::abi::AnonymRef<'a>,
                                                        ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                                                    ) -> Option<FmtOutput<'a>>,
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
                                    "StabbyVtableObjectV0\'s size was mis-evaluated by stabby, this is definitely a bug and may cause UB, please file an issue",
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
                                    "StabbyVtableObjectV0\'s align was mis-evaluated by stabby, this is definitely a bug and may cause UB, please file an issue",
                                ),
                            );
                        }
                    }
                    stabby::abi::report::gen_id(Self::REPORT)
                };
            }
            #[allow(dead_code, missing_docs)]
            struct OptimizedLayoutForStabbyVtableObjectV0<'stabby_vt_lt> {
                ///An stabby-generated item for [`ObjectV0`]
                pub debug_fmt: stabby::abi::StableLike<
                    for<'a> extern "C" fn(
                        stabby::abi::AnonymRef<'a>,
                        ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                    ) -> Option<FmtOutput<'a>>,
                    &'static (),
                >,
                ///An stabby-generated item for [`ObjectV0`]
                pub display_fmt: stabby::abi::StableLike<
                    for<'a> extern "C" fn(
                        stabby::abi::AnonymRef<'a>,
                        ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                    ) -> Option<FmtOutput<'a>>,
                    &'static (),
                >,
            }
            impl<'stabby_vt_lt> StabbyVtableObjectV0<'stabby_vt_lt>
            where
                stabby::abi::Struct<
                    stabby::abi::FieldPair<
                        stabby::abi::StableLike<
                            for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            ) -> Option<FmtOutput<'a>>,
                            &'static (),
                        >,
                        stabby::abi::StableLike<
                            for<'a> extern "C" fn(
                                stabby::abi::AnonymRef<'a>,
                                ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                            ) -> Option<FmtOutput<'a>>,
                            &'static (),
                        >,
                    >,
                >: stabby::abi::IStable,
                stabby::abi::StableLike<
                    for<'a> extern "C" fn(
                        stabby::abi::AnonymRef<'a>,
                        ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                    ) -> Option<FmtOutput<'a>>,
                    &'static (),
                >: stabby::abi::IStable,
            {
                ///Returns true if the layout for [`StabbyVtableObjectV0`] is smaller or equal to that Rust would have generated for it.
                pub const fn has_optimal_layout() -> bool {
                    core::mem::size_of::<Self>()
                        <= core::mem::size_of::<
                            OptimizedLayoutForStabbyVtableObjectV0<'stabby_vt_lt>,
                        >()
                }
            }
            impl<'stabby_vt_lt> Clone for StabbyVtableObjectV0<'stabby_vt_lt> {
                fn clone(&self) -> Self {
                    *self
                }
            }
            #[allow(unknown_lints)]
            #[allow(clippy::multiple_bound_locations)]
            impl<'stabby_vt_lt> Copy for StabbyVtableObjectV0<'stabby_vt_lt> {}
            #[allow(unknown_lints)]
            #[allow(clippy::multiple_bound_locations)]
            impl<'stabby_vt_lt> core::cmp::PartialEq
            for StabbyVtableObjectV0<'stabby_vt_lt> {
                fn eq(&self, other: &Self) -> bool {
                    core::ptr::eq(
                        (*unsafe { self.debug_fmt.as_ref_unchecked() }) as *const (),
                        (*unsafe { other.debug_fmt.as_ref_unchecked() }) as *const _,
                    )
                        && core::ptr::eq(
                            (*unsafe { self.display_fmt.as_ref_unchecked() })
                                as *const (),
                            (*unsafe { other.display_fmt.as_ref_unchecked() })
                                as *const _,
                        ) && true
                }
            }
            #[allow(unknown_lints)]
            #[allow(clippy::multiple_bound_locations)]
            impl<'stabby_vt_lt> core::hash::Hash
            for StabbyVtableObjectV0<'stabby_vt_lt> {
                fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                    self.debug_fmt.hash(state);
                    self.display_fmt.hash(state);
                }
            }
            #[allow(unknown_lints)]
            #[allow(clippy::multiple_bound_locations)]
            impl<'stabby_vt_lt> core::fmt::Debug
            for StabbyVtableObjectV0<'stabby_vt_lt> {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    let mut s = f.debug_struct("StabbyVtableObjectV0");
                    s.field(
                        "debug_fmt",
                        &format_args!(
                            "{0:p}",
                            unsafe { self.debug_fmt.as_ref_unchecked() },
                        ),
                    );
                    s.field(
                        "display_fmt",
                        &format_args!(
                            "{0:p}",
                            unsafe { self.display_fmt.as_ref_unchecked() },
                        ),
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
            for StabbyVtableObjectV0<'stabby_vt_lt>
            where
                StabbyArbitraryType: ObjectV0,
            {
                const VTABLE: StabbyVtableObjectV0<'stabby_vt_lt> = StabbyVtableObjectV0 {
                    debug_fmt: unsafe {
                        stabby::abi::StableLike::new({
                            extern "C" fn ext_debug_fmt<
                                'stabby_local_lt,
                                'a,
                                StabbyArbitraryType: 'stabby_local_lt,
                            >(
                                this: stabby::abi::AnonymRef<'a>,
                                _lt_proof: ::core::marker::PhantomData<
                                    &'a &'stabby_local_lt (),
                                >,
                            ) -> Option<FmtOutput<'a>>
                            where
                                StabbyArbitraryType: ObjectV0,
                            {
                                unsafe {
                                    <StabbyArbitraryType as ObjectV0>::debug_fmt(
                                        this.cast::<StabbyArbitraryType>().as_ref(),
                                    )
                                }
                            }
                            ext_debug_fmt::<StabbyArbitraryType>
                                as for<'a> extern "C" fn(
                                    stabby::abi::AnonymRef<'a>,
                                    ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                                ) -> Option<FmtOutput<'a>>
                        })
                    },
                    display_fmt: unsafe {
                        stabby::abi::StableLike::new({
                            extern "C" fn ext_display_fmt<
                                'stabby_local_lt,
                                'a,
                                StabbyArbitraryType: 'stabby_local_lt,
                            >(
                                this: stabby::abi::AnonymRef<'a>,
                                _lt_proof: ::core::marker::PhantomData<
                                    &'a &'stabby_local_lt (),
                                >,
                            ) -> Option<FmtOutput<'a>>
                            where
                                StabbyArbitraryType: ObjectV0,
                            {
                                unsafe {
                                    <StabbyArbitraryType as ObjectV0>::display_fmt(
                                        this.cast::<StabbyArbitraryType>().as_ref(),
                                    )
                                }
                            }
                            ext_display_fmt::<StabbyArbitraryType>
                                as for<'a> extern "C" fn(
                                    stabby::abi::AnonymRef<'a>,
                                    ::core::marker::PhantomData<&'a &'stabby_vt_lt ()>,
                                ) -> Option<FmtOutput<'a>>
                        })
                    },
                };
            }
            #[allow(unknown_lints)]
            #[allow(clippy::multiple_bound_locations)]
            impl<'stabby_vt_lt> stabby::abi::vtable::CompoundVt<'stabby_vt_lt>
            for dyn ObjectV0 {
                ///An stabby-generated item for [`ObjectV0`]
                type Vt<StabbyNextVtable> = stabby::abi::vtable::VTable<
                    StabbyVtableObjectV0<'stabby_vt_lt>,
                    StabbyNextVtable,
                >;
            }
            #[allow(unknown_lints)]
            #[allow(clippy::multiple_bound_locations)]
            ///An stabby-generated item for [`ObjectV0`]
            pub trait ObjectV0Dyn<StabbyTransitiveDerefN> {
                ///An stabby-generated item for [`ObjectV0`]
                extern "C" fn debug_fmt<'a>(&'a self) -> Option<FmtOutput<'a>>;
                ///An stabby-generated item for [`ObjectV0`]
                extern "C" fn display_fmt<'a>(&'a self) -> Option<FmtOutput<'a>>;
            }
            #[allow(unknown_lints)]
            #[allow(clippy::multiple_bound_locations)]
            impl<
                'stabby_vt_lt,
                StabbyVtProvider: stabby::abi::vtable::TransitiveDeref<
                        StabbyVtableObjectV0<'stabby_vt_lt>,
                        StabbyTransitiveDerefN,
                    > + Copy,
                StabbyTransitiveDerefN,
            > ObjectV0Dyn<StabbyTransitiveDerefN>
            for stabby::abi::DynRef<'_, StabbyVtProvider> {
                ///An stabby-generated item for [`ObjectV0`]
                extern "C" fn debug_fmt<'a>(&'a self) -> Option<FmtOutput<'a>> {
                    unsafe {
                        (self
                            .vtable()
                            .tderef()
                            .debug_fmt
                            .as_ref_unchecked())(self.ptr(), ::core::marker::PhantomData)
                    }
                }
                ///An stabby-generated item for [`ObjectV0`]
                extern "C" fn display_fmt<'a>(&'a self) -> Option<FmtOutput<'a>> {
                    unsafe {
                        (self
                            .vtable()
                            .tderef()
                            .display_fmt
                            .as_ref_unchecked())(self.ptr(), ::core::marker::PhantomData)
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
                        StabbyVtableObjectV0<'stabby_vt_lt>,
                        StabbyTransitiveDerefN,
                    >,
                StabbyTransitiveDerefN,
            > ObjectV0Dyn<StabbyTransitiveDerefN>
            for stabby::abi::Dyn<'_, StabbyPtrProvider, StabbyVtProvider> {
                ///An stabby-generated item for [`ObjectV0`]
                extern "C" fn debug_fmt<'a>(&'a self) -> Option<FmtOutput<'a>> {
                    unsafe {
                        (self
                            .vtable()
                            .tderef()
                            .debug_fmt
                            .as_ref_unchecked())(
                            self.ptr().as_ref(),
                            ::core::marker::PhantomData,
                        )
                    }
                }
                ///An stabby-generated item for [`ObjectV0`]
                extern "C" fn display_fmt<'a>(&'a self) -> Option<FmtOutput<'a>> {
                    unsafe {
                        (self
                            .vtable()
                            .tderef()
                            .display_fmt
                            .as_ref_unchecked())(
                            self.ptr().as_ref(),
                            ::core::marker::PhantomData,
                        )
                    }
                }
            }
            #[allow(unknown_lints)]
            #[allow(clippy::multiple_bound_locations)]
            ///An stabby-generated item for [`ObjectV0`]
            pub trait ObjectV0DynMut<
                StabbyTransitiveDerefN,
            >: ObjectV0Dyn<StabbyTransitiveDerefN> {}
            #[allow(unknown_lints)]
            #[allow(clippy::multiple_bound_locations)]
            impl<
                'stabby_vt_lt,
                StabbyPtrProvider,
                StabbyVtProvider,
                StabbyTransitiveDerefN,
            > ObjectV0DynMut<StabbyTransitiveDerefN>
            for stabby::abi::Dyn<'_, StabbyPtrProvider, StabbyVtProvider>
            where
                StabbyPtrProvider: stabby::abi::IPtrOwned + stabby::abi::IPtrMut,
                StabbyVtProvider: stabby::abi::vtable::HasDropVt + Copy
                    + stabby::abi::vtable::TransitiveDeref<
                        StabbyVtableObjectV0<'stabby_vt_lt>,
                        StabbyTransitiveDerefN,
                    >,
            {}
            pub struct VariableDataV0(
                stabby::abi::Result<
                    Arc<
                        stabby::abi::Dyn<
                            'static,
                            RBox<()>,
                            <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                'static,
                            >>::Vt<stabby::abi::vtable::VtDrop>,
                        >,
                    >,
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >,
            )
            where
                stabby::abi::Dyn<
                    'static,
                    RBox<()>,
                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                        'static,
                    >>::Vt<stabby::abi::vtable::VtDrop>,
                >: stabby::abi::IStable,
                Arc<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >: stabby::abi::IStable,
                Arc<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >: stabby::abi::IDeterminantProvider<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >,
                stabby::abi::Dyn<
                    'static,
                    RBox<()>,
                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                        'static,
                    >>::Vt<stabby::abi::vtable::VtDrop>,
                >: stabby::abi::IStable;
            #[allow(dead_code)]
            #[repr(u8)]
            enum ReprCLayoutForVariableDataV0
            where
                Arc<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >: stabby::abi::IDeterminantProvider<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >,
                stabby::abi::Dyn<
                    'static,
                    RBox<()>,
                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                        'static,
                    >>::Vt<stabby::abi::vtable::VtDrop>,
                >: stabby::abi::IStable,
            {
                Arc(
                    Arc<
                        stabby::abi::Dyn<
                            'static,
                            RBox<()>,
                            <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                'static,
                            >>::Vt<stabby::abi::vtable::VtDrop>,
                        >,
                    >,
                ),
                Object(
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                ),
            }
            impl VariableDataV0
            where
                stabby::abi::Dyn<
                    'static,
                    RBox<()>,
                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                        'static,
                    >>::Vt<stabby::abi::vtable::VtDrop>,
                >: stabby::abi::IStable,
                Arc<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >: stabby::abi::IStable,
                Arc<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >: stabby::abi::IDeterminantProvider<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >,
                stabby::abi::Dyn<
                    'static,
                    RBox<()>,
                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                        'static,
                    >>::Vt<stabby::abi::vtable::VtDrop>,
                >: stabby::abi::IStable,
                stabby::abi::Result<
                    Arc<
                        stabby::abi::Dyn<
                            'static,
                            RBox<()>,
                            <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                'static,
                            >>::Vt<stabby::abi::vtable::VtDrop>,
                        >,
                    >,
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >: stabby::abi::IStable,
            {
                ///Returns true if the layout for [`VariableDataV0`] is smaller than what `#[repr(C)]` would have generated for it.
                pub const fn has_optimal_layout() -> bool {
                    core::mem::size_of::<Self>()
                        < core::mem::size_of::<ReprCLayoutForVariableDataV0>()
                }
            }
            const _: () = {
                if !<VariableDataV0>::has_optimal_layout() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "VariableDataV0\'s layout is sub-optimal, reorder fields or use `#[repr(stabby)]` to silence this error.",
                            ),
                        );
                    }
                }
                if core::mem::size_of::<VariableDataV0>()
                    != <<VariableDataV0 as stabby::abi::IStable>::Size as stabby::abi::Unsigned>::USIZE
                {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "VariableDataV0\'s size was mis-evaluated by stabby, this is definitely a bug and may cause UB, please fill an issue",
                            ),
                        );
                    }
                }
                if core::mem::align_of::<VariableDataV0>()
                    != <<VariableDataV0 as stabby::abi::IStable>::Align as stabby::abi::Unsigned>::USIZE
                {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "VariableDataV0\'s align was mis-evaluated by stabby, this is definitely a bug and may cause UB, please fill an issue",
                            ),
                        );
                    }
                }
            };
            #[automatically_derived]
            unsafe impl stabby::abi::IStable for VariableDataV0
            where
                stabby::abi::Dyn<
                    'static,
                    RBox<()>,
                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                        'static,
                    >>::Vt<stabby::abi::vtable::VtDrop>,
                >: stabby::abi::IStable,
                Arc<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >: stabby::abi::IStable,
                Arc<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >: stabby::abi::IDeterminantProvider<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >,
                stabby::abi::Dyn<
                    'static,
                    RBox<()>,
                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                        'static,
                    >>::Vt<stabby::abi::vtable::VtDrop>,
                >: stabby::abi::IStable,
                stabby::abi::Result<
                    Arc<
                        stabby::abi::Dyn<
                            'static,
                            RBox<()>,
                            <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                'static,
                            >>::Vt<stabby::abi::vtable::VtDrop>,
                        >,
                    >,
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >: stabby::abi::IStable,
            {
                type ForbiddenValues = <stabby::abi::Result<
                    Arc<
                        stabby::abi::Dyn<
                            'static,
                            RBox<()>,
                            <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                'static,
                            >>::Vt<stabby::abi::vtable::VtDrop>,
                        >,
                    >,
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                > as stabby::abi::IStable>::ForbiddenValues;
                type UnusedBits = <stabby::abi::Result<
                    Arc<
                        stabby::abi::Dyn<
                            'static,
                            RBox<()>,
                            <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                'static,
                            >>::Vt<stabby::abi::vtable::VtDrop>,
                        >,
                    >,
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                > as stabby::abi::IStable>::UnusedBits;
                type Size = <stabby::abi::Result<
                    Arc<
                        stabby::abi::Dyn<
                            'static,
                            RBox<()>,
                            <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                'static,
                            >>::Vt<stabby::abi::vtable::VtDrop>,
                        >,
                    >,
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                > as stabby::abi::IStable>::Size;
                type Align = <stabby::abi::Result<
                    Arc<
                        stabby::abi::Dyn<
                            'static,
                            RBox<()>,
                            <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                'static,
                            >>::Vt<stabby::abi::vtable::VtDrop>,
                        >,
                    >,
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                > as stabby::abi::IStable>::Align;
                type HasExactlyOneNiche = stabby::abi::B0;
                type ContainsIndirections = <stabby::abi::Result<
                    Arc<
                        stabby::abi::Dyn<
                            'static,
                            RBox<()>,
                            <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                'static,
                            >>::Vt<stabby::abi::vtable::VtDrop>,
                        >,
                    >,
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                > as stabby::abi::IStable>::ContainsIndirections;
                const REPORT: &'static stabby::abi::report::TypeReport = &stabby::abi::report::TypeReport {
                    name: stabby::abi::str::Str::new("VariableDataV0"),
                    module: stabby::abi::str::Str::new(
                        "lrtexec_lib::commands::v0::structs",
                    ),
                    fields: unsafe {
                        stabby::abi::StableLike::new(
                            Some(
                                &stabby::abi::report::FieldReport {
                                    name: stabby::abi::str::Str::new("Object"),
                                    ty: <stabby::abi::Dyn<
                                        'static,
                                        RBox<()>,
                                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                            'static,
                                        >>::Vt<stabby::abi::vtable::VtDrop>,
                                    > as stabby::abi::IStable>::REPORT,
                                    next_field: stabby::abi::StableLike::new(
                                        Some(
                                            &stabby::abi::report::FieldReport {
                                                name: stabby::abi::str::Str::new("Arc"),
                                                ty: <Arc<
                                                    stabby::abi::Dyn<
                                                        'static,
                                                        RBox<()>,
                                                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                                            'static,
                                                        >>::Vt<stabby::abi::vtable::VtDrop>,
                                                    >,
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
                    tyty: stabby::abi::report::TyTy::Enum(
                        stabby::abi::str::Str::new("stabby"),
                    ),
                };
                const ID: u64 = stabby::abi::report::gen_id(Self::REPORT);
            }
            #[automatically_derived]
            impl VariableDataV0
            where
                stabby::abi::Dyn<
                    'static,
                    RBox<()>,
                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                        'static,
                    >>::Vt<stabby::abi::vtable::VtDrop>,
                >: stabby::abi::IStable,
                Arc<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >: stabby::abi::IStable,
                Arc<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >: stabby::abi::IDeterminantProvider<
                    stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                >,
                stabby::abi::Dyn<
                    'static,
                    RBox<()>,
                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                        'static,
                    >>::Vt<stabby::abi::vtable::VtDrop>,
                >: stabby::abi::IStable,
            {
                #[allow(non_snake_case)]
                pub fn Arc(
                    value: Arc<
                        stabby::abi::Dyn<
                            'static,
                            RBox<()>,
                            <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                'static,
                            >>::Vt<stabby::abi::vtable::VtDrop>,
                        >,
                    >,
                ) -> Self {
                    Self(stabby::abi::Result::Ok(value))
                }
                #[allow(non_snake_case)]
                pub fn Object(
                    value: stabby::abi::Dyn<
                        'static,
                        RBox<()>,
                        <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                            'static,
                        >>::Vt<stabby::abi::vtable::VtDrop>,
                    >,
                ) -> Self {
                    Self(stabby::abi::Result::Err(value))
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match self`.
                pub fn match_owned<
                    StabbyOut,
                    ArcFn: FnOnce(
                            Arc<
                                stabby::abi::Dyn<
                                    'static,
                                    RBox<()>,
                                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                        'static,
                                    >>::Vt<stabby::abi::vtable::VtDrop>,
                                >,
                            >,
                        ) -> StabbyOut,
                    ObjectFn: FnOnce(
                            stabby::abi::Dyn<
                                'static,
                                RBox<()>,
                                <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                    'static,
                                >>::Vt<stabby::abi::vtable::VtDrop>,
                            >,
                        ) -> StabbyOut,
                >(self, Arc: ArcFn, Object: ObjectFn) -> StabbyOut {
                    self.0.match_owned(Arc, Object)
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match &self`.
                pub fn match_ref<
                    'st_lt,
                    StabbyOut,
                    ArcFn: FnOnce(
                            &'st_lt Arc<
                                stabby::abi::Dyn<
                                    'static,
                                    RBox<()>,
                                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                        'static,
                                    >>::Vt<stabby::abi::vtable::VtDrop>,
                                >,
                            >,
                        ) -> StabbyOut,
                    ObjectFn: FnOnce(
                            &'st_lt stabby::abi::Dyn<
                                'static,
                                RBox<()>,
                                <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                    'static,
                                >>::Vt<stabby::abi::vtable::VtDrop>,
                            >,
                        ) -> StabbyOut,
                >(&'st_lt self, Arc: ArcFn, Object: ObjectFn) -> StabbyOut {
                    self.0.match_ref(Arc, Object)
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match &mut self`.
                pub fn match_mut<
                    'st_lt,
                    StabbyOut,
                    ArcFn: FnOnce(
                            &mut Arc<
                                stabby::abi::Dyn<
                                    'static,
                                    RBox<()>,
                                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                        'static,
                                    >>::Vt<stabby::abi::vtable::VtDrop>,
                                >,
                            >,
                        ) -> StabbyOut,
                    ObjectFn: FnOnce(
                            &mut stabby::abi::Dyn<
                                'static,
                                RBox<()>,
                                <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                    'static,
                                >>::Vt<stabby::abi::vtable::VtDrop>,
                            >,
                        ) -> StabbyOut,
                >(&'st_lt mut self, Arc: ArcFn, Object: ObjectFn) -> StabbyOut {
                    self.0.match_mut(|mut a| Arc(&mut *a), |mut a| Object(&mut *a))
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match self`, but allows you to pass common arguments to all closures to make the borrow checker happy.
                pub fn match_owned_ctx<
                    StabbyOut,
                    StabbyCtx,
                    ArcFn: FnOnce(
                            StabbyCtx,
                            Arc<
                                stabby::abi::Dyn<
                                    'static,
                                    RBox<()>,
                                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                        'static,
                                    >>::Vt<stabby::abi::vtable::VtDrop>,
                                >,
                            >,
                        ) -> StabbyOut,
                    ObjectFn: FnOnce(
                            StabbyCtx,
                            stabby::abi::Dyn<
                                'static,
                                RBox<()>,
                                <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                    'static,
                                >>::Vt<stabby::abi::vtable::VtDrop>,
                            >,
                        ) -> StabbyOut,
                >(
                    self,
                    stabby_ctx: StabbyCtx,
                    Arc: ArcFn,
                    Object: ObjectFn,
                ) -> StabbyOut {
                    self.0.match_owned_ctx(stabby_ctx, Arc, Object)
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match &self`, but allows you to pass common arguments to all closures to make the borrow checker happy.
                pub fn match_ref_ctx<
                    'st_lt,
                    StabbyCtx,
                    StabbyOut,
                    ArcFn: FnOnce(
                            StabbyCtx,
                            &'st_lt Arc<
                                stabby::abi::Dyn<
                                    'static,
                                    RBox<()>,
                                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                        'static,
                                    >>::Vt<stabby::abi::vtable::VtDrop>,
                                >,
                            >,
                        ) -> StabbyOut,
                    ObjectFn: FnOnce(
                            StabbyCtx,
                            &'st_lt stabby::abi::Dyn<
                                'static,
                                RBox<()>,
                                <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                    'static,
                                >>::Vt<stabby::abi::vtable::VtDrop>,
                            >,
                        ) -> StabbyOut,
                >(
                    &'st_lt self,
                    stabby_ctx: StabbyCtx,
                    Arc: ArcFn,
                    Object: ObjectFn,
                ) -> StabbyOut {
                    self.0.match_ref_ctx(stabby_ctx, Arc, Object)
                }
                #[allow(non_snake_case)]
                /// Equivalent to `match &mut self`, but allows you to pass common arguments to all closures to make the borrow checker happy.
                pub fn match_mut_ctx<
                    'st_lt,
                    StabbyCtx,
                    StabbyOut,
                    ArcFn: FnOnce(
                            StabbyCtx,
                            &mut Arc<
                                stabby::abi::Dyn<
                                    'static,
                                    RBox<()>,
                                    <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                        'static,
                                    >>::Vt<stabby::abi::vtable::VtDrop>,
                                >,
                            >,
                        ) -> StabbyOut,
                    ObjectFn: FnOnce(
                            StabbyCtx,
                            &mut stabby::abi::Dyn<
                                'static,
                                RBox<()>,
                                <dyn ObjectV0 as stabby::abi::vtable::CompoundVt<
                                    'static,
                                >>::Vt<stabby::abi::vtable::VtDrop>,
                            >,
                        ) -> StabbyOut,
                >(
                    &'st_lt mut self,
                    stabby_ctx: StabbyCtx,
                    Arc: ArcFn,
                    Object: ObjectFn,
                ) -> StabbyOut {
                    self.0
                        .match_mut_ctx(
                            stabby_ctx,
                            |stabby_ctx, mut a| Arc(stabby_ctx, &mut *a),
                            |stabby_ctx, mut a| Object(stabby_ctx, &mut *a),
                        )
                }
            }
            pub extern "C" fn create_v0<T: ObjectV0>(data: T) -> VariableDataV0 {
                let _ = stabby::abi::AssertStable::<
                    VariableDataV0,
                >(::core::marker::PhantomData);
                let _ = stabby::abi::AssertStable::<T>(::core::marker::PhantomData);
                { VariableDataV0::Object(RBox::new(data)) }
            }
            impl<T: Debug> ObjectV0 for T {
                extern "C" fn debug_fmt<'a>(&'a self) -> Option<FmtOutput<'a>> {
                    Some(
                        FmtOutput::String(
                            ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(format_args!("{0:?}", self));
                                    res
                                })
                                .into_boxed_str(),
                        ),
                    )
                }
                extern "C" fn display_fmt<'a>(&'a self) -> Option<FmtOutput<'a>> {
                    None
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
