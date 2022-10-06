
use ordlebot::{game_rules::TEST, game_run::GameRun};

fn main() {
    let target_words = TEST.get_targets_for_id(215);
    println!("Target words for id 215:\n {:?}", target_words);

    let mut run = GameRun::new(TEST, target_words);
    run.print_state();
    run.add_guess(String::from("STONE"));
    run.add_guess(String::from("GLOWN"));
    run.print_state();
}
