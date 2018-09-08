#![no_std]

extern crate fst;

use fst::Automaton;

/// An automaton that matches if the input contains a specific subsequence
/// ignoring ASCII case.
///
/// It is similar to `fst::automaton::Subsequence`, and can be used to build
/// a simple fuzzy-finder for ASCII-only content.
#[derive(Clone, Debug)]
pub struct SubseqAsciiCaseless<'a> {
    subseq: &'a [u8],
}

impl<'a> SubseqAsciiCaseless<'a> {
    /// Constructs automaton that matches input containing the
    /// specified subsequence ignoring ASCII case.
    ///
    /// # Panics
    ///
    /// Panics if `subseq` contains any ASCII uppercase character.
    pub fn new(subseq: &'a str) -> Self {
        assert!(!subseq.bytes().any(|b| b.is_ascii_uppercase()));
        Self::new_unchecked(subseq)
    }

    /// Same as `new()` but don't check the string.
    ///
    /// It would never match if the subseq contains any ASCII uppercase
    /// character.
    pub fn new_unchecked(subseq: &'a str) -> Self {
        SubseqAsciiCaseless {
            subseq: subseq.as_bytes(),
        }
    }
}

impl<'a> Automaton for SubseqAsciiCaseless<'a> {
    type State = usize;

    fn start(&self) -> usize {
        0
    }

    fn is_match(&self, state: &usize) -> bool {
        *state == self.subseq.len()
    }

    fn can_match(&self, _: &usize) -> bool {
        true
    }

    fn will_always_match(&self, state: &usize) -> bool {
        self.is_match(state)
    }

    fn accept(&self, state: &usize, byte: u8) -> usize {
        if self.is_match(state) {
            return *state;
        }
        state + (byte.to_ascii_lowercase() == self.subseq[*state]) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PATTERN1: &str = "sqaicl";

    #[test]
    fn test_states() {
        let subseq = SubseqAsciiCaseless::new(PATTERN1);
        assert_eq!(subseq.start(), 0);
        assert_eq!(subseq.accept(&0, b'q'), 0);
        assert_eq!(subseq.accept(&0, b'S'), 1);
        assert_eq!(subseq.accept(&1, b'x'), 1);
        assert_eq!(subseq.accept(&1, b'q'), 2);
        assert_eq!(subseq.accept(&2, b'a'), 3);
        assert_eq!(subseq.accept(&3, b'I'), 4);
        assert_eq!(subseq.accept(&4, b'l'), 4);
        assert_eq!(subseq.accept(&4, b'C'), 5);
        assert_eq!(subseq.accept(&5, b'L'), 6);
    }

    #[test]
    fn test_is_match() {
        let subseq = SubseqAsciiCaseless::new(PATTERN1);
        for i in 0..PATTERN1.len() {
            assert!(!subseq.is_match(&i));
        }
        assert!(subseq.is_match(&PATTERN1.len()));
    }

    #[test]
    #[should_panic]
    fn test_new_check() {
        SubseqAsciiCaseless::new("SqAiCl");
    }
}
