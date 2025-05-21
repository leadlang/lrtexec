#![cfg_attr(docsrs, feature(doc_cfg))]

use proc_macro::TokenStream;

mod macros;
mod instructions;

#[cfg(feature = "internal")]
#[proc_macro]
pub fn instructions(item: TokenStream) -> TokenStream {
  instructions::inst(item)
}

#[cfg(feature = "internal")]
#[proc_macro]
pub fn ver(item: TokenStream) -> TokenStream {
  macros::ver(item)
}