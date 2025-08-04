use crate::commands::FFISafeContainer;

#[repr(C)]
pub enum PollResult {
  Continue,
  Break,
}

#[deprecated(
    note = "This API is unstable and subject to significant, breaking changes in future versions. Use at your own risk. To silence this warning, use `#[allow(deprecated)]`."
)]
#[repr(C)]
pub struct UnstableAsyncInterface<T: FFISafeContainer> {
  pub data: T,
  pub poll: extern "C" fn() -> PollResult
}