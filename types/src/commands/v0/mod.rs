use lrt_macros::declare;

pub mod compat;
pub mod structs;

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
    regload,
    dlopen,
    drop,
  },
  {
    hi
  }
}
