use comfy_table::presets::UTF8_BORDERS_ONLY;
use comfy_table::{Table, Row, Cell};

use crate::game_rules::GameRules;
use crate::types::{MatchResult, Color};
use crate::utils::compute_match;


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameRun {
    rules: GameRules,
    target_words: Vec<String>,
    guesses: Vec<String>,
    boards_matches: Vec<Vec<MatchResult>>,
}

impl GameRun {
    pub fn new(rules: GameRules, target_words: Vec<String>) -> Self {
        let mut board_matches = Vec::with_capacity(rules.num_boards);
        board_matches.extend((0..rules.num_boards).map(|_| Vec::new()));
        Self {
            rules: rules,
            target_words: target_words,
            guesses: Vec::new(),
            boards_matches: board_matches,
        }
    }

    pub fn add_guess(&mut self, guess: String) {
        for (board_matches, target) in self.boards_matches.iter_mut().zip(self.target_words.iter()) {
            if let Some(last_match) = board_matches.last() {
                if *last_match == [Color::GREEN, Color::GREEN, Color::GREEN, Color::GREEN, Color::GREEN] {
                    continue;
                }
            }
            let match_result = compute_match(&guess, target);
            board_matches.push(match_result);
        }
        self.guesses.push(guess);
    }

    pub fn print_state(&self) {
        let mut tab = Table::new();
        tab.load_preset(UTF8_BORDERS_ONLY);
        let mut rows: Vec<Row> = (0..self.guesses.len()).map(|_| Row::new()).collect();

        for (board_i, board) in self.boards_matches.iter().enumerate() {
            for (guess_i, (word, match_result)) in self.guesses.iter().zip(board.iter()).enumerate() {
                let board_row = board_i / 4;
                let row = rows.get_mut(board_row + guess_i).unwrap();
                for (letter, letter_color) in word.chars().zip(match_result.iter()) {
                    row.add_cell(Cell::new(letter)
                        .fg(comfy_table::Color::White)
                        .bg(match letter_color {
                            Color::GREEN => comfy_table::Color::Green,
                            Color::GREY => comfy_table::Color::DarkGrey,
                            Color::YELLOW => comfy_table::Color::Yellow,
                        }));
                }
                row.add_cell(Cell::new("|"));
            }
        }
        tab.add_rows(rows);
        println!("{}", tab);
    }
}
