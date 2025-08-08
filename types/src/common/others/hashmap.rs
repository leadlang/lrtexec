use crate::{common::{others::FFISafeString, FFIableObject}, Maybe, Ref};
use std::{env, mem::transmute, path::PathBuf, str::FromStr, sync::LazyLock};
use libloading::{Library, Symbol, library_filename};

macro_rules! create {
  (
    defines $name:ident
    loads $lib:literal

    extern C {
      $(
        $(doc = $doc:literal)?
        fn $fname:ident -> $ret:ty { $($arg:ident: $typ:ty),* }
      )*
    }
  ) => {
    pastey::paste! {
      pub static [< $name:upper >]: LazyLock<[< $name:camel >]> = LazyLock::new(|| {
        [< $name:camel >]::new()
      });

      $(
        pub type [< Call $fname:camel Fn >] = unsafe extern "C" fn(
          $(
            $arg: $typ
          ),*
        ) -> $ret;
      )*

      pub struct [< $name:camel >] {
        _lib: Library,
        $(
          $(
            #[doc = $doc]
          )?
          pub $fname: [< Call $fname:camel Fn >]
        ),*
      }

      impl [< $name:camel >] {
        pub fn new() -> Self {
          let lrt = env::var("LRT_HOME").expect("LRT Home not present");

          let file = library_filename($lib);

          let mut path = PathBuf::from_str(&lrt).unwrap();

          path.push("libs");
          path.push("waker");
          path.push(file);

          let lib = unsafe { Library::new(path).expect("Unable to load async_waker") }; 

          $(
            let $fname = {
              let ptr: Symbol<
                '_, 
                [< Call $fname:camel Fn >]
              > = unsafe { lib.get(stringify!($fname).as_bytes()).unwrap() };

              let out: Symbol<'static, [< Call $fname:camel Fn >]> = unsafe{ transmute(ptr) };

              out
            };
          )*

          Self {
            _lib: lib,
            $(
              $fname: *$fname
            ),*
          }
        }
      }
    }
  };
}

create! {
  defines FfiHashMap
  loads "hashmap"

  extern C {
    doc = "Creates a new HashMap"
    fn create -> Ref { use_str_map: bool }
    fn insert_int -> Maybe<FFIableObject> { map: *const Ref, k: usize, v: FFIableObject }
    fn get_int -> Maybe<*const FFIableObject> { map: *const Ref, k: *const usize }
    fn remove_int -> Maybe<FFIableObject> { map: *const Ref, k: *const usize }
    fn insert_str -> Maybe<FFIableObject> { map: *const Ref, k: FFISafeString, v: FFIableObject }
    fn get_str -> Maybe<*const FFIableObject> { map: *const Ref, k: *const FFISafeString }
    fn remove_str -> Maybe<FFIableObject> { map: *const Ref, k: *const FFISafeString }
  }
}