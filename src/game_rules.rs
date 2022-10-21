
use fxhash::{FxHashMap, FxBuildHasher};
use mersenne_twister_m::MT19937::MT19937;
use roaring::RoaringBitmap;

use crate::types::MatchMapping;
use crate::utils::compute_match;

pub const DUOTRIGORDLE: GameRules = include!("../data/duotrigordle.rs");
pub const QUORDLE: GameRules = DUOTRIGORDLE.with_boards_and_guesses(4, 9);
pub const DORDLE: GameRules = DUOTRIGORDLE.with_boards_and_guesses(2, 7);
pub const WORDLE: GameRules = DUOTRIGORDLE.with_boards_and_guesses(1, 6);
pub const TEST: GameRules = include!("../data/test.rs");



#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GameRules {
    pub num_boards: usize,
    pub num_guesses: usize,
    pub alphabet: &'static[&'static str],
    pub words_target: &'static[&'static str],
    pub words_valid: &'static[&'static str],
}

impl GameRules {
    const fn with_boards_and_guesses(self, boards: usize, guesses: usize) -> Self {
        let mut rules = self;
        rules.num_boards = boards;
        rules.num_guesses = guesses;
        rules
    }

    // Ported from https://github.com/thesilican/duotrigordle/blob/719aff6fdc2f0b1504be426f163b993c8fdb6261/src/funcs.ts#L93
    pub fn get_targets_for_id(self, id: usize) -> Vec<String> {
        // Temporary, for migration of GIPSY/GYPSY
        // WORDS_TARGET will be updated after daily duotrigordle 188
        let mut target_pool = self.words_target.to_owned();
        if id > 187 {
            for word in ["GIPSY", "GYPSY"] {
                if let Some(idx) = target_pool.iter().position(|w| *w == word) {
                    target_pool.remove(idx);
                }
            }
        }
        let mut target_words: Vec<String> = Vec::with_capacity(self.num_boards);
        let mut rng = MT19937::new_with_seed(id as u32);

        while target_words.len() < self.num_boards {
            let idx = rng.genrand() as usize % target_pool.len();
            let word = target_pool[idx].to_owned();
            if !target_words.contains(&word) {
                target_words.push(word);
            }
        }
        target_words
    }

    pub fn gen_match_mapping(&self) -> MatchMapping {
        let mut pattern_mapping = FxHashMap::with_capacity_and_hasher(self.words_valid.len(), FxBuildHasher::default());
        for &word in self.words_valid {
            let mut outcome_mapping = FxHashMap::with_hasher(FxBuildHasher::default());
            for (target_i, &target_word) in self.words_target.iter().enumerate() {
                let response = compute_match(word, target_word);
                outcome_mapping.entry(response)
                    .or_insert_with(|| RoaringBitmap::new())
                    .insert(target_i as u32);
            }
            pattern_mapping.insert(word.to_owned(), outcome_mapping);
        }
        pattern_mapping
    }
}
