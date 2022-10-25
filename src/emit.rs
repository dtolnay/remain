use proc_macro::TokenStream;
use quote::quote;
use syn::Error;

#[derive(Copy, Clone)]
pub enum Kind {
    Enum,
    Match,
    Struct,
    Let,
}

pub fn emit(err: &Error, kind: Kind, output: TokenStream) -> TokenStream {
    let err = err.to_compile_error();
    let output = proc_macro2::TokenStream::from(output);

    let expanded = match kind {
        Kind::Enum | Kind::Let | Kind::Struct => quote!(#err #output),
        Kind::Match => quote!({ #err #output }),
    };

    TokenStream::from(expanded)
}
