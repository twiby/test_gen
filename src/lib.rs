extern crate proc_macro;
use proc_macro::Span;
use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse::ParseStream;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::Attribute;
use syn::Ident;
use syn::ItemFn;
use syn::Meta;
use syn::Path;
use syn::Result;
use syn::Token;

// TODO: handle multiple type argument ?

/// Trait for getting a quasi unique valid identifier for an object.
trait ToFunName {
    fn to_fun_name(&self) -> String;
}
impl ToFunName for Path {
    fn to_fun_name(&self) -> String {
        self.to_token_stream()
            .to_string()
            .chars()
            .filter_map(|c| match c {
                ' ' => None,
                '<' | '>' => Some('_'),
                ':' => Some('_'),
                c => Some(c),
            })
            .collect::<String>()
    }
}

/// Type for listing every type on which to specialize our test
struct TypesToTest {
    attrs: Punctuated<Path, Token![,]>,
}

impl Parse for TypesToTest {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(TypesToTest {
            attrs: input.parse_terminated(Path::parse, Token![,])?,
        })
    }
}

/// Type that wraps an `ItemFn`along with a bool specifying if the `#[should_panic]`
/// attribute was added.
struct TestFn {
    fun: ItemFn,
    should_panic: bool,
}

impl TestFn {
    fn is_attribute_should_panic(a: &Attribute) -> bool {
        match a {
            Attribute {
                pound_token: _,
                style: syn::AttrStyle::Outer,
                bracket_token: _,
                meta: Meta::Path(p),
            } => p.is_ident(&Ident::new("should_panic", Span::mixed_site().into())),
            _ => false,
        }
    }
}

impl Parse for TestFn {
    fn parse(input: ParseStream) -> Result<Self> {
        let fun: ItemFn = input.parse()?;
        let should_panic = fun.attrs.iter().any(Self::is_attribute_should_panic);

        Ok(Self {
            fun: fun,
            should_panic: should_panic,
        })
    }
}

/// procedural attribute macro: can replace the attribute `#[test]` with
/// `#[test_with(u32, u64)]` for example (when placed on a function with only one generic type parameter),
/// to generate a test on each type.
///
/// ```rust
/// use typed_test_gen::test_with;
///
/// #[test_with(u32, u64, char)]
/// fn test_vec<T>() {
///     let vec = Vec::<T>::with_capacity(10);
///     assert_eq!(vec.len(), 0);
///     assert!(vec.capacity() >= 10);
/// }
/// ```
///
/// Supports using the `#[should_panic]` attribute
/// ```rust
/// use typed_test_gen::test_with;
///
/// #[test_with(u32, u64, char)]
/// #[should_panic]
/// fn test_vec<T>() {
///     let vec = Vec::<T>::with_capacity(10);
///     assert_eq!(vec.len(), 0);
///     assert!(vec.capacity() < 10);
/// }
/// ```
#[proc_macro_attribute]
pub fn test_with(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ret = item.clone();
    let types = parse_macro_input!(attr as TypesToTest);
    let function = parse_macro_input!(item as TestFn);
    if function.fun.sig.inputs.len() > 0 {
        panic!("Testable function must have no arguments");
    }

    let name = &function.fun.sig.ident;
    let name_str = name.to_string();

    for a in types.attrs {
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
