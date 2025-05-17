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
          pub extern "C" fn [<into_common_ $x>](value: Wrapper<$x>) -> Self {
            RustVariable::new(value).to_c()
          }

          /// This creates a Clone of the inner value
          #[unsafe(no_mangle)]
          pub extern "C" fn [<get_common_ $x>](self) -> $x {
            let data: &RustVariable<Wrapper<$x>> = unsafe { RustVariable::from_ptr(self.data) };

            data.inner()
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