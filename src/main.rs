
use ordlebot::{game_rules::{WORDLE, DORDLE, QUORDLE, DUOTRIGORDLE}, game_run::GameRun};

fn main() {
    let run_id = 219;
    let run_rules = WORDLE;
    let target_words = run_rules.get_targets_for_id(run_id);

    let mut run = GameRun::new(run_rules, vec![String::from("DANDY")]);
    for guess_num in 0..run_rules.num_guesses {
        let (guess, reduction) = run.compute_best_guess();
        println!("Guess '{}' will eliminate {} possibilities; guessing...", guess, reduction);
        run.add_guess(guess.to_owned());
        run.print_state();
        if run.is_solved() {
            println!("Victory in {} guesses!", guess_num + 1);
            break;
        }
    }
    if !run.is_solved() {
        println!("FAILURE TO SOLVE");
    }
}
