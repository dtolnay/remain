use quote::quote;
use syn::{Arm, Attribute, Ident, Result, Variant};
use syn::{Error, Field, Pat, PatIdent};
use syn::{ExprMatch, ItemEnum, ItemStruct};

use crate::compare::Path;
use crate::format;
use crate::parse::Input::{self, *};

pub fn sorted(input: &mut Input) -> Result<()> {
    let paths = match input {
        Enum(item) => filter_unsorted_enum(item)?,
        Struct(item) => filter_unsorted_struct(item)?,
        Match(expr) | Let(expr) => filter_unsorted_match(expr)?,
    };

    for i in 1..paths.len() {
        let cur = &paths[i];
        if *cur < paths[i - 1] {
            let lesser = cur;
            let correct_pos = paths[..i - 1].binary_search(cur).unwrap_err();
            let greater = &paths[correct_pos];
            return Err(format::error(lesser, greater));
        }
    }

    Ok(())
}

fn take_unsorted_attr(attrs: &mut Vec<Attribute>) -> bool {
    for i in 0..attrs.len() {
        let path = &attrs[i].path;
        let path = quote!(#path).to_string();
        if path == "unsorted" || path == "remain :: unsorted" {
            attrs.remove(i);
            return true;
        }
    }

    false
}

fn filter_unsorted_enum(item: &mut ItemEnum) -> Result<Vec<Path>> {
    item.variants
        .iter_mut()
        .filter_map(|variant| {
            if take_unsorted_attr(&mut variant.attrs) {
                return None;
            }
            Some(variant.to_path())
        })
        .collect()
}

fn filter_unsorted_struct(item: &mut ItemStruct) -> Result<Vec<Path>> {
    item.fields
        .iter_mut()
        .filter_map(|field| {
            if take_unsorted_attr(&mut field.attrs) {
                return None;
            }
            Some(field.to_path())
        })
        .collect()
}

fn filter_unsorted_match(expr: &mut ExprMatch) -> Result<Vec<Path>> {
    expr.arms
        .iter_mut()
        .filter_map(|arm| {
            if take_unsorted_attr(&mut arm.attrs) {
                return None;
            }
            Some(arm.to_path())
        })
        .collect()
}

trait ToPath {
    fn to_path(&self) -> Result<Path>;
}

impl ToPath for Variant {
    fn to_path(&self) -> Result<Path> {
        Ok(Path {
            segments: vec![self.ident.clone()],
        })
    }
}

impl ToPath for Field {
    fn to_path(&self) -> Result<Path> {
        Ok(Path {
            segments: vec![self.ident.clone().expect("must be named field")],
        })
    }
}

impl ToPath for Arm {
    fn to_path(&self) -> Result<Path> {
        // Sort by just the first pat.
        let pat = match &self.pat {
            Pat::Or(pat) => pat.cases.iter().next().expect("at least one pat"),
            _ => &self.pat,
        };

        let segments = match pat {
            Pat::Ident(pat) if is_just_ident(&pat) => vec![pat.ident.clone()],
            Pat::Path(pat) => idents_of_path(&pat.path),
            Pat::Struct(pat) => idents_of_path(&pat.path),
            Pat::TupleStruct(pat) => idents_of_path(&pat.path),
            Pat::Wild(pat) => vec![Ident::from(pat.underscore_token)],
            other => {
                let msg = "unsupported by #[remain::sorted]";
                return Err(Error::new_spanned(other, msg));
            }
        };

        Ok(Path { segments })
    }
}

fn idents_of_path(path: &syn::Path) -> Vec<Ident> {
    path.segments
        .clone()
        .into_iter()
        .map(|seg| seg.ident)
        .collect()
}

fn is_just_ident(pat: &PatIdent) -> bool {
    pat.by_ref.is_none() && pat.mutability.is_none() && pat.subpat.is_none()
}
