use std::collections::HashMap;

pub struct Ref {
  
}

pub struct Command {
  call: String,
  // Standard argv split by ` `
  argv: Vec<String>
}

pub struct Codebase {
  references: HashMap<String, Ref>,
  instructions: Vec<Command>
}