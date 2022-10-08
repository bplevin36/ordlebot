use crate::constants::WORD_LENGTH;
use fxhash::FxHashMap;
use roaring::RoaringBitmap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Color {
    GREEN,
    GREY,
    YELLOW,
}

// The colored result of making a guess
pub type MatchResult = [Color; WORD_LENGTH];

// map of guessed word -> match result -> set of possible targets
pub type MatchMapping = FxHashMap<String, FxHashMap<MatchResult, RoaringBitmap>>;


