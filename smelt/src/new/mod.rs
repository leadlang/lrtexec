use std::path::PathBuf;
use std::{env::current_exe, fs};

use terminal::owo_colors::OwoColorize;

pub fn pack_dir() -> PathBuf {
  let mut dir = current_exe().unwrap();

  dir.pop();

  dir
}

pub fn copy_dir(src: &str, dest: &str) {
  let paths = fs::read_dir(src).unwrap();

  for path in paths {
    let src_path = path.unwrap().path();
    let dest_path = dest.to_string() + "/" + src_path.file_name().unwrap().to_str().unwrap();

    if src_path.is_dir() {
      fs::create_dir_all(&dest_path).unwrap();
      copy_dir(src_path.to_str().unwrap(), dest_path.as_str());
    } else {
      fs::copy(&src_path, &dest_path).unwrap();
    }
  }
}

pub fn create(chosen_dir: &str) {
  let mut templates = pack_dir();
  templates.push("templates");

  let dir = fs::read_dir(&templates)
    .unwrap()
    .map(|x| x.unwrap())
    .map(|x| x.file_name().into_string().unwrap())
    .collect::<Vec<_>>();

  let template = inquire::Select::new("Select your template", dir)
    .prompt()
    .expect("You must select a template");

  fs::create_dir_all(format!("./{chosen_dir}")).unwrap();

  templates.push(template.as_str());

  let src = templates.to_str().unwrap();

  copy_dir(src, &format!("./{chosen_dir}"));

  // Colorize
  println!("\nCreated project in `{}`", chosen_dir.green());
  println!("Run {} to build the project", "`smelt build`".yellow());
}
