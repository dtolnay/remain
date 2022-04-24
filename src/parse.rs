use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Error, Expr, Fields, Result, Stmt, Token, Visibility};

use crate::emit::Kind;

pub enum Input {
    Enum(syn::ItemEnum),
    Match(syn::ExprMatch),
    Struct(syn::ItemStruct),
    Let(syn::ExprMatch),
}

impl Input {
    pub fn kind(&self) -> Kind {
        match self {
            Input::Enum(_) => Kind::Enum,
            Input::Match(_) => Kind::Match,
            Input::Struct(_) => Kind::Struct,
            Input::Let(_) => Kind::Let,
        }
    }
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let ahead = input.fork();
        let _ = ahead.call(Attribute::parse_outer)?;

        if ahead.peek(Token![match]) {
            let expr = match input.parse()? {
                Expr::Match(expr) => expr,
                _ => unreachable!("expected match"),
            };
            return Ok(Input::Match(expr));
        }

        if ahead.peek(Token![let]) {
            let stmt = match input.parse()? {
                Stmt::Local(stmt) => stmt,
                _ => unreachable!("expected let"),
            };
            let init = match stmt.init {
                Some((_, init)) => *init,
                None => return Err(unexpected()),
            };
            let expr = match init {
                Expr::Match(expr) => expr,
                _ => return Err(unexpected()),
            };
            return Ok(Input::Let(expr));
        }

        let _: Visibility = ahead.parse()?;
        if ahead.peek(Token![enum]) {
            return input.parse().map(Input::Enum);
        } else if ahead.peek(Token![struct]) {
            let input: syn::ItemStruct = input.parse()?;
            if let Fields::Named(_) = &input.fields {
                return Ok(Input::Struct(input));
            }
        }

        Err(unexpected())
    }
}

impl ToTokens for Input {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Input::Enum(item) => item.to_tokens(tokens),
            Input::Struct(item) => item.to_tokens(tokens),
            Input::Match(expr) | Input::Let(expr) => expr.to_tokens(tokens),
        }
    }
}

fn unexpected() -> Error {
    let span = Span::call_site();
    let msg = "expected enum, struct, or match expression";
    Error::new(span, msg)
}
