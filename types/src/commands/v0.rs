use lrt_macros::declare;

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
