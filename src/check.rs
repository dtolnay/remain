use quote::quote;
use std::cmp::Ordering;
use syn::{Arm, Attribute, Result, Variant};
use syn::{Error, Field, Pat, PatIdent};

use crate::compare::{cmp, Comparable, Segment, UnderscoreOrder};
use crate::format;
use crate::parse::Input::{self, *};

pub fn sorted(input: &mut Input) -> Result<()> {
    let paths = match input {
        Enum(item) => collect_comparables(&mut item.variants)?,
        Struct(item) => collect_comparables(&mut item.fields)?,
        Match(expr) | Let(expr) => collect_comparables(&mut expr.arms)?,
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

fn find_misordered(paths: &[Comparable], mode: UnderscoreOrder) -> Option<usize> {
    for i in 1..paths.len() {
        if cmp(&paths[i], &paths[i - 1], mode) == Ordering::Less {
            return Some(i);
        }
    }

    None
}

fn collect_comparables<'a, I, P>(iter: I) -> Result<Vec<Comparable>>
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
    fn to_path(&self) -> Result<Comparable>;
    fn attrs(&mut self) -> &mut Vec<Attribute>;
}

impl Sortable for Variant {
    fn to_path(&self) -> Result<Comparable> {
        Ok(Comparable::of(self.ident.clone()))
    }
    fn attrs(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }
}

impl Sortable for Field {
    fn to_path(&self) -> Result<Comparable> {
        Ok(Comparable::of(
            self.ident.as_ref().expect("must be named field").clone(),
        ))
    }
    fn attrs(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }
}

impl Sortable for Arm {
    fn to_path(&self) -> Result<Comparable> {
        // Sort by just the first pat.
        let pat = match &self.pat {
            Pat::Or(pat) => pat.cases.iter().next().expect("at least one pat"),
            _ => &self.pat,
        };

        let segments: Option<Comparable> = match pat {
            Pat::Lit(pat_lit) => match pat_lit.expr.as_ref() {
                syn::Expr::Lit(lit) => match &lit.lit {
                    syn::Lit::Str(s) => Some(Comparable::of(s.clone())),
                    _ => None,
                },
                _ => None,
            },
            Pat::Ident(pat) if is_just_ident(pat) => Some(Comparable::of(pat.ident.clone())),
            Pat::Path(pat) => Some(comparables_of_path(&pat.path)),
            Pat::Struct(pat) => Some(comparables_of_path(&pat.path)),
            Pat::TupleStruct(pat) => Some(comparables_of_path(&pat.path)),
            Pat::Wild(pat) => Some(Comparable::of(pat.underscore_token)),
            _ => None,
        };

        if let Some(segments) = segments {
            Ok(segments)
        } else {
            let msg = "unsupported by #[remain::sorted]";
            Err(Error::new_spanned(pat, msg))
        }
    }
    fn attrs(&mut self) -> &mut Vec<Attribute> {
        &mut self.attrs
    }
}

fn comparables_of_path(path: &syn::Path) -> Comparable {
    let mut segments: Vec<Box<dyn Segment>> = vec![];

    for seg in path.segments.iter() {
        segments.push(Box::new(seg.ident.clone()));
    }

    Comparable { segments }
}

fn is_just_ident(pat: &PatIdent) -> bool {
    pat.by_ref.is_none() && pat.mutability.is_none() && pat.subpat.is_none()
}
