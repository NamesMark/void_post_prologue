#[macro_use]
mod utils;

mod engine;
mod entity;
mod parser;
mod world;

use std::fmt::Display;

//use world::World;
use world::room::Direction;
use world::room::RoomIdentifier;

use engine::actions;
use engine::state::GameState;
use engine::tui;
use parser::command::Command;
use rand::prelude::SliceRandom;

use voidlogue::process_input;

fn main() {
    let mut game_state = GameState::new(RoomIdentifier::Storage);

    println!("{}", game_state.current_room_first_thoughts());
    println!("{}", game_state.current_room_description());

    loop {
        if game_state.lost {
            std::process::exit(0);
        }

        // Get input from the player
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Parse the input
        let result = process_input(&mut game_state, parser::command::parse(&input));
        if let Some(game_message) = result {
            display_message(game_message);
        }
    }
}

fn display_message(msg: impl Display) {
    println!("{msg}");
}
