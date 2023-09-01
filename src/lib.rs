use quote::ToTokens;
extern crate proc_macro;
use proc_macro::Span;
use proc_macro::TokenStream;
use quote::quote;
use syn::parenthesized;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::Attribute;
use syn::Block;
use syn::FnArg;
use syn::Generics;
use syn::Ident;
use syn::Result;
use syn::Token;

struct Attributes {
    attrs: Punctuated<Ident, Token![,]>,
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Attributes {
            attrs: input.parse_terminated(Ident::parse)?,
        })
    }
}

struct SpecializableFunction {
    attrs: Vec<Attribute>,
    _fn_token: Token![fn],
    ident: Ident,
    _gen: Generics,
    _paren: Paren,
    _args: Punctuated<FnArg, Token![,]>,
    _block: Block,
}

impl Parse for SpecializableFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(SpecializableFunction {
            attrs: input.call(Attribute::parse_outer)?,
            _fn_token: input.parse()?,
            ident: input.parse()?,
            _gen: input.parse()?,
            _paren: parenthesized!(content in input),
            _args: content.parse_terminated(FnArg::parse)?,
            _block: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn test_with(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ret = item.clone();
    let attributes = parse_macro_input!(attr as Attributes);
    let function = parse_macro_input!(item as SpecializableFunction);
    if function._args.len() > 0 {
        panic!("Testable function must have no arguments");
    }

    let name = &function.ident;
    let name_str = function.ident.to_string();

    for a in attributes.attrs {
        let mut fun_full_name = "_specialized__".to_string();
        fun_full_name.push_str(&name_str);
        fun_full_name.push_str("__");
        fun_full_name.push_str(&a.to_string());
        fun_full_name.push('_');
        let fun_full_ident = Ident::new(&fun_full_name, Span::call_site().into());

        for super_attr in &function.attrs {
            ret.extend(Into::<TokenStream>::into(super_attr.to_token_stream()));
        }
        let expanded = quote!(
            #[test]
            fn #fun_full_ident() {
                #name::<#a>();
            }
        );
        ret.extend(TokenStream::from(expanded));
    }

    ret
}
