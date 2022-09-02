use proc_macro2::Ident;
use std::cmp::Ordering;

use crate::{atom::iter_atoms, check::Category};

#[derive(Copy, Clone, PartialEq)]
pub enum UnderscoreOrder {
    First,
    Last,
}

pub struct Path {
    pub segments: Vec<Ident>,
}

pub fn cmp(lhs: &(Category, Path), rhs: &(Category, Path), mode: UnderscoreOrder) -> Ordering {
    match lhs.0.cmp(&rhs.0) {
        Ordering::Equal => {}
        non_eq => return non_eq,
    };

    // Lexicographic ordering across path segments.
    for (lhs, rhs) in lhs.1.segments.iter().zip(&rhs.1.segments) {
        match cmp_segment(&lhs.to_string(), &rhs.to_string(), mode) {
            Ordering::Equal => {}
            non_eq => return non_eq,
        }
    }

    lhs.1.segments.len().cmp(&rhs.1.segments.len())
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

    // Path segments can't be empty.
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
