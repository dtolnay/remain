use std::cmp::{Ord, Ordering, PartialOrd};
use std::str::from_utf8_unchecked;

#[derive(Eq, PartialEq)]
pub enum Atom<'a> {
    /// A sequence of digits.
    Number(u64),
    /// A sequence of characters.
    Chars(&'a str),
    /// A sequence of underscores.
    Underscore(usize),
}

impl<'a> PartialOrd<Atom<'a>> for Atom<'a> {
    fn partial_cmp(&self, other: &Atom<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Atom<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        use Atom::*;

        match (self, other) {
            (Chars(l), Chars(r)) => cmp_ignore_case(l, r),
            (Chars(_), _) => Ordering::Less,
            (_, Chars(_)) => Ordering::Greater,
            (Number(l), Number(r)) => l.cmp(r),
            (Number(_), _) => Ordering::Less,
            (_, Number(_)) => Ordering::Greater,
            (Underscore(l), Underscore(r)) => l.cmp(r),
        }
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

// https://tools.ietf.org/html/rfc3629
#[cfg_attr(rustfmt, rustfmt_skip)]
static UTF8_CHAR_WIDTH: [u8; 256] = [
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x1F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x3F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x5F
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
    1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, // 0x7F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0x9F
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, // 0xBF
    0,0,2,2,2,2,2,2,2,2,2,2,2,2,2,2,
    2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, // 0xDF
    3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3, // 0xEF
    4,4,4,4,4,0,0,0,0,0,0,0,0,0,0,0, // 0xFF
];

impl<'a> Iterator for AtomIter<'a> {
    type Item = Atom<'a>;

    fn next(&mut self) -> Option<Atom<'a>> {
        // No more characters.
        if self.offset >= self.bytes.len() {
            return None;
        }

        let x = self.bytes[self.offset];

        // Underscore or Number.
        match x {
            // Underscore.
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
            // Number.
            b'0'..=b'9' => {
                let start = self.offset;

                self.offset += 1;
                while self.offset < self.bytes.len() {
                    match self.bytes[self.offset] {
                        b'0'..=b'9' => self.offset += 1,
                        _ => break,
                    }
                }

                // This is safe since we've verified it's valid utf8.
                let s = &self.bytes[start..self.offset];
                let s = unsafe { from_utf8_unchecked(s) };

                let int = match s.parse::<u64>() {
                    Ok(n) => n,
                    _ => unreachable!("expected integer"),
                };

                Some(Atom::Number(int))
            }
            // Don't care.
            _ => {
                let start = self.offset;

                self.offset += 1;
                while self.offset < self.bytes.len() {
                    match self.bytes[self.offset] {
                        b'_' | b'0'..=b'9' => break,
                        b => {
                            // Use a copy of the UTF8_CHAR_WIDTH array from core::str to avoid
                            // relying on unstable internals.
                            self.offset += match UTF8_CHAR_WIDTH[b as usize] {
                                0 => unreachable!("expected valid utf8"),
                                width => width as usize,
                            };
                        }
                    }
                }

                let s = if self.offset <= self.bytes.len() {
                    &self.bytes[start..self.offset]
                } else {
                    &self.bytes[start..self.bytes.len()]
                };
                let s = unsafe { from_utf8_unchecked(s) };

                return Some(Atom::Chars(s));
            }
        }
    }
}
