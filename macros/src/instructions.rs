use proc_macro::TokenStream;
use syn::{parse::{Parse, ParseStream}, punctuated::Punctuated, Ident, Result, Token};

struct Instruction(u8, Ident);

fn parse_instruction(index: &u8, name: Ident) -> Instruction {

}

struct InstructionSet(Vec<Instruction>);

impl Parse for InstructionSet {
  fn parse(input: ParseStream) -> Result<Self> {
    let content = Punctuated::<Instruction, Token![,]>::parse_terminated(input)?;

    Ok(InstructionSet(content.into_iter().collect()))
  }
}

impl Iterator for InstructionSet {
  type Item = Instruction;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

pub fn inst(item: TokenStream) -> TokenStream {

}