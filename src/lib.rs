extern crate proc_macro;
use proc_macro::Span;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
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
use syn::Meta;
use syn::Path;
use syn::Result;
use syn::Token;

// TODO: handle multiple type argument ?

trait ToFunName {
    fn to_fun_name(&self) -> String;
}
impl ToFunName for Path {
    fn to_fun_name(&self) -> String {
        let token_string = self.to_token_stream().to_string();
        let mut ret = "".to_string();
        for c in token_string.chars() {
            match c {
                ' ' => (),
                '<' | '>' => ret.push('_'),
                ':' => ret.push('_'),
                c => ret.push(c),
            };
        }
        ret
    }
}

struct Attributes {
    attrs: Punctuated<Path, Token![,]>,
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Attributes {
            attrs: input.parse_terminated(Path::parse, Token![,])?,
        })
    }
}

struct SpecializableFunction {
    should_panic: bool,
    _fn_token: Token![fn],
    ident: Ident,
    _gen: Generics,
    _paren: Paren,
    args: Punctuated<FnArg, Token![,]>,
    _block: Block,
}

impl Parse for SpecializableFunction {
    fn parse(input: ParseStream) -> Result<Self> {
        let _attrs: Vec<Attribute> = input.call(Attribute::parse_outer)?;

        let mut should_panic = false;
        for a in _attrs {
            match a {
                Attribute {
                    pound_token: _,
                    style: syn::AttrStyle::Outer,
                    bracket_token: _,
                    meta: Meta::Path(p),
                } => {
                    if p.is_ident(&Ident::new("should_panic", Span::mixed_site().into())) {
                        should_panic = true;
                        break;
                    }
                }
                _ => (),
            };
        }

        let content;
        Ok(SpecializableFunction {
            should_panic: should_panic,
            _fn_token: input.parse()?,
            ident: input.parse()?,
            _gen: input.parse()?,
            _paren: parenthesized!(content in input),
            args: content.parse_terminated(FnArg::parse, Token![,])?,
            _block: input.parse()?,
        })
    }
}

#[proc_macro_attribute]
pub fn test_with(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ret = item.clone();
    let attributes = parse_macro_input!(attr as Attributes);
    let function = parse_macro_input!(item as SpecializableFunction);
    if function.args.len() > 0 {
        panic!("Testable function must have no arguments");
    }

    let name = &function.ident;
    let name_str = function.ident.to_string();

    for a in attributes.attrs {
        let mut fun_full_name = "_specialized__".to_string();
        fun_full_name.push_str(&name_str);
        fun_full_name.push_str("__");
        fun_full_name.push_str(&a.to_fun_name());
        fun_full_name.push('_');
        let fun_full_ident = Ident::new(&fun_full_name, Span::call_site().into());

        if function.should_panic {
            ret.extend(TokenStream::from(quote!(#[should_panic])));
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
