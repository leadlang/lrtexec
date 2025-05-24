use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
  Ident, Result, Token, braced,
  parse::{Parse, ParseStream, Parser},
  parse_macro_input,
  punctuated::Punctuated,
};

#[derive(Clone)]
struct Instruction {
  name: Ident,
  id: u8,
  cmdset: u8,
}

impl Instruction {
  fn create(set: u8, id: u8, name: Ident) -> Self {
    Instruction {
      name,
      id,
      cmdset: set,
    }
  }
}

struct InstSet {
  cmdset: u8,
  instructions: Vec<Instruction>,
}

impl InstSet {
  fn new(set: u8, input: TokenStream2) -> Self {
    let data = Punctuated::<Ident, Token![,]>::parse_terminated
      .parse2(input)
      .unwrap();

    let data = data
      .into_iter()
      .enumerate()
      // Set 0,s Id 0 = extend command
      .map(|(id, data)| Instruction::create(set, if set == 0 { id + 1 } else { id } as u8, data))
      .collect();

    InstSet {
      cmdset: set,
      instructions: data,
    }
  }
}

struct GlobalData {
  insts: Vec<InstSet>,
}

struct GroupedBlocks(TokenStream2);

impl Parse for GroupedBlocks {
  fn parse(input: ParseStream) -> Result<Self> {
    let content;
    braced!(content in input);

    Ok(GroupedBlocks(content.parse()?))
  }
}

impl Parse for GlobalData {
  fn parse(input: ParseStream) -> Result<Self> {
    let content = Punctuated::<GroupedBlocks, Token![,]>::parse_terminated(input)?;

    let insts = content
      .into_iter()
      .enumerate()
      .map(|(set, inst)| InstSet::new(set as u8, inst.0))
      .collect::<Vec<_>>();

    Ok(GlobalData { insts })
  }
}

impl ToTokens for GlobalData {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let insts = &self.insts;

    let mut to_int = TokenStream2::new();
    insts.iter().for_each(|x| {
      let id = x.cmdset;

      if id != 0 {
        x.instructions.iter().for_each(|x| {
          let name: &str = &x.name.to_string();
          let set = x.id;

          to_int.extend(quote! {
            #name => {
              vect.push(0u8);
              vect.push(#id);
              vect.push(#set);
              Some(())
            }
          });
        });
      } else {
        x.instructions.iter().for_each(|x| {
          let name: &str = &x.name.to_string();
          let set = x.id;

          to_int.extend(quote! {
            #name => {
              vect.push(#set);
              Some(())
            }
          });
        });
      }
    });

    tokens.extend(quote! {
      pub fn cmd_to_int(
        cmd: &str,
        vect: &mut Vec<u8>
      ) -> Option<()> {
        match cmd {
          #to_int
          _ => None
        }
      }

      pub fn handle_line(
        s: &mut Script
      ) {

      }
    });
  }
}

pub fn declare(item: TokenStream) -> TokenStream {
  let _data = parse_macro_input!(item as GlobalData);

  quote! {
    #_data
  }
  .into()
}
