use crate::{constants::WORD_LENGTH, types::{MatchResult, Color}};

/* Compute the response colors for a guess given the actual */
pub fn compute_match(guess: &str, actual: &str) -> MatchResult{
    let mut matches = [Color::GREY; WORD_LENGTH];
    let guess_bytes = guess.as_bytes();
    let actual_bytes = actual.as_bytes();

    let mut guess_occurrences = [0u8; u8::MAX as usize - 1];
    let mut actual_occurrences = [0u8; u8::MAX as usize - 1];
    for &byte in actual_bytes {
        actual_occurrences[byte as usize] += 1;
    }

    for idx in 0..WORD_LENGTH {
        let guess_char = guess_bytes[idx];
        let actual_char = actual_bytes[idx];
        guess_occurrences[guess_char as usize] += 1;

        if guess_char == actual_char {
            matches[idx] = Color::GREEN;
        } else {
            if actual_occurrences[guess_char as usize] >= guess_occurrences[guess_char as usize] {
                matches[idx] = Color::YELLOW;
            }
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use crate::types::Color;
    use crate::utils::compute_match;

    #[test]
    fn basic_pairs() {
        assert_eq!(compute_match("green", "green"), [Color::GREEN, Color::GREEN, Color::GREEN, Color::GREEN, Color::GREEN]);
        assert_eq!(compute_match("algol", "llola"), [Color::YELLOW, Color::GREEN, Color::GREY, Color::YELLOW, Color::YELLOW]);
        assert_eq!(compute_match("dealt", "teeth"), [Color::GREY, Color::GREEN, Color::GREY, Color::GREY, Color::YELLOW]);
    }

    #[test]
    fn harder_pairs() {
        assert_eq!(compute_match("fffff", "ffffa"), [Color::GREEN, Color::GREEN, Color::GREEN, Color::GREEN, Color::GREY]);
        assert_eq!(compute_match("ffffa", "fffff"), [Color::GREEN, Color::GREEN, Color::GREEN, Color::GREEN, Color::GREY]);
        assert_eq!(compute_match("affff", "ffffa"), [Color::YELLOW, Color::GREEN, Color::GREEN, Color::GREEN, Color::YELLOW]);
    }
}
