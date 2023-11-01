use crate::engine::state::GameState;
use crate::world::room::Direction;

pub fn look(game_state: &GameState) -> String {
    // Fetch the room blueprint with RoomIdentifier
    if let Some(room_attributes) = game_state.world.rooms.get(&game_state.current_room) {
        room_attributes.full_description.clone()
    } else {
        "Room not found in the world. Are you in space? Oh crap".to_string()
    }
}

pub fn look_at(game_state: &GameState, obj: &str) -> String {
    format!("You look at the {}.", obj)
}

pub fn move_in_direction(game_state: &mut GameState, direction: Direction) -> Result<String, String> {
    match game_state.world.get_adjacent_room(&game_state.current_room, direction) {
        Some(new_room) => {
            game_state.current_room = new_room;
            if !game_state.was_current_room_visited() {
                game_state.world.set_visited(&game_state.current_room);
                Ok(format!("{}\n{}", game_state.current_room_first_thoughts(), game_state.current_room_description()))
            } else {
                Ok(game_state.current_room_description().to_string())
            }
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