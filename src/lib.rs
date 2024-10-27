#[macro_use]
mod utils;

mod engine;
mod entity;
mod parser;
mod world;

//use world::World;
use world::room::Direction;
use world::room::RoomIdentifier;

use engine::actions;
use engine::state::GameState;
use engine::tui;
use parser::command::Command;
use rand::prelude::SliceRandom;

pub fn process_input(game_state: &mut GameState, command: Option<Command>) -> String {
    match command {

        // Look(Option<String>),  // Look at something specific
        // Examine(Option<String>),  // Examine something in detail
        Some(Command::Look(None)) => {
            actions::look(&game_state)
        },

        Some(Command::Look(obj)) => actions::look_at(game_state, &obj.unwrap_or(String::from("I'm not sure where to look at"))),
        
        
        // Open(String),         // Open something
        // Close(String),        // Close something
        Some(Command::Open(obj)) => actions::open(game_state, &obj),
        Some(Command::Close(obj)) => actions::close(game_state, &obj),

        
        // Go(Direction),        // Move in a direction
        Some(Command::Go(direction)) => {handle_movement(game_state, direction); "".to_string()},
        // Enter(String),        // Enter something
        
        // // Interaction
        // Take(String),         // Take an object
        Some(Command::Take(obj)) => actions::pick_up(game_state, &obj),
        // Drop(String),         // Drop an object
        Some(Command::Drop(obj)) => actions::drop(game_state, &obj),
        //TakeFrom(String, String), // Take from a container
        Some(Command::TakeFrom(obj, cont)) => actions::take_from_container(game_state, &obj, &cont),
        //PutInto(String, String), // Put into a container
        Some(Command::PutInto(obj, cont)) => actions::put_into(game_state, &obj, &cont),
        // Use(String),          // Use an object
        Some(Command::Use(obj)) => actions::r#use(game_state, &obj),
        // Enter(String)         // Enter a command
        Some(Command::Enter(command)) => actions::enter(game_state, &command),
        // Combine(String, String), // Combine two items
        // Push(String),         // Push something
        // Pull(String),         // Pull something
        // Turn(String),         // Turn something (like a knob or switch)
        // Read(String),         // Read something (like a note)
        Some(Command::Read(obj)) => actions::read(game_state, &obj),
        // Eat(String),           // Eat something that's a food
        Some(Command::Eat(obj)) => actions::eat(game_state, &obj),
        
        // // Communication
        // // TalkTo(String),       // Talk to a character
        // // Give(String, String), // Give an item to someone
        
        // // Inventory & status
        // Inventory,            // Check your items
        Some(Command::Inventory) => tui::inventory(game_state),
        // Help
        Some(Command::Help) => tui::help(),
        // Status,               // Check player's status or health
        
        // // Misc
        // Save,                 // Save the game
        // Load,                 // Load the game
                    
        _ => any_of!("Erm, say that again?", 
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
        }
        Err(error) => {
            print_any!(format!("I can't go to {}: {}", direction, error),
                       format!("There's nowhere to go at {}, I think?", direction),
                       format!("I could try, I remember walking through solid walls worked at a certain spaceport platform to get on a spacecruiser to the nanoscience school for gifted kids... or was it in a holoseries?..")
        );
        }
    }
}
