#![cfg_attr(docsrs, feature(doc_cfg))]

use proc_macro::TokenStream;

mod declare;
mod instructions;
mod macros;

#[cfg(feature = "internal")]
#[proc_macro]
pub fn declare(item: TokenStream) -> TokenStream {
  declare::declare(item)
}

#[cfg(feature = "internal")]
#[proc_macro]
pub fn instructions(item: TokenStream) -> TokenStream {
  instructions::inst(item)
}

#[cfg(feature = "internal")]
#[proc_macro]
pub fn extends(item: TokenStream) -> TokenStream {
  instructions::extend(item)
}

#[cfg(feature = "internal")]
#[proc_macro]
pub fn ver(item: TokenStream) -> TokenStream {
  macros::ver(item)
}
