use std::collections::HashMap;

use crate::entity::{Entity, EntityId};
use crate::entity::item::{ItemId};
use crate::world::room::{RoomIdentifier, Access, Direction, PassageType};
use crate::entity::furniture::main_terminal::MainTerminalCommand;
use crate::world::data::World;
use crate::engine::shuttle::ShuttleState;

use strum::IntoEnumIterator;

pub struct GameState {
    pub current_room: RoomIdentifier,
    pub room_states: HashMap<RoomIdentifier, RoomState>,
    pub world: World,                  
    pub inventory: Vec<ItemId>,
    shuttle_state: ShuttleState,
    pub lost: bool,
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

        let inventory = vec![
                ItemId::AssistantCard,
                //ItemId::CaptainCard, // for Debugging
            ];

        GameState {
            current_room: starting_room,
            room_states,
            world,
            inventory,
            shuttle_state: ShuttleState::new(),
            lost: false,
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

    pub fn enter_shuttle_command(&mut self, command: &str) -> Result<String, String> {
        match MainTerminalCommand::from_string(command) {
            Ok(parsed_command) => {
                let result = self.shuttle_state.handle_command(parsed_command);
                if let Ok(ref message) = result {
                    if message.starts_with("You have successfully maneuvered the shuttle for docking.") {
                        if let Some(airlock_room) = self.world.rooms.get_mut(&RoomIdentifier::AirlockA) {
                            airlock_room.connected_rooms = vec!((Direction::North, PassageType::Free, RoomIdentifier::StationAirlock));
                        }
                    }
                }
                result
            }
            Err(e) => {
                if e.starts_with("You are mesmerized") ||
                   e.starts_with("You are thrown") ||
                   e.starts_with("You carefully navigate the shuttle") ||
                   e.starts_with("As if in slow motion") {
                    self.lost = true; 
                }
                Err(e.to_string())
            }
        }
    }
}