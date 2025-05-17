#[macro_export]
macro_rules! generate {
  (
    $struct:ty;
    generate: $($x:ty),*
  ) => {
    impl $struct {
      paste! {
        $(
          #[unsafe(no_mangle)]
          pub extern "C" fn [<from_c_to_rust $x>](value: Wrapper<$x>) -> Self {
            RustVariable::new(value).to_c()
          }
        )*
      }
    }

    $(
      impl From<Wrapper<$x>> for $struct {
        fn from(value: Wrapper<$x>) -> Self {
          RustVariable::new(value).to_c()
        }
      }
    )*
  };
}