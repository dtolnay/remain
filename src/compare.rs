use proc_macro2::Ident;
use std::cmp::Ordering;

pub struct Path {
    pub segments: Vec<Ident>,
}

pub fn cmp(lhs: &Path, rhs: &Path) -> Ordering {
    // Lexicographic ordering across path segments.
    for (lhs, rhs) in lhs.segments.iter().zip(&rhs.segments) {
        match cmp_segment(&lhs.to_string(), &rhs.to_string()) {
            Ordering::Equal => {}
            non_eq => return non_eq,
        }
    }

    lhs.segments.len().cmp(&rhs.segments.len())
}

// TODO: more intelligent comparison
// for example to handle numeric cases like E9 < E10.
fn cmp_segment(lhs: &str, rhs: &str) -> Ordering {
    // Sort `_` last.
    match (lhs == "_", rhs == "_") {
        (true, true) => return Ordering::Equal,
        (true, false) => return Ordering::Greater,
        (false, true) => return Ordering::Less,
        (false, false) => {}
    }

    let lhs = lhs.to_ascii_uppercase();
    let rhs = rhs.to_ascii_uppercase();

    // For now: asciibetical ordering.
    lhs.cmp(&rhs)
}
