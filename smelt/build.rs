use std::env::var;

pub fn main() {
  // Add the TARGET env
  println!("cargo:rustc-env=TARGET={}", var("TARGET").unwrap());
}