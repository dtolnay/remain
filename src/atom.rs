use std::cmp::{Ord, Ordering, PartialOrd};
use std::str;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Atom<'a> {
    /// A sequence of underscores.
    Underscore(usize),
    /// A sequence of digits.
    Number(&'a str),
    /// A sequence of characters.
    Chars(&'a str),
}

impl Atom<'_> {
    pub fn underscores(&self) -> usize {
        match *self {
            Atom::Underscore(n) => n,
            _ => 0,
        }
    }
}

impl PartialOrd for Atom<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Atom<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        use self::Atom::*;

        match (self, other) {
            (Underscore(l), Underscore(r)) => l.cmp(r),
            (Underscore(_), _) => Ordering::Less,
            (_, Underscore(_)) => Ordering::Greater,
            (Number(l), Number(r)) => cmp_numeric(l, r),
            (Number(_), Chars(_)) => Ordering::Less,
            (Chars(_), Number(_)) => Ordering::Greater,
            (Chars(l), Chars(r)) => cmp_ignore_case(l, r),
        }
    }
}

fn cmp_numeric(l: &str, r: &str) -> Ordering {
    // Trim leading zeros.
    let l = l.trim_start_matches('0');
    let r = r.trim_start_matches('0');

    match l.len().cmp(&r.len()) {
        Ordering::Equal => l.cmp(r),
        non_eq => non_eq,
    }
}

fn cmp_ignore_case(l: &str, r: &str) -> Ordering {
    for (a, b) in l.bytes().zip(r.bytes()) {
        match a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase()) {
            Ordering::Equal => match a.cmp(&b) {
                Ordering::Equal => {}
                non_eq => return non_eq,
            },
            non_eq => return non_eq,
        }
    }

    l.len().cmp(&r.len())
}

pub fn iter_atoms(string: &str) -> AtomIter {
    AtomIter {
        bytes: string.as_bytes(),
        offset: 0,
    }
}

pub struct AtomIter<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Iterator for AtomIter<'a> {
    type Item = Atom<'a>;

    fn next(&mut self) -> Option<Atom<'a>> {
        if self.offset >= self.bytes.len() {
            return None;
        }

        let x = self.bytes[self.offset];

        match x {
            b'_' => {
                self.offset += 1;

                let mut n = 1;
                while self.offset < self.bytes.len() {
                    match self.bytes[self.offset] {
                        b'_' => {
                            self.offset += 1;
                            n += 1;
                        }
                        _ => break,
                    }
                }

                Some(Atom::Underscore(n))
            }
            b'0'..=b'9' => {
                let start = self.offset;

                self.offset += 1;
                while self.offset < self.bytes.len() {
                    match self.bytes[self.offset] {
                        b'0'..=b'9' => self.offset += 1,
                        _ => break,
                    }
                }

                let bytes = &self.bytes[start..self.offset];
                let number = str::from_utf8(bytes).expect("valid utf8");
                Some(Atom::Number(number))
            }
            _ => {
                let start = self.offset;

                self.offset += 1;
                while self.offset < self.bytes.len() {
                    match self.bytes[self.offset] {
                        b'_' | b'0'..=b'9' => break,
                        _ => self.offset += 1,
                    }
                }

                let bytes = &self.bytes[start..self.offset];
                let chars = str::from_utf8(bytes).expect("valid utf8");
                Some(Atom::Chars(chars))
            }
        }
    }
}
