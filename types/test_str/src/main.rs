use lrtexec_lib::common::others::FFISafeString;

fn main() {
  let mut n = FFISafeString::null();

  println!("{} len={}", &*n, n.len());

  n.push_str("Hello World");
  println!("{} len={}", n, n.len());

  n.edit(1, "eLLo_WORld_001 This will surely go beyond the alloacation that was done. I am sure about that");
  println!("{} len={}", n, n.len());

  println!("{}", &n[0..11]);
}
