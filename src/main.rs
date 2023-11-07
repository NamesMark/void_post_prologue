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
use engine::tui;
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

        // Look(Option<String>),  // Look at something specific
        // Examine(Option<String>),  // Examine something in detail
        Some(Command::Look(None)) => {
            println!("{}", actions::look(&game_state));
        },

        Some(Command::Look(obj)) => println!("{}", actions::look_at(game_state, &obj.unwrap_or(String::from("I'm not sure where to look at")))),
        
        
        // Open(String),         // Open something
        // Close(String),        // Close something
        Some(Command::Open(obj)) => println!("{}", actions::open(game_state, &obj)),
        Some(Command::Close(obj)) => println!("{}", actions::close(game_state, &obj)),

        
        // Go(Direction),        // Move in a direction
        Some(Command::Go(direction)) => handle_movement(game_state, direction),
        // Enter(String),        // Enter something
        
        // // Interaction
        // Take(String),         // Take an object
        Some(Command::Take(obj)) => println!("{}", actions::pick_up(game_state, &obj)),
        // Drop(String),         // Drop an object
        Some(Command::Drop(obj)) => println!("{}", actions::drop(game_state, &obj)),
        //TakeFrom(String, String), // Take from a container
        Some(Command::TakeFrom(obj, cont)) => println!("{}", actions::take_from_container(game_state, &obj, &cont)),
        //PutInto(String, String), // Put into a container
        Some(Command::PutInto(obj, cont)) => println!("{}", actions::put_into(game_state, &obj, &cont)),
        // Use(String),          // Use an object
        // Combine(String, String), // Combine two items
        // Push(String),         // Push something
        // Pull(String),         // Pull something
        // Turn(String),         // Turn something (like a knob or switch)
        // Read(String),         // Read something (like a note)
        Some(Command::Read(obj)) => println!("{}", actions::read(game_state, &obj)),
        // Eat(String),           // Eat something that's a food
        Some(Command::Eat(obj)) => println!("{}", actions::eat(game_state, &obj)),
        
        // // Communication
        // // TalkTo(String),       // Talk to a character
        // // Give(String, String), // Give an item to someone
        
        // // Inventory & status
        // Inventory,            // Check your items
        Some(Command::Inventory) => println!("{}", tui::inventory(game_state)),
        // Help
        Some(Command::Help) => println!("{}", tui::help()),
        // Status,               // Check player's status or health
        
        // // Misc
        // Help,                 // Show available commands
        // Save,                 // Save the game
        // Load,                 // Load the game
                    
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