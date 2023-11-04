use std::collections::HashMap;

use crate::entity::{Entity, EntityId};
use crate::entity::item::{ItemId};
use crate::world::room::{RoomIdentifier, Access};
use crate::world::data::World;

use strum::IntoEnumIterator;

pub struct GameState {
    pub current_room: RoomIdentifier,
    pub room_states: HashMap<RoomIdentifier, RoomState>,
    pub world: World,                  
    pub inventory: Vec<ItemId>
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

        let inventory = vec![ItemId::AssistantCard];

        GameState {
            current_room: starting_room,
            room_states,
            world,
            inventory
        }
    }

    pub fn was_current_room_visited(&self) -> bool {
        self.world.was_visited(&self.current_room)
    }
    pub fn current_room_description(&self) -> &str {
        self.world.get_room_short_description(&self.current_room)
    }
    pub fn current_room_first_thoughts(&self) -> &str {
        self.world.get_room_first_thoughts(&self.current_room)
    }

    pub fn current_room_entities(&self) -> Option<&Vec<EntityId>> {
        self.world.get_room_entities(&self.current_room)
    }

    pub fn get_player_access(&self) -> &Access {
        todo!("Check the highest access level card in the inventory")
    }
}