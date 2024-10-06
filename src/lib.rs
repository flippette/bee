#![feature(portable_simd)]
#![forbid(unsafe_code)]
#![no_std]

use core::simd::{prelude::*, LaneCount, SupportedLaneCount};

/// Check if a word fits a given set of characters.
///
/// Equivalent to this, but faster:
///
/// ```rust
/// word.contains(&set[0]) && word.iter().all(|ch| set.contains(&ch))
/// ```
#[inline]
pub fn fits<const N: usize>(word: &[u8], set: &[u8]) -> bool
where
    LaneCount<N>: SupportedLaneCount,
{
    // Check if a word contains a character.
    #[inline]
    fn word_contains_char<const N: usize>(word: &[u8], ch: u8) -> bool
    where
        LaneCount<N>: SupportedLaneCount,
    {
        let ss = Simd::<u8, N>::splat(ch);

        // we just need _any_ byte in each chunk to satisfy, so `chunks_exact`
        // is unnecessary here
        word.chunks(N)
            .map(Simd::<u8, N>::load_or_default)
            .any(|sw| sw.simd_eq(ss).any())
    }

    // Check if a word only has characters from a set.
    #[inline]
    fn word_only_set_chars<const N: usize>(word: &[u8], set: &[u8]) -> bool
    where
        LaneCount<N>: SupportedLaneCount,
    {
        word.iter().all(|ch| word_contains_char(set, *ch))
    }

    word_contains_char(word, set[0]) && word_only_set_chars(word, set)
}
