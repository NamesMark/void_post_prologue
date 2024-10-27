use std::fmt::Display;

use voidlogue::engine::state::GameState;
use voidlogue::parser;
use voidlogue::process_input;
use voidlogue::world::room::RoomIdentifier;

fn main() {
    let mut game_state = GameState::new(RoomIdentifier::Storage);

    // TODO: configure
    let is_cli = true;

    let interface: Box<dyn Interface> = if is_cli {
        Box::new(Cli)
    } else {
        unimplemented!("Only CLI is available for now")
    };

    interface.post(&game_state.current_room_first_thoughts());
    interface.post(&game_state.current_room_description());

    loop {
        // if let Ok(_) = rx.try_recv() {
        //     interface.post(&"Bye!");
        //     break;
        // }

        if game_state.lost {
            interface.post(&"Game has concluded. Ciao!");
            std::process::exit(0);
        }

        // Get input from the player
        let input = interface.get_input();

        // Parse the input to Command
        let parsed_command = parser::command::parse(&input);

        // Act on the Command
        let message = process_input(&mut game_state, parsed_command);

        // Display the game response
        interface.post(&message);
    }
}

trait Interface {
    fn get_input(&self) -> String;
    fn post(&self, msg: &dyn Display);
}

struct Cli;

impl Interface for Cli {
    fn get_input(&self) -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
    }

    fn post(&self, msg: &dyn Display) {
        println!("{msg}");
    }
}
