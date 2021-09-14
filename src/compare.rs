use proc_macro2::Ident;
use std::cmp::Ordering;
use syn::LitStr;

use crate::atom::iter_atoms;

#[derive(Copy, Clone, PartialEq)]
pub enum UnderscoreOrder {
    First,
    Last,
}

pub struct Comparable {
    pub segments: Vec<Box<dyn Segment>>,
}

impl Comparable {
    pub fn of(segment: impl Segment + 'static) -> Comparable {
        Comparable {
            segments: vec![Box::new(segment)],
        }
    }
}

pub trait Segment: quote::ToTokens {
    /// The string representation of these tokens to be used for sorting.
    fn to_string(&self) -> String;
}

impl Segment for Ident {
    fn to_string(&self) -> String {
        ToString::to_string(&self)
    }
}

impl Segment for LitStr {
    fn to_string(&self) -> String {
        self.value().to_string()
    }
}

impl Segment for syn::token::Underscore {
    fn to_string(&self) -> String {
        "_".to_string()
    }
}

pub fn cmp(lhs: &Comparable, rhs: &Comparable, mode: UnderscoreOrder) -> Ordering {
    // Lexicographic ordering across path segments.
    for (lhs, rhs) in lhs.segments.iter().zip(&rhs.segments) {
        match cmp_segment(&lhs.to_string(), &rhs.to_string(), mode) {
            Ordering::Equal => {}
            non_eq => return non_eq,
        }
    }

    lhs.segments.len().cmp(&rhs.segments.len())
}

fn cmp_segment(lhs: &str, rhs: &str, mode: UnderscoreOrder) -> Ordering {
    // Sort `_` last.
    match (lhs, rhs) {
        ("_", "_") => return Ordering::Equal,
        ("_", _) => return Ordering::Greater,
        (_, "_") => return Ordering::Less,
        (_, _) => {}
    }

    let mut lhs_atoms = iter_atoms(lhs);
    let mut rhs_atoms = iter_atoms(rhs);

    // Comparable segments can't be empty.
    let mut left = lhs_atoms.next().unwrap();
    let mut right = rhs_atoms.next().unwrap();

    if mode == UnderscoreOrder::Last {
        // Compare leading underscores.
        match left.underscores().cmp(&right.underscores()) {
            Ordering::Equal => {}
            non_eq => return non_eq,
        }
    }

    loop {
        match left.cmp(&right) {
            Ordering::Equal => {}
            non_eq => return non_eq,
        }

        match (lhs_atoms.next(), rhs_atoms.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(nextl), Some(nextr)) => {
                left = nextl;
                right = nextr;
            }
        }
    }
}
