#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use proc_macro::TokenStream;
mod macros {
    use proc_macro::TokenStream;
    use proc_macro2::TokenStream as TokenStream2;
    use quote::{quote, ToTokens};
    use syn::{
        parse::{Parse, ParseStream},
        parse_macro_input, punctuated::Punctuated, Result, Token,
    };
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
            tokens
                .extend({
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_pound(&mut _s);
                    ::quote::__private::push_group(
                        &mut _s,
                        ::quote::__private::Delimiter::Bracket,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "cfg");
                            ::quote::__private::push_group(
                                &mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "any");
                                    ::quote::__private::push_group(
                                        &mut _s,
                                        ::quote::__private::Delimiter::Parenthesis,
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "feature");
                                            ::quote::__private::push_eq(&mut _s);
                                            ::quote::__private::parse(&mut _s, "\"v_all\"");
                                            ::quote::__private::push_comma(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "feature");
                                            ::quote::__private::push_eq(&mut _s);
                                            ::quote::ToTokens::to_tokens(&lit, &mut _s);
                                            _s
                                        },
                                    );
                                    _s
                                },
                            );
                            _s
                        },
                    );
                    ::quote::__private::push_ident(&mut _s, "pub");
                    ::quote::__private::push_ident(&mut _s, "mod");
                    ::quote::ToTokens::to_tokens(&name, &mut _s);
                    ::quote::__private::push_semi(&mut _s);
                    _s
                });
        }
    }
    struct NameSet(Vec<VarName>);
    impl Parse for NameSet {
        fn parse(input: ParseStream) -> Result<Self> {
            let content = Punctuated::<
                VarName,
                ::syn::token::Comma,
            >::parse_terminated(input)?;
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
        let data = match ::syn::parse::<NameSet>(item) {
            ::syn::__private::Ok(data) => data,
            ::syn::__private::Err(err) => {
                return ::syn::__private::TokenStream::from(err.to_compile_error());
            }
        };
        {
            let mut _s = ::quote::__private::TokenStream::new();
            {
                use ::quote::__private::ext::*;
                let has_iter = ::quote::__private::ThereIsNoIteratorInRepetition;
                #[allow(unused_mut)]
                let (mut data, i) = data.quote_into_iter();
                let has_iter = has_iter | i;
                let _: ::quote::__private::HasIterator = has_iter;
                while true {
                    let data = match data.next() {
                        Some(_x) => ::quote::__private::RepInterp(_x),
                        None => break,
                    };
                    ::quote::ToTokens::to_tokens(&data, &mut _s);
                }
            }
            _s
        }
            .into()
    }
}
#[proc_macro]
pub fn ver(item: TokenStream) -> TokenStream {
    macros::ver(item)
}
const _: () = {
    extern crate proc_macro;
    #[rustc_proc_macro_decls]
    #[used]
    #[allow(deprecated)]
    static _DECLS: &[proc_macro::bridge::client::ProcMacro] = &[
        proc_macro::bridge::client::ProcMacro::bang("ver", ver),
    ];
};
