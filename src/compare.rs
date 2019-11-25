use proc_macro2::Ident;
use std::cmp::Ordering;

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

// TODO: more intelligent comparison
// for example to handle numeric cases like E9 < E10.
fn cmp_segment(lhs: &str, rhs: &str, mode: UnderscoreOrder) -> Ordering {
    // Sort `_` last.
    match (lhs == "_", rhs == "_") {
        (true, true) => return Ordering::Equal,
        (true, false) => return Ordering::Greater,
        (false, true) => return Ordering::Less,
        (false, false) => {}
    }

    if mode == UnderscoreOrder::Last {
        match count_leading_underscores(lhs).cmp(&count_leading_underscores(rhs)) {
            Ordering::Equal => {}
            non_eq => return non_eq,
        }
    }

    let lhs = lhs.to_ascii_lowercase();
    let rhs = rhs.to_ascii_lowercase();

    // For now: asciibetical ordering.
    lhs.cmp(&rhs)
}

fn count_leading_underscores(segment: &str) -> usize {
    segment.chars().take_while(|c| *c == '_').count()
}
