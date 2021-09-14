use quote::quote;
use std::cmp::Ordering;
use syn::spanned::Spanned;
use syn::{Arm, Attribute, Ident, Result, Variant};
use syn::{Error, Field, Pat, PatIdent};

use crate::compare::{cmp, Path, UnderscoreOrder};
use crate::format;
use crate::parse::Input::{self, *};

pub fn sorted(input: &mut Input) -> Result<()> {
    let paths = match input {
        Enum(item) => collect_paths(&mut item.variants)?,
        Struct(item) => collect_paths(&mut item.fields)?,
        Match(expr) | Let(expr) => collect_paths(&mut expr.arms)?,
    };

    let mode = UnderscoreOrder::First;
    if find_misordered(&paths, mode).is_none() {
        return Ok(());
    }

    let mode = UnderscoreOrder::Last;
    let wrong = match find_misordered(&paths, mode) {
        Some(wrong) => wrong,
        None => return Ok(()),
    };

    let lesser = &paths[wrong];
    let correct_pos = match paths[..wrong - 1].binary_search_by(|probe| cmp(probe, lesser, mode)) {
        Err(correct_pos) => correct_pos,
        Ok(equal_to) => equal_to + 1,
    };
    let greater = &paths[correct_pos];
    Err(format::error(lesser, greater))
}

fn find_misordered(paths: &[Path], mode: UnderscoreOrder) -> Option<usize> {
    for i in 1..paths.len() {
        if cmp(&paths[i], &paths[i - 1], mode) == Ordering::Less {
            return Some(i);
        }
    }

    None
}

fn collect_paths<'a, I, P>(iter: I) -> Result<Vec<Path>>
where
    I: IntoIterator<Item = &'a mut P>,
    P: Sortable + 'a,
{
    iter.into_iter()
        .filter_map(|item| {
            if remove_unsorted_attr(item.attrs()) {
                None
            } else {
                Some(item.to_path())
            }
        })
        .collect()
}

fn remove_unsorted_attr(attrs: &mut Vec<Attribute>) -> bool {
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

trait Sortable {
    fn to_path(&self) -> Result<Path>;
    fn attrs(&mut self) -> &mut Vec<Attribute>;
}

impl Sortable for Variant {
    fn to_path(&self) -> Result<Path> {
        Ok(Path {
            segments: vec![self.ident.clone()],
        })
    }
    fn attrs(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }
}

impl Sortable for Field {
    fn to_path(&self) -> Result<Path> {
        Ok(Path {
            segments: vec![self.ident.clone().expect("must be named field")],
        })
    }
    fn attrs(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }
}

impl Sortable for Arm {
    fn to_path(&self) -> Result<Path> {
        // Sort by just the first pat.
        let pat = match &self.pat {
            Pat::Or(pat) => pat.cases.iter().next().expect("at least one pat"),
            _ => &self.pat,
        };

        let segments = match pat {
            Pat::Lit(pat_lit) => match pat_lit.expr.as_ref() {
                syn::Expr::Lit(lit) => match &lit.lit {
                    syn::Lit::Str(s) => Some(vec![Ident::new(&s.value(), self.span())]),
                    _ => None,
                },
                _ => None,
            },
            Pat::Ident(pat) if is_just_ident(pat) => Some(vec![pat.ident.clone()]),
            Pat::Path(pat) => Some(idents_of_path(&pat.path)),
            Pat::Struct(pat) => Some(idents_of_path(&pat.path)),
            Pat::TupleStruct(pat) => Some(idents_of_path(&pat.path)),
            Pat::Wild(pat) => Some(vec![Ident::from(pat.underscore_token)]),
            _ => None,
        };

        if let Some(segments) = segments {
            Ok(Path { segments })
        } else {
            let msg = "unsupported by #[remain::sorted]";
            Err(Error::new_spanned(pat, msg))
        }
    }
    fn attrs(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }
}

fn idents_of_path(path: &syn::Path) -> Vec<Ident> {
    path.segments.iter().map(|seg| seg.ident.clone()).collect()
}

fn is_just_ident(pat: &PatIdent) -> bool {
    pat.by_ref.is_none() && pat.mutability.is_none() && pat.subpat.is_none()
}
