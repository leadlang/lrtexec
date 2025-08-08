use std::env;

fn main() {
  if let Err(_) = env::var("LRT_HOME") {
    println!("cargo:warning=`LRT_HOME` was not found. This library depends on LRT (Lead Runtime) for the `waker` dynamic library. If you're planning to run this dylib in this environment without LRT, the library will fail as LRT is not present. Ignore if you are only compiling this on a CI and do not plan to test this library.");
  }
}
