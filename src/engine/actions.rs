use crate::engine::state::GameState;

pub fn look(game_state: &GameState) -> String {
    game_state.current_room.description.clone()
}