use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::Error;

#[derive(Copy, Clone)]
pub enum Kind {
    Enum,
    Match,
    Struct,
    Let,
}

pub fn emit(err: Error, kind: Kind, output: TokenStream) -> TokenStream {
    let mut err = err;
    if !probably_has_spans(kind) {
        // Otherwise the error is printed without any line number.
        err = Error::new(Span::call_site(), &err.to_string());
    }

    let err = err.to_compile_error();
    let output = proc_macro2::TokenStream::from(output);

    let expanded = match kind {
        Kind::Enum | Kind::Let | Kind::Struct => quote!(#err #output),
        Kind::Match => quote!({ #err #output }),
    };

    TokenStream::from(expanded)
}

// Rustc is so bad at spans.
// https://github.com/rust-lang/rust/issues/43081
fn probably_has_spans(kind: Kind) -> bool {
    match kind {
        Kind::Enum | Kind::Struct => true,
        Kind::Match | Kind::Let => false,
    }
}
