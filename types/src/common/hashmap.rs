use std::collections::HashMap;

use stabby::stabby;

use crate::CVariable;

#[stabby]
pub trait RHashMap {
  extern "C" fn get<'a>(&'a self, key: &'a u16) -> Option<&'a CVariable>;
  extern "C" fn insert(&mut self, key: u16, value: CVariable);
}

pub struct RTVariableMap {
  map: HashMap<u16, CVariable>,
}

impl RTVariableMap {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
    }
  }
}

impl RHashMap for RTVariableMap {
  extern "C" fn get<'a>(&'a self, key: &'a u16) -> Option<&'a CVariable> {
    self.map.get(key)
  }

  extern "C" fn insert(&mut self, key: u16, value: CVariable) {
    self.map.insert(key, value);
  }
}
