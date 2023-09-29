mod engine;
mod parser;

use engine::state::GameState;
use engine::actions;
use parser::command::Command;

fn main() {
    let mut game_state = GameState::new();
    
    loop {
        // Get input from the player
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Parse the input
        match parser::command::parse(&input) {
            Some(Command::Look) => {
                println!("{}", actions::look(&game_state));
            },
            None => {
                println!("I don't understand that command.");
            },
            _ => todo!()
        }
    }
}