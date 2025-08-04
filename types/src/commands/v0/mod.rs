use lrt_macros::declare;

pub mod compat;
pub mod structs;

pub use super::FFISafeContainer;

// Declare the assembly syntax of lrtexec
// This function will generate the bytecode and the compiler; interpreter implementations
declare! {
  v0;
  {
    set,
    drop,
    regload,
    regdrop,

    set,
    loadfromreg,
    regset,
    dlopen,
    drop,
  },
  {
    hi
  }
}
