use lrt_macros::declare;

pub mod compat;
pub mod structs;
pub struct ScriptV0 {}

pub type Script = ScriptV0;

declare! {
  {
    extend,
    set,
    loadfromreg,
    regset,
    regload,
    dlopen,
    drop,
  },
  {
    hello
  }
}
