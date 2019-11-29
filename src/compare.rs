use proc_macro2::Ident;
use std::cmp::Ordering;

use crate::atom::{iter_atoms, Atom, AtomIter};

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

    let (mut left, mut right) = match next_or_ordering(&mut lhs_atoms, &mut rhs_atoms) {
        Ok(next) => next,
        Err(ord) => return ord,
    };

    // Leading underscores.
    if mode == UnderscoreOrder::Last {
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

        match next_or_ordering(&mut lhs_atoms, &mut rhs_atoms) {
            Ok((l, r)) => {
                left = l;
                right = r;
            }
            Err(ord) => return ord,
        };
    }
}

#[inline]
fn next_or_ordering<'a>(
    lhs_atoms: &mut AtomIter<'a>,
    rhs_atoms: &mut AtomIter<'a>,
) -> Result<(Atom<'a>, Atom<'a>), Ordering> {
    match (lhs_atoms.next(), rhs_atoms.next()) {
        (None, None) => return Err(Ordering::Equal),
        (None, Some(_)) => return Err(Ordering::Greater),
        (Some(_), None) => return Err(Ordering::Less),
        (Some(left), Some(right)) => Ok((left, right)),
    }
}
