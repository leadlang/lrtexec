#![feature(trivial_bounds)]
// use common::hashmap::RTVariableMap;

pub mod commands;
pub mod common;

// pub const VERSION: u16 = 0;

// pub type Maybe<T> = stabby::option::Option<T>;

// #[stabby::stabby]
// pub struct FnStack {
//   /// Return value (identifier in MemoryMap)
//   pub ret: Maybe<VariableData>,
//   // General Purpose (identifiers in MemoryMap)
//   pub itself: Option<NonZeroU16>,
//   pub r1: Option<NonZeroU16>,
//   pub r2: Option<NonZeroU16>,
//   pub r3: Option<NonZeroU16>,
//   pub r4: Option<NonZeroU16>,
//   pub r5: Option<NonZeroU16>,
//   pub r6: Option<NonZeroU16>,
//   pub r7: Option<NonZeroU16>,
//   pub r8: Option<NonZeroU16>,
// }

// impl FnStack {
//   #[unsafe(no_mangle)]
//   pub extern "C" fn create() -> Self {
//     Self {
//       ret: None.into(),
//       itself: None,
//       r1: None,
//       r2: None,
//       r3: None,
//       r4: None,
//       r5: None,
//       r6: None,
//       r7: None,
//       r8: None,
//     }
//   }

//   pub fn clear(&mut self) {
//     self.itself = None;
//     self.r1 = None;
//     self.r2 = None;
//     self.r3 = None;
//     self.r4 = None;
//     self.r5 = None;
//     self.r6 = None;
//     self.r7 = None;
//     self.r8 = None;
//   }

//   #[unsafe(no_mangle)]
//   pub extern "C" fn setVariableOutput(&mut self, data: VariableData) {
//     self.ret = Some(data).into();
//   }
// }

// #[stabby::stabby]
// #[derive(Debug, Clone)]
// pub struct Wrapper<T: Copy + Clone>(T);

// impl<T: Copy + Clone> Wrapper<T> {
//   pub fn inner(&self) -> T {
//     self.0
//   }
// }

// pub struct MemoryMap {
//   variables: RTVariableMap,
// }

// impl MemoryMap {
//   #[cfg(feature = "memory")]
//   pub fn create_map() -> Self {
//     Self {
//       variables: HashMap::new(),
//     }
//   }
// }

// pub type AnyData = dynptr!(RBox<dyn Any + 'static>);

// #[stabby::stabby]
// #[repr(stabby)]
// pub enum FmtOut {
//   Owned(BoxedStr),
//   Slice(Str<'static>)
// }

// #[stabby::stabby]
// pub trait Object {
//   extern "C" fn fmt(&self) -> FmtOut;
// }

// #[stabby::stabby]
// pub enum VariableData {
//   CodePtr(Slice<'static, u8>),
//   Obj(dynptr!(RBox<dyn Object + 'static>)),
// }
