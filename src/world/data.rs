use std::collections::HashMap;
use super::room::{RoomAttributes, RoomBlueprint, RoomIdentifier, Direction};
use crate::entity::furniture::Furniture;

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
                visited: true,
                short_description: "It is quite a small room. One of the shorter walls has a door. There are storage shelves to the right. Opposite of the door is an illuminator.".to_string(),
                full_description: "Looks like it's used to store janitorial and other miscellaneous items. Mops, buckets, sanitizers and other janitorial equipment are haphazardly put together seemingly without any system.".to_string(),
                first_thoughts: "You wake up on the steel floor. If you laid there any longer, you'd probably get arthritis, you think. Now, where are we? Let's stand up and see.".to_string(),
                furniture: vec![
                    Furniture { 
                        name: "Shelves".to_string(), 
                        description: "Regular storage shelves that you'd find for your ship in SpaceMart™ - everything for your galactical travel needs!".to_string()
                    },
                    Furniture { 
                        name: "Illuminator".to_string(), 
                        description: r#"Wow, the view is beautiful. You can see a dimply lit large station floating not so far away - the side turned towards you has letters 'Vo.. 9', the rest of the letters are undiscernable. Now it's clear that you are drifting in another vessel. Something must've happened."#.to_string()
                    },
                ],
                //potential_items: vec![], 
                connected_rooms: vec![(Direction::East, RoomIdentifier::Mess)],
            }
        );
        rooms.insert(RoomIdentifier::Mess, 
            RoomAttributes {
                room_identifier: RoomIdentifier::Mess,
                visited: false,
                short_description: "You enter a dimly lit medium-sized room, with some tables and a small counter. Looks like the lights are in the emergency power saving mode.".to_string(),
                full_description: "This room has half a dozen tables with benches, and a small counter with various machines, most likely used for cooking and other canteen-related activities. This looks like the place where the crew would have their meals, and get together for some friendly banter.".to_string(),
                first_thoughts: "Smells of... biscuits? It feels like I haven't eaten anything for a century.".to_string(),
                //potential_items: vec![], 
                furniture: vec![
                    Furniture { 
                        name: "Tables".to_string(), 
                        description: "There's nothing on the tables. They have this look of heavily used and clean tables, that were scrubbed so much they never get fully clean.".to_string()
                    },
                ],
                connected_rooms: vec![(Direction::West, RoomIdentifier::Storage)],
            }
        );
        rooms.insert(RoomIdentifier::Mess, 
            RoomAttributes {
                room_identifier: RoomIdentifier::Mess,
                visited: false,
                short_description: "You enter a dimly lit medium-sized room, with some tables and a small counter. Looks like the lights are in the emergency power saving mode.".to_string(),
                full_description: "This room has half a dozen tables with benches, and a small counter with various machines, most likely used for cooking and other canteen-related activities. This looks like the place where the crew would have their meals, and get together for some friendly banter.".to_string(),
                first_thoughts: "Smells of... biscuits? It feels like I haven't eaten anything for a century.".to_string(),
                //potential_items: vec![], 
                furniture: vec![
                    Furniture { 
                        name: "Tables".to_string(), 
                        description: "There's nothing on the tables. They have this look of heavily used and clean tables, that were scrubbed so much they never get fully clean.".to_string()
                    },
                ],
                connected_rooms: vec![(Direction::West, RoomIdentifier::Storage)],
            }
        );

        // ... Repeat for other rooms

        World { rooms }
    }


    pub fn was_visited(&self, room_id: &RoomIdentifier) -> bool {
        if let Some(attributes) = self.rooms.get(&room_id) {
            attributes.visited
        } else {
            eprintln!("ERROR: Unknown room");
            false
        }
    }
    pub fn set_visited(&mut self, room_id: &RoomIdentifier) -> () {
        if let Some(room_attributes) = self.rooms.get_mut(room_id) {
            room_attributes.visited = true;
        }
    }

    pub fn get_room_short_description(&self, room_id: &RoomIdentifier) -> &str {
        if let Some(attributes) = self.rooms.get(&room_id) {
            &attributes.short_description
        } else {
            eprintln!("ERROR: Unknown room");
            "Unknown room"
        }
    }

    pub fn get_room_first_thoughts(&self, room_id: &RoomIdentifier) -> &str {
        if let Some(attributes) = self.rooms.get(&room_id) {
            &attributes.first_thoughts
        } else {
            eprintln!("ERROR: Unknown room");
            "Unknown room"
        }
    }

    pub fn get_room_entities(&self, room_id: &RoomIdentifier) -> &str {
        if let Some(attributes) = self.rooms.get(&room_id) {
            todo!()
        } else {
            eprintln!("ERROR: Unknown room");
            "Unknown room"
        }
    }

    pub fn get_adjacent_room(&self, room_id: &RoomIdentifier, direction: Direction) -> Option<RoomIdentifier> {
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