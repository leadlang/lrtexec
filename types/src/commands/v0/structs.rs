//! Structs related to v0 commands

use std::{fmt::Debug, num::NonZeroU16};

use stabby::{boxed::{Box as RBox, BoxedStr}, dynptr, stabby, str::Str, sync::Arc};

#[stabby]
pub struct FnStackV0 {
  /// Return value (identifier in MemoryMap)
  pub ret: Option<VariableDataV0>,
  // General Purpose (identifiers in MemoryMap)
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

#[stabby]
pub enum FmtOutput<'a> {
  String(BoxedStr),
  Str(Str<'a>)
}


#[stabby]
pub trait ObjectV0 {
  extern "C" fn debug_fmt<'a>(&'a self) -> Option<FmtOutput<'a>>;
  extern "C" fn display_fmt<'a>(&'a self) -> Option<FmtOutput<'a>>;
}

#[stabby]
pub enum VariableDataV0 {
  Arc(Arc<dynptr!(RBox<dyn ObjectV0 + 'static>)>),
  Object(dynptr!(RBox<dyn ObjectV0 + 'static>))
}

#[stabby]
pub fn create_v0<T: ObjectV0>(data: T) -> VariableDataV0 {
  VariableDataV0::Object(RBox::new(data))
}

impl<T: Debug> ObjectV0 for T {
  extern "C" fn debug_fmt<'a>(&'a self) -> Option<FmtOutput<'a> > {
    Some(FmtOutput::String(format!("{:?}", self).into_boxed_str()))
  }

  extern "C" fn display_fmt<'a>(&'a self) -> Option<FmtOutput<'a> > {
    None
  }
}