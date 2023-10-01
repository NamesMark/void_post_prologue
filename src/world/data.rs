use std::collections::HashMap;
use super::room::{RoomAttributes, RoomBlueprint, RoomIdentifier, Direction};

pub struct World {
    pub rooms: HashMap<RoomIdentifier, RoomAttributes>,
}

impl World {
    pub fn initialize() -> Self {
        let mut rooms = HashMap::new();
        
        // Populate rooms hashmap
        rooms.insert(RoomIdentifier::Storage, 
            RoomAttributes {
                room_identifier: RoomIdentifier::Storage,
                default_description: "It is quite a small room. Mops, buckets, sanitizers and other janitorial equipment are haphazardly put together seemingly without any system.
                One of the shorter walls has a door. There are storage shelves to the right of the door. Opposite of the door is an illuminator.".to_string(),
                first_thoughts: "You wake up on the steel floor. If you laid there any longer, you'd probably get arthritis, you think. Now, where are we? Let's stand up and see.".to_string(),
                potential_items: vec![], 
                connected_rooms: vec![(Direction::East, RoomIdentifier::Mess)],
            }
        );

        // ... Repeat for other rooms

        World { rooms }
    }


    pub fn get_room_description(&self, room_id: RoomIdentifier) -> &str {
        if let Some(attributes) = self.rooms.get(&room_id) {
            &attributes.default_description
        } else {
            "Unknown room"
        }
    }

    pub fn get_adjacent_room(&self, room_id: RoomIdentifier, direction: Direction) -> Option<RoomIdentifier> {
        if let Some(room_blueprint) = self.rooms.get(&room_id) {
            for (dir, adjacent_room_id) in &room_blueprint.connected_rooms {
                if *dir == direction {
                    return Some(*adjacent_room_id);
                }
            }
        }

        None 
    }
}