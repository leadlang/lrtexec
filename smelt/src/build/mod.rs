use std::{fs, ops::{Deref, DerefMut}};

use crate::config::{Dependency, Package};

use indicatif::{MultiProgress, ProgressBar};
use tokio::{runtime::Builder, task::JoinSet};

mod downloader;

pub fn build(_release: bool, _host: bool, out: Option<&str>) {
  let _out = out.unwrap_or("dist");

  // Create cache directory
  fs::create_dir_all("lrt_cache").unwrap();

  let pkg = Package::get().expect("Unable to get package");

  let runtime = Builder::new_current_thread()
    .thread_name("Downloader")
    .enable_all()
    .build()
    .expect("Runtime");

  let pkg = runtime.block_on(async move {
    let mut set = JoinSet::new();

    let progress = MultiProgress::new();

    // SAFETY:
    // - name is a valid string reference throughout `JoinSet` lifetime
    // - dep is a valid dependency reference throughout `JoinSet` lifetime
    pkg.dependencies.iter().for_each(|(name, dep)| {
      let name = name as &str;
      let name = name as *const str;
      let name = unsafe { &*name };

      let dep = dep as &Dependency;
      let dep = dep as *const Dependency;
      let dep = unsafe { &*dep };

      set.spawn(NoSendWrapper(downloader::download(name, dep, progress.add(
        ProgressBar::new_spinner()
            .with_message("Downloading")
      ))));
    });

    set.join_all().await;

    pkg
  });
}

struct NoSendWrapper<T: Future>(T);

unsafe impl<T: Future> Send for NoSendWrapper<T> {}
impl <T: Future> Unpin for NoSendWrapper<T> {

}
impl<T:Future> Deref for NoSendWrapper<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl<T: Future> DerefMut for NoSendWrapper<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl<T: Future> Future for NoSendWrapper<T> {
  type Output = T::Output;

  fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
    unsafe { std::pin::Pin::new_unchecked(&mut self.0) }.poll(cx)
  }
}
