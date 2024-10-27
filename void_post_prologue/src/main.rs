use std::fmt::Display;

use adventurine::engine::state::GameState;
use adventurine::parser;
use adventurine::world::room::RoomIdentifier;

use tokio::io::{self, AsyncBufReadExt, BufReader};
use tokio::{select, signal};

use voidlogue::process_input;

#[tokio::main]
async fn main() {
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
        select! {
            _ = signal::ctrl_c() => {
                interface.post(&"Bye!");
                std::process::exit(0);
            }
            input = interface.get_input() => {
                // TODO: Add a handler to receive game_lost signal
                if game_state.lost {
                    interface.post(&"Game has concluded. Ciao!");
                    std::process::exit(0);
                }

                // Parse the input to Command
                let parsed_command = parser::command::parse(&input);

                // Act on the Command
                let message = process_input(&mut game_state, parsed_command);

                // Display the game response
                interface.post(&message);
            }
        }
    }
}

#[async_trait::async_trait]
trait Interface {
    async fn get_input(&self) -> String;
    fn post(&self, msg: &dyn Display);
}

struct Cli;

#[async_trait::async_trait]
impl Interface for Cli {
    async fn get_input(&self) -> String {
        let mut input = String::new();
        let stdin = io::stdin();
        let mut reader = BufReader::new(stdin);
        reader.read_line(&mut input).await.unwrap();
        input
    }

    fn post(&self, msg: &dyn Display) {
        println!("{msg}");
    }
}
