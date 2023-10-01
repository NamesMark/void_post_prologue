use crate::engine::state::GameState;

pub fn look(game_state: &GameState) -> String {
    // Fetch the room blueprint with RoomIdentifier
    if let Some(room_blueprint) = game_state.world.rooms.get(&game_state.current_room) {
        room_blueprint.description().to_string()
    } else {
        "Room not found in the world.".to_string()
    }
}