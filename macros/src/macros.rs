use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, Result, Token};

struct VarName(syn::Ident);

impl Parse for VarName {
  fn parse(input: ParseStream) -> Result<Self> {
    Ok(VarName(input.parse()?))
  }
}

impl ToTokens for VarName {
  fn to_tokens(&self, tokens: &mut TokenStream2) {
    let name = &self.0;
    let lit = name.to_string();

    tokens.extend(quote! {
      #[cfg(
        any(
          feature = "v_all",
          feature = #lit
        )
      )]
      pub mod #name;
    });
  }
}

struct NameSet(Vec<VarName>);

impl Parse for NameSet {
  fn parse(input: ParseStream) -> Result<Self> {
    let content = Punctuated::<VarName, Token![,]>::parse_terminated(input)?;

    Ok(NameSet(content.into_iter().collect()))
  }
}

impl Iterator for NameSet {
  type Item = VarName;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.pop()
  }
}

pub fn ver(item: TokenStream) -> TokenStream {
  let data = parse_macro_input!(item as NameSet);

  quote! {
    #(#data)*
  }.into()
}