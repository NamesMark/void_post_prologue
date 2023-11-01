#[macro_use]
mod utils;

mod engine;
mod parser;
mod world;
mod entity;

//use world::World;
use world::room::RoomIdentifier;
use world::room::Direction;

use engine::state::GameState;
use engine::actions;
use parser::command::Command;
use rand::prelude::SliceRandom;

fn main() {
    let mut game_state = GameState::new(RoomIdentifier::Storage);

    println!("{}", game_state.current_room_first_thoughts());
    println!("{}", game_state.current_room_description());
    
    loop {
        // Get input from the player
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        // Parse the input
        process_input(&mut game_state, parser::command::parse(&input));
    }
}

fn process_input(game_state: &mut GameState, command: Option<Command>) {
    match command {
        Some(Command::Look(None)) => {
            println!("{}", actions::look(&game_state));
        },

        Some(Command::Go(direction)) => handle_movement(game_state, direction),

        Some(Command::Look(obj)) => println!("{}", actions::look_at(game_state, &obj.unwrap_or(String::from("I'm not sure where to look at")))),

        Some(Command::Open(obj)) => println!("{}", actions::open(game_state, &obj)),
        Some(Command::Close(obj)) => println!("{}", actions::close(game_state, &obj)),
                    
        _ => print_any!("Erm, say that again?", 
                        "I am not sure I follow", 
                        "Put what where? Huh?", 
                        "I think I keep hearing voices, maybe I shouldn't have sent those vitnesses of Mandalor away after all.")    
    }
}

fn handle_movement(game_state: &mut GameState, direction: Direction) {
    // This function will handle the movement logic.
    match actions::move_in_direction(game_state, direction) {
        Ok(new_description) => {
            println!("{}", new_description);
        },
        Err(error) => {
            print_any!(format!("I can't go to {}: {}", direction, error),
                       format!("There's nowhere to go at {}, I think?", direction),
                       format!("I could try, I remember walking through solid walls worked at a certain spaceport platform to get on a spacecruiser to the nanoscience school for gifted kids... or was it in a holoseries?..")
        );
        },
    }
}