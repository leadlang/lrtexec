use std::collections::HashMap;

use stabby::stabby;

use crate::VariableData;

#[stabby]
pub trait RHashMap {
  extern "C" fn get<'a>(&'a self, key: &'a u16) -> Option<&'a VariableData>;
  extern "C" fn insert(&mut self, key: u16, value: VariableData);
}

pub struct RTVariableMap {
  map: HashMap<u16, VariableData>,
}

impl RTVariableMap {
  pub fn new() -> Self {
    Self {
      map: HashMap::new(),
    }
  }
}

impl RHashMap for RTVariableMap {
  extern "C" fn get<'a>(&'a self, key: &'a u16) -> Option<&'a VariableData> {
    self.map.get(key)
  }

  extern "C" fn insert(&mut self, key: u16, value: VariableData) {
    self.map.insert(key, value);
  }
}
