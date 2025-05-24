use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, format_ident, quote};
use syn::{
  Ident, Index, Result, Token,
  parse::{Parse, ParseStream},
  parse_macro_input,
  punctuated::Punctuated,
};

#[derive(Clone)]
struct Instruction(u8, Ident, u8);

impl ToTokens for Instruction {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let index = self.0;
    let name = &self.1;

    let name = name.to_string();

    let inst = format!("{}_{}_{}", self.2, index, name);

    tokens.extend(quote! {
      #inst => #index,
    });
  }
}

struct StructInstruction(InstructionSet, u8);

impl ToTokens for StructInstruction {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let instruction = &self.0.0.iter().map(|x| &x.1).collect::<Vec<_>>();
    let indexes = &self
      .0
      .0
      .iter()
      .enumerate()
      .map(|(x, _)| Index::from(x))
      .collect::<Vec<_>>();

    let ver = Index::from(self.1 as usize);
    let name = if self.1 == 0 {
      format_ident!("StructInstructions")
    } else {
      format_ident!("ExtendedInstructionsV{}", &ver)
    };

    tokens.extend(quote! {
      pub enum #name {
        #(#instruction = #indexes),*
      }
    });
  }
}

fn parse_instruction(index: u8, name: Ident) -> Instruction {
  Instruction(index, name, 0)
}

#[derive(Clone)]
struct InstructionSet(Vec<Instruction>);

impl ToTokens for InstructionSet {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let tok = &self.0;

    tokens.extend(quote! {
      pub fn map_to() -> Option<u8> {
        Some(match "" {
          #(#tok)*
          _ => {
            return None
          }
        })
      }
    });
  }
}

impl Parse for InstructionSet {
  fn parse(input: ParseStream) -> Result<Self> {
    let content = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

    Ok(InstructionSet(
      content
        .into_iter()
        .collect::<Vec<Ident>>()
        .into_iter()
        .enumerate()
        .map(|(index, name)| parse_instruction(index as u8, name))
        .collect(),
    ))
  }
}

impl Iterator for InstructionSet {
  type Item = Instruction;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

#[derive(Clone)]
struct ExtendedInstructionSet(u8, Vec<Instruction>);

impl ToTokens for ExtendedInstructionSet {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let tok = &self.1;

    tokens.extend(quote! {
      #(#tok)*
    });
  }
}

impl Parse for ExtendedInstructionSet {
  fn parse(input: ParseStream) -> Result<Self> {
    let id = input.parse::<Index>()?.index as u8;
    _ = input.parse::<Token![;]>();
    let content = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;

    Ok(ExtendedInstructionSet(
      id,
      content
        .into_iter()
        .collect::<Vec<Ident>>()
        .into_iter()
        .enumerate()
        .map(|(index, name)| Instruction(index as u8, name, id))
        .collect(),
    ))
  }
}

impl Iterator for ExtendedInstructionSet {
  type Item = Instruction;

  fn next(&mut self) -> Option<Self::Item> {
    self.1.pop()
  }
}

pub fn inst(item: TokenStream) -> TokenStream {
  let instruction = parse_macro_input!(item as InstructionSet);

  let def = StructInstruction(instruction.clone(), 0);

  quote! {
    #instruction

    #def
  }
  .into()
}

pub fn extend(item: TokenStream) -> TokenStream {
  let instruction = parse_macro_input!(item as ExtendedInstructionSet);

  let def = StructInstruction(InstructionSet(instruction.1.clone()), instruction.0);

  quote! {
    #instruction

    #def
  }
  .into()
}
