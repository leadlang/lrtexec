#[macro_export]
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
