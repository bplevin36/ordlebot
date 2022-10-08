use std::ops::BitOrAssign;

use comfy_table::presets::{UTF8_BORDERS_ONLY, UTF8_FULL};
use comfy_table::{Table, Row, Cell};
use fxhash::{FxHashMap, FxBuildHasher};
use roaring::RoaringBitmap;

use crate::constants::WORD_LENGTH;
use crate::game_rules::GameRules;
use crate::types::{MatchResult, Color, MatchMapping};
use crate::utils::compute_match;

#[derive(Clone, PartialEq, Debug)]
pub struct Board {
    pub target: String,
    pub guesses: Vec<String>,
    pub matches: Vec<MatchResult>,
    pub possible: RoaringBitmap,
    pub solved: bool,
}

impl Board {
    fn with_target_and_possible(target: String, possible: RoaringBitmap) -> Self {
        Self {
            target: target,
            guesses: Vec::new(),
            matches: Vec::new(),
            possible: possible,
            solved: false,
        }
    }

    fn add_guess(&mut self, guess: String, match_mapping: &MatchMapping) {
        if self.solved {
            return;
        }
        let match_result = compute_match(&guess, &self.target);
        // update possibilities to only include what is possible after this guess
        match match_mapping.get(&guess).expect(&format!("unknown guess {}", guess)).get(&match_result) {
            Some(possibilities) => {
                self.possible &= possibilities;
            },
            None => unreachable!(),
        }
        if guess == self.target {
            self.solved = true;
        }
        self.matches.push(match_result);
        self.guesses.push(guess);
    }

    // Expected value of how many possibilities this guess would eliminate
    fn calculate_possiblity_reduction(&self, guess: &str, match_mapping: &MatchMapping) -> f32 {
        let match_to_possibles = match_mapping.get(guess).expect(&format!("unknown guess {}", guess));
        // for each match result, find unnormalized odds of it happening and number of possibilities it would eliminate
        // we assume that each match result eliminates a disjoint set of possibilities. this is a bad assumption but see how it works for now
        let mut match_to_prob_and_eliminated: FxHashMap<MatchResult, (f32, f32)> = FxHashMap::with_hasher(FxBuildHasher::default());
        for (match_result, possibles) in match_to_possibles.iter() {
            let possible_for_this_match = self.possible.intersection_len(possibles);
            let mut prob = possible_for_this_match as f32 / self.possible.len() as f32; // number of target words that could give us this result / number of target words still possible
            if possible_for_this_match == 0 {
                prob = 0.;
            }
            let possibilities_eliminated = self.possible.len() - possible_for_this_match;
            match_to_prob_and_eliminated.insert(match_result.clone(), (prob, possibilities_eliminated as f32));
        }
        // normalize probability of each match and scale expectation of number of eliminated possibilities
        let mut expected_eliminated: f32 = 0.;
        let prob_sum: f32 = match_to_prob_and_eliminated.values().map(|t| t.0).sum();
        for (prob, n_eliminated) in match_to_prob_and_eliminated.values() {
            let scaled_prob = prob / prob_sum;
            let scaled_eliminated = n_eliminated * scaled_prob;
            expected_eliminated += scaled_eliminated;
        }
        //println!("{:?}", match_to_prob_and_eliminated);
        expected_eliminated
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct GameRun {
    rules: GameRules,
    match_mapping: MatchMapping,
    guesses: Vec<String>,
    boards: Vec<Board>,
}

impl GameRun {
    pub fn new(rules: GameRules, target_words: Vec<String>) -> Self {
        // set up possible sets
        let mut all_possible = RoaringBitmap::new();
        all_possible.insert_range(0u32..rules.words_target.len().try_into().unwrap());

        let mut boards = Vec::with_capacity(rules.num_boards);
        for target in target_words {
            boards.push(Board::with_target_and_possible(target, all_possible.clone()));
        }
        Self {
            rules: rules,
            match_mapping: rules.gen_match_mapping(),
            guesses: Vec::new(),
            boards: boards,
        }
    }

    pub fn add_guess(&mut self, guess: String) {
        for board in self.boards.iter_mut() {
            board.add_guess(guess.clone(), &self.match_mapping);
        }
        self.guesses.push(guess);
    }

    pub fn is_solved(&self) -> bool {
        self.boards.iter().all(|b| b.solved)
    }

    pub fn all_possibility_sets(&self) -> impl Iterator<Item = impl Iterator<Item = &str>> {
        self.boards.iter().map(|board| board.possible.iter().map(|idx| self.rules.words_target[idx as usize]))
    }

    // Overall possibility score; currently just the sum of number of possibilities
    pub fn possibility_score(&self) -> u64 {
        self.boards.iter().map(|board| board.possible.len()).sum()
    }

    // Sum of reduction for all boards
    pub fn calculate_possiblity_reduction(&self, guess: &str) -> f32 {
        self.boards.iter().map(|board| board.calculate_possiblity_reduction(guess, &self.match_mapping)).sum()
    }

    // find the guess that maximizes the number of possibilities eliminated
    pub fn compute_best_guess(&self) -> (&'static str, f32) {
        // check if there are any known answers first
        for board in self.boards.iter() {
            if !board.solved && board.possible.len() == 1 {
                let sol_idx = board.possible.max().unwrap();
                return (self.rules.words_target[sol_idx as usize], 1.0);
            }
        }
        self.rules.words_target.iter().map(|&guess| (guess, self.calculate_possiblity_reduction(guess))).max_by(|(_, r1), (_, r2)| r1.partial_cmp(r2).unwrap()).unwrap()
    }

    pub fn print_state(&self) {
        let mut tab = Table::new();
        tab.load_preset(UTF8_BORDERS_ONLY);
        let mut rows: Vec<Row> = (0..self.guesses.len()).map(|_| Row::new()).collect();

        for (board_i, board) in self.boards.iter().enumerate() {
            for (guess_i, word) in self.guesses.iter().enumerate() {
                let board_row = board_i / 4;
                let row = rows.get_mut(board_row + guess_i).unwrap();
                match board.matches.get(guess_i) {
                    Some(match_result) => {
                        for (letter, letter_color) in word.chars().zip(match_result.iter()) {
                            row.add_cell(Cell::new(letter)
                                .fg(comfy_table::Color::White)
                                .bg(match letter_color {
                                    Color::GREEN => comfy_table::Color::Green,
                                    Color::GREY => comfy_table::Color::DarkGrey,
                                    Color::YELLOW => comfy_table::Color::Yellow,
                                }));
                        }
                    },
                    None => {
                        for _ in 0..WORD_LENGTH {
                            row.add_cell(Cell::new(" "));
                        }
                    }
                }

                // print the current number of possibilities next to the last guess
                if guess_i == board.guesses.len() - 1 {
                    row.add_cell(Cell::new(board.possible.len()));
                } else {
                    row.add_cell(Cell::new("   "));
                }
                // add horizontal separator between boards
                if board_i < self.boards.len() - 1 {
                    row.add_cell(Cell::new(&UTF8_FULL.chars().nth(8).unwrap()));
                }
            }
        }
        tab.add_rows(rows);
        println!("{}", tab);
    }
}
