#![cfg_attr(docsrs, feature(doc_cfg))]

use proc_macro::TokenStream;

mod declare;
mod macros;

#[cfg(feature = "internal")]
#[proc_macro]
pub fn declare(item: TokenStream) -> TokenStream {
  declare::declare(item)
}

#[cfg(feature = "internal")]
#[proc_macro]
pub fn ver(item: TokenStream) -> TokenStream {
  macros::ver(item)
}
