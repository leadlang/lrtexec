use std::{collections::HashMap, fs, io::{Error, ErrorKind, Result}};

use serde::{Serialize, Deserialize};
use serde_json::from_str;

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
  pub name: String,
  pub version: String,
  pub description: String,
  pub authors: Vec<String>,
  pub dependencies: HashMap<String, Dependency>,
  pub platforms: String,
  pub platform: HashMap<String, Platform>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
  pub dependencies: HashMap<String, Dependency>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
  pub source: Source,
  pub tag_name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Source {
  #[serde(rename = "github")]
  GitHub
}

impl Package {
  pub fn get() -> Result<Self> {
    Ok(
      from_str(
        fs::read_to_string("./lrt.json")?.as_str()
      ).map_err(|_| 
        Error::new(ErrorKind::InvalidData, "Invalid Data")
      )?
    )
  }
}

impl Default for Package {
  fn default() -> Self {
    Self {
      name: String::from(""),
      version: String::from(""),
      description: String::from(""),
      authors: vec![],
      dependencies: HashMap::new(),
      platforms: String::from(""),
      platform: HashMap::new()
    }
  }
}