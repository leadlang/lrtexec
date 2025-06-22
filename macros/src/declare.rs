use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
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
  // cmdset: u8,
}

impl Instruction {
  fn create(/*set: u8,*/ id: u8, name: Ident) -> Self {
    Instruction {
      name,
      id,
      // cmdset: set,
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
      .map(|(id, data)| Instruction::create(if set == 0 { id + 1 } else { id } as u8, data))
      .collect();

    InstSet {
      cmdset: set,
      instructions: data,
    }
  }
}

struct GlobalData {
  set: Ident,
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
    let set = input.parse::<Ident>()?;
    let _ = input.parse::<Token![;]>()?;
    let content = Punctuated::<GroupedBlocks, Token![,]>::parse_terminated(input)?;

    let insts = content
      .into_iter()
      .enumerate()
      .map(|(set, inst)| InstSet::new(set as u8, inst.0))
      .collect::<Vec<_>>();

    Ok(GlobalData { set, insts })
  }
}

impl ToTokens for GlobalData {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let insts = &self.insts;

    let mut to_int = TokenStream2::new();

    let mut general_instruction_set = TokenStream2::new();
    let mut extended_instruction_set = TokenStream2::new();

    insts.iter().for_each(|x| {
      let id = x.cmdset;

      if id != 0 {
        let mut matches = quote! {

        };

        x.instructions.iter().for_each(|x| {
          let name = &x.name;

          let name_str: &str = &x.name.to_string();
          let set = x.id;

          let fn_name = format_ident!("execute_{}_{}", self.set, name);

          to_int.extend(quote! {
            #name_str => {
              vect.push(0u8);
              vect.push(#id);
              vect.push(#set);
              Some(())
            }
          });

          matches.extend(quote! {
            #set => {
              #fn_name(tokens, ends_at, $($argname),*)?;
              return Ok(());
            }
          });
        });

        let subcmd = quote! {
          #id => {
            match &tokens[2] {
              #matches
            }
            return Ok(());
          }
        };

        extended_instruction_set.extend(subcmd);
      } else {
        x.instructions.iter().for_each(|x| {
          let name = &x.name;
          
          let name_str: &str = &x.name.to_string();
          let set = x.id;

          let fn_name = format_ident!("execute_{}_{}", self.set, name);

          to_int.extend(quote! {
            #name_str => {
              vect.push(#set);
              Some(())
            }
          });

          general_instruction_set.extend(quote! {
            #set => {
              #fn_name(tokens, ends_at, $($argname),*)?;
              return Ok(());
            },
          });
        });
      }
    });

    let cmd_map_name = format_ident!("cmd_map_{}", self.set);

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

      #[macro_export]
      /// Macro to create a command parser.
      /// This is a macro that speeds up runtime execution and complications of changing intruction numbers.
      /// 
      /// # Format
      /// ```rust
      /// cmd_map! {
      ///   <arg1 name>: <arg1 type>, <arg2 name>: <arg2 type>, ...
      /// }
      /// ```
      /// 
      /// # What does it return?
      /// The macro creates a function named `handle_intructionset` that takes a vector of bytes and returns () on success or a ParseError and the arguments.
      /// The macro also creates an Error type `ParseError`
      macro_rules! #cmd_map_name {
        (
          $($argname:ident : $argty:ty ),*
        ) => {
          pub enum ParseError {
            InvalidInstruction,
            TokensNotEnough,
            CustomDefinedError(String)
          }

          pub fn handle_intructionset(tokens: &[u8], ends_at: &mut usize, $($argname: $argty),*) -> ::std::result::Result<(), ParseError> {
            if tokens.is_empty() {
              return Err(ParseError::TokensNotEnough);
            }

            *ends_at += 1;
            match &tokens[0] {
              0 => {
                if tokens.len() <= 2 {
                  return Err(ParseError::TokensNotEnough);
                }
                *ends_at += 2;

                // Extension CMDSet Id
                match &tokens[1] {
                  #extended_instruction_set
                  _ => {
                    return Err(ParseError::InvalidInstruction);
                  }
                };
              }
              #general_instruction_set
              _ => Err(ParseError::InvalidInstruction)
            }
          }
        };
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
