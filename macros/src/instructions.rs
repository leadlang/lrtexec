use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, punctuated::Punctuated, Ident, Result, Token};

struct Instruction(u8, Ident);

impl ToTokens for Instruction {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let index = self.0;
    let name = &self.1;

    tokens.extend(quote! {
      #name => #index
    });
  }
}

fn parse_instruction(index: u8, name: Ident) -> Instruction {
  Instruction(index, name)
}

struct InstructionSet(Vec<Instruction>);

impl Parse for InstructionSet {
  fn parse(input: ParseStream) -> Result<Self> {
    let content = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

    Ok(InstructionSet(
      content.into_iter()
        .collect::<Vec<Ident>>()
        .into_iter()
        .enumerate()
        .map(|(index, name)| parse_instruction(index as u8, name))
        .collect()
    ))
  }
}

impl Iterator for InstructionSet {
  type Item = Instruction;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

pub fn inst(item: TokenStream) -> TokenStream {
  item
}