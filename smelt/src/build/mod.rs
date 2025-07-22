use std::{
  fs, ops::{Deref, DerefMut}, path::PathBuf
};

use crate::config::{Dependency, Package};

use anyhow::Context;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use tokio::{runtime::Builder, task::JoinSet};

use rayon::prelude::*;

mod downloader;
mod builder;

pub(super) mod codegen;
pub(super) mod foldergen;

mod expand;

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

  let _pkg = runtime.block_on(async move {
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

      set.spawn(NoSendWrapper(downloader::download(
        name,
        dep,
        progress.add(ProgressBar::new_spinner().with_message("Downloading")),
      )));
    });

    set.join_all().await;

    pkg
  });

  let dir = trace_files_recursive("src")
    .expect("Unable to find src directory, does it exist?");

  println!("Recursively searched {}", dir.len());

  let progress = ProgressBar::new(dir.len() as u64)
    .with_style(
      ProgressStyle::with_template(
        "{prefix:>12.cyan.bold} [{bar:57}] {pos}/{len}"
      ).expect("Impossible error")
    ) 
    .with_prefix("Expanding");

  // Expand (i.e. codegen)
  dir.par_iter()
    .for_each(|x| {
      expand::expand(x);
      progress.inc(1);
    });

  progress.finish();
}

fn trace_files_recursive(dir_path: &str) -> anyhow::Result<Vec<PathBuf>> {
  let mut files_found: Vec<PathBuf> = Vec::new();

  if fs::metadata(dir_path)?.is_dir() {
    for entry in fs::read_dir(dir_path)? {
      let entry = entry?;
      let path = entry.path();

      if path.is_dir() {
        // If it's a directory, recurse into it and extend our list
        files_found.extend(trace_files_recursive(path.to_str().context("Invalid string")?)?);
      } else {
        // If it's a file, add its path to our list
        println!("Path: {:?}", path);
        if path.to_str().expect("Impossible").ends_with(".lrt") {
          files_found.push(path);
        }
      }
    }
  }
  Ok(files_found)
}

struct NoSendWrapper<T: Future>(T);

unsafe impl<T: Future> Send for NoSendWrapper<T> {}
impl<T: Future> Unpin for NoSendWrapper<T> {}
impl<T: Future> Deref for NoSendWrapper<T> {
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

  fn poll(
    mut self: std::pin::Pin<&mut Self>,
    cx: &mut std::task::Context<'_>,
  ) -> std::task::Poll<Self::Output> {
    unsafe { std::pin::Pin::new_unchecked(&mut self.0) }.poll(cx)
  }
}
