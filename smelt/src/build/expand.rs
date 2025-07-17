use std::{collections::HashMap, fs, path::PathBuf, sync::LazyLock};

use regex::{Regex, RegexBuilder};

// use super::codegen;

static REGEX: LazyLock<Regex> = LazyLock::new(|| {
  RegexBuilder::new("import ([a-z:0-9]+)( as ([a-z0-9]+))?")
    .case_insensitive(true)
    .build()
    .expect("Unknown error")
});

// macros the defined by the `@macroName` syntax for core macros and `@pkg:macroName` syntax for external macros
// You can define the pkg using the `{file}.macro` file
// the content of the whole line is passed to the macro with that name
pub fn expand(file: &PathBuf) {
  let f_str = file.to_str().expect("Invalid file encoding");

  get_external_macros(f_str);
}

/// Gets macros and maps it to packages
pub fn get_external_macros(file: &str) -> HashMap<String, String> {
  let splitted = file.split_once(".lrt").expect("Invalid file extension").0;

  let macro_path = format!("{splitted}.macro");

  let data = fs::read_to_string(macro_path).unwrap_or_default();

  let data = data.lines()
    .map(|x| x.trim())
    .filter(|x| !x.is_empty() && !x.starts_with("#"))

    .map(|x| {
      let captures = REGEX.captures(x).expect("Error");

      let pkg = captures.get(1).expect("Expected import").as_str();
      let imp_as = captures.get(3).and_then(|x| Some(x.as_str()));

      (pkg, imp_as)
    })

    .collect::<Vec<_>>();

  println!("{data:?}");

  todo!("Will be done soon");
}