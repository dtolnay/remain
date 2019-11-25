use proc_macro2::Ident;
use std::cmp::Ordering;

use crate::atom::{iter_atoms, Atom};

#[derive(Copy, Clone, PartialEq)]
pub enum UnderscoreOrder {
    First,
    Last,
}

pub struct Path {
    pub segments: Vec<Ident>,
}

pub fn cmp(lhs: &Path, rhs: &Path, mode: UnderscoreOrder) -> Ordering {
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
    match (lhs == "_", rhs == "_") {
        (true, true) => return Ordering::Equal,
        (true, false) => return Ordering::Greater,
        (false, true) => return Ordering::Less,
        (false, false) => {}
    }

    let mut lhs_atoms = iter_atoms(&lhs);
    let mut rhs_atoms = iter_atoms(&rhs);

    loop {
        let left = lhs_atoms.next();
        let right = rhs_atoms.next();

        let (left, right) = match (left, right) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Greater,
            (Some(_), None) => return Ordering::Less,
            (Some(left), Some(right)) => (left, right),
        };

        // Compare underscores with respect to mode.
        match (&left, &right) {
            (Atom::Underscore(l), Atom::Underscore(r)) => match l.cmp(&r) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            },
            (Atom::Underscore(_), _) if mode == UnderscoreOrder::First => return Ordering::Greater,
            (Atom::Underscore(_), _) => return Ordering::Less,
            (_, Atom::Underscore(_)) if mode == UnderscoreOrder::First => return Ordering::Less,
            (_, Atom::Underscore(_)) => return Ordering::Greater,
            _ => match left.cmp(&right) {
                Ordering::Equal => continue,
                non_eq => return non_eq,
            },
        }
    }
}
