use std::collections::HashMap;
use super::room::{RoomAttributes, RoomBlueprint, RoomIdentifier, Direction};

pub struct World {
    pub rooms: HashMap<RoomIdentifier, RoomBlueprint>,
}

impl World {
    pub fn initialize() -> Self {
        let mut rooms = HashMap::new();
        
        // Populate rooms hashmap
        rooms.insert(RoomIdentifier::Storage, RoomBlueprint::Storage(
            RoomAttributes {
                identifier: RoomIdentifier::Storage,
                default_description: "It is quite a small room. Mops, buckets, sanitizers and other janitorial equipment are haphazardly put together seemingly without any system.
                One of the shorter walls has a door. There are storage shelves to the right of the door. Opposite of the door is an illuminator.".to_string(),
                first_thoughts: "You wake up on the steel floor. If you laid there any longer, you'd probably get arthritis, you think. Now, where are we? Let's stand up and see.".to_string(),
                potential_items: vec![], 
                connected_rooms: vec![(Direction::East, RoomIdentifier::Mess)],
            }
        ));

        // ... Repeat for other rooms

        World { rooms }
    }
}