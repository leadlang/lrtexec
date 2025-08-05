#[cfg(feature = "waker")]
use std::os::raw::c_void;
use std::{
  fmt::Debug, future::Future, ops::{Deref, DerefMut}, pin::Pin, task::{Context, Poll}
};

use tokio::task::{JoinHandle, spawn_blocking};

use crate::{commands::FFISafeContainer, common::FFIableObject};

#[cfg(feature = "waker")]
mod waker;

#[cfg(feature = "waker")]
pub use waker::{WAKER, call_waker_consume_ptr};

#[repr(C)]
#[allow(deprecated)]
pub enum AsyncInterface<T: FFISafeContainer + 'static> {
  Threaded(ThreadedTask<T>),
  Lazy(LazyableTask<T>),
  #[cfg(feature = "waker")]
  LazyWithWaker(LazyableTaskWithWaker<T>)
}

impl<T: FFISafeContainer + 'static> Future for AsyncInterface<T> {
  type Output = SafeWrapped<T>;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    match self.as_mut().get_mut() {
      AsyncInterface::Lazy(lazy) => Pin::new(lazy).poll(cx).map(|x| SafeWrapped(x)),
      AsyncInterface::Threaded(threaded) => Pin::new(threaded).poll(cx),
      #[cfg(feature = "waker")]
      AsyncInterface::LazyWithWaker(lazy) => Pin::new(lazy).poll(cx).map(|x| SafeWrapped(x)),
    }
  }
}

#[repr(C)]
pub struct ThreadedTask<T: FFISafeContainer + 'static> {
  pub computation: extern "C" fn() -> T,

  handle: Option<JoinHandle<SafeWrapped<T>>>,
}

impl<T: FFISafeContainer + 'static> ThreadedTask<T> {
  pub fn new(task: extern "C" fn() -> T) -> Self {
    Self {
      computation: task,
      handle: None
    }
  }

  fn start_once(&mut self) {
    if self.handle.is_none() {
      let computation = UnsafeWrapped(self.computation);
      let handle = spawn_blocking(move || {
        let computation_fn_ptr = computation.0;
        SafeWrapped((computation_fn_ptr)())
      });
      self.handle = Some(handle);
    }
  }
}

impl<T: FFISafeContainer + 'static> Future for ThreadedTask<T> {
  type Output = SafeWrapped<T>;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    self.start_once();

    let handle = Pin::new(self.as_mut().get_mut().handle.as_mut().unwrap());
    handle
      .poll(cx)
      .map(|result| result.expect("Threaded task panicked during execution"))
  }
}

pub(crate) struct UnsafeWrapped<T: 'static>(T);
unsafe impl<T> Send for UnsafeWrapped<T> {}

pub struct SafeWrapped<T: 'static + FFISafeContainer>(T);
unsafe impl<T: FFISafeContainer> Send for SafeWrapped<T> {}

impl<T: Debug + FFISafeContainer + 'static> Debug for SafeWrapped<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.0.fmt(f)
  }
}

impl<T: 'static + FFISafeContainer> SafeWrapped<T> {
  pub fn get_pure(self) -> T {
    self.0
  }
}

impl<T: 'static + FFISafeContainer> Deref for SafeWrapped<T> {
  type Target = T;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T: 'static + FFISafeContainer> DerefMut for SafeWrapped<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

#[repr(C)]
pub struct LazyableTask<T: FFISafeContainer> {
  pub state: FFIableObject,
  pub poll: extern "C" fn(state: *mut FFIableObject) -> PollResult<T>,
}

#[repr(C)]
pub enum PollResult<T: FFISafeContainer> {
  Ready(T),
  Poll,
}

unsafe impl<T: FFISafeContainer> Send for LazyableTask<T> {}

impl<T: FFISafeContainer + 'static> Future for LazyableTask<T> {
  type Output = T;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let data = (self.poll)(&mut self.state);
    match data {
      PollResult::Ready(r) => Poll::Ready(r),
      PollResult::Poll => {
        let waker = cx.waker().clone();

        tokio::spawn(async move {
          tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
          waker.wake();
        });
        Poll::Pending
      }
    }
  }
}

#[repr(C)]
#[cfg(feature = "waker")]
pub struct LazyableTaskWithWaker<T: FFISafeContainer> {
  pub state: FFIableObject,
  pub waker: Option<*mut c_void>,
  pub poll: extern "C" fn(state: *mut FFIableObject) -> PollResult<T>,
  pub append_waker: extern "C" fn(waker: *mut c_void) -> ()
}

#[cfg(feature = "waker")]
unsafe impl<T: FFISafeContainer> Send for LazyableTaskWithWaker<T> {}

#[cfg(feature = "waker")]
extern "C" fn call_waker_inner(waker: FFIableObject) {
  use std::task::Waker;

  let waker: Waker = unsafe { waker.reconstruct() };
  waker.wake();
}

#[cfg(feature = "waker")]
impl<T: FFISafeContainer + 'static> Future for LazyableTaskWithWaker<T> {
  type Output = T;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let data = (self.poll)(&mut self.state);
    match data {
      PollResult::Ready(r) => Poll::Ready(r),
      PollResult::Poll => {
        if self.waker.is_none() {
          let waker: std::task::Waker = cx.waker().clone();
          self.waker = Some({
            (WAKER.create_waker)(FFIableObject::create_using_box_no_display(waker), call_waker_inner)
          });

          (self.append_waker)(self.waker.clone().unwrap())
        }

        Poll::Pending
      }
    }
  }
}
