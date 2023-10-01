use crate::engine::state::GameState;
use crate::world::room::Direction;

pub fn look(game_state: &GameState) -> String {
    // Fetch the room blueprint with RoomIdentifier
    if let Some(room_attributes) = game_state.world.rooms.get(&game_state.current_room) {
        room_attributes.default_description.clone()
    } else {
        "Room not found in the world.".to_string()
    }
}

pub fn look_at(game_state: &GameState, obj: &str) -> String {
    format!("You look at the {}.", obj)
}

pub fn move_in_direction(game_state: &mut GameState, direction: Direction) -> Result<String, String> {
    match game_state.world.get_adjacent_room(game_state.current_room, direction) {
        Some(new_room) => {
            game_state.current_room = new_room;
            Ok(game_state.world.get_room_description(game_state.current_room).to_string())
        },
        None => Err(format!("Can't go in the direction of {}.", direction))
    }
}

pub fn open(game_state: &GameState, obj: &str) -> String {
    format!("You try to open the {}.", obj)
}

pub fn close(game_state: &GameState, obj: &str) -> String {
    format!("You try to close the {}.", obj)
}