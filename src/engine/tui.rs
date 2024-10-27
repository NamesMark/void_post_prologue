use crate::engine::state::GameState;
use crate::entity::{Entity, EntityId};

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
    format!("Enter one of the following commands:")
}
