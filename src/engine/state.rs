pub struct GameState {
    pub current_room: Room,
}

pub struct Room {
    pub description: String,
    pub is_explored: bool, 
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            current_room: Room {
                description: "You wake up on the steel floor. If you laid there any longer, you'd probably get arthritis, you think. Now, where are we?".to_string(),
                is_explored: false,
            },
        }
    }
}