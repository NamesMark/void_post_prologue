use std::collections::HashMap;

use crate::world::room::RoomIdentifier;
use crate::world::data::World;

pub struct GameState {
    pub current_room: RoomIdentifier,
    pub room_states: HashMap<RoomIdentifier, RoomState>,
    pub world: World,                  
}

pub struct RoomState {
    pub is_explored: bool,
    // Other dynamic attributes like taken items, flipped switches, etc.
}

impl GameState {
    pub fn new(starting_room: RoomIdentifier) -> Self {
        let world = World::initialize();

        let mut room_states: HashMap<RoomIdentifier, RoomState> = HashMap::new();
        
        // Initialize room states with the default values.
        for room_id in RoomIdentifier::iter() {
            room_states.insert(room_id, RoomState { is_explored: false });
        }

        GameState {
            current_room: starting_room,
            room_states,
            world,
        }
    }

    pub fn current_room_description(&self) -> &str {
        self.world.get_room_description(self.current_room)
    }
}