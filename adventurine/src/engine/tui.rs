use crate::engine::state::GameState;
use crate::entity::EntityId;
use crate::parser::command::Command;
use strum::{EnumMessage, IntoEnumIterator};

pub fn inventory(game_state: &GameState) -> String {
    let mut output = "Your inventory contains:\n".to_string();
    if game_state.inventory.is_empty() {
        output.push_str("nothing.\n");
    } else {
        for item_id in &game_state.inventory {
            if let Some(entity) = game_state.world.entities.get(&EntityId::Item(*item_id)) {
                output.push_str(&entity.name().to_lowercase());
                output.push_str(", ");
            }
        }
        // Remove the last ", " and replace it with "."
        if output.ends_with(", ") {
            output.truncate(output.len() - 2);
            output.push('.');
        }
    }
    output
}

pub fn help() -> String {
    let mut help_message = String::new();
    help_message.push_str("Try looking around, searching for items that might help you, taking a closer look at your surroundings. You'll get it!");
    help_message
}

pub fn list_all_commands() -> String {
    let mut help_message = String::new();
    help_message.push_str("The available commands are:");
    for command in Command::iter() {
        let message = command.get_message().unwrap_or("No description available");
        help_message.push_str(&format!("{}: {}\n", command, message));
    }
    help_message
}
