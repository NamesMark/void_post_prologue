use std::collections::HashMap;
use super::room::{RoomAttributes, RoomBlueprint, RoomIdentifier, Direction};
use crate::entity::furniture::{Furniture, FurnId};
use crate::entity::item::{Item, ItemId, Container, Food, Drink, TextItem, Size};
use crate::entity::{Entity, EntityId, PassiveEntity};

use strum::IntoEnumIterator;

pub struct World {
    pub rooms: HashMap<RoomIdentifier, RoomAttributes>,
    entities: HashMap<EntityId, Box<dyn Entity>>,
    //items: HashMap<FurnId, Box<dyn Entity>>,
}

impl World {
    pub fn initialize() -> Self {
        let mut rooms = HashMap::new();
        let mut entities = HashMap::new();
        //let mut items = HashMap::new();
        
        // Populate rooms hashmap
        rooms.insert(RoomIdentifier::Storage, 
            RoomAttributes {
                room_identifier: RoomIdentifier::Storage,
                visited: true,
                short_description: "It is quite a small room. One of the shorter walls has a door. There are storage shelves to the right. Opposite of the door is an illuminator.".to_string(),
                full_description: "Looks like it's used to store janitorial and other miscellaneous items. Mops, buckets, sanitizers and other janitorial equipment are haphazardly put together seemingly without any system.".to_string(),
                first_thoughts: "You wake up on the steel floor. If you laid there any longer, you'd probably get arthritis, you think. Now, where are we? Let's stand up and see.".to_string(),
                entities: vec![
                    EntityId::Furniture(FurnId::StorageShelf),
                    EntityId::Item(ItemId::Bucket),
                ],
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
                entities: vec![
                    EntityId::Furniture(FurnId::MessTable),
                    EntityId::Item(ItemId::SpaceRation),
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
                entities: vec![
                    EntityId::Furniture(FurnId::MessTable),
                    EntityId::Item(ItemId::SpaceRation),
                ],
                connected_rooms: vec![(Direction::West, RoomIdentifier::Storage)],
            }
        );

        // ... Repeat for other rooms

        // Populate furniture:
        for id in FurnId::iter() {
            entities.insert(EntityId::Furniture(id), World::create_furniture(id));
        }
        // Populate items:
        for id in ItemId::iter() {
            entities.insert(EntityId::Item(id), World::create_item(id));
        }


        World { rooms, entities }
    }

    pub fn create_furniture(id: FurnId) -> Box<dyn Entity> {
        match id {
            FurnId::Illuminator => Box::new(PassiveEntity::new(
                "Illuminator".to_string(), 
                r#"Wow, the view is beautiful. You can see a dimply lit large station floating not so far away - the side turned towards you has letters 'Vo.. 9', the rest of the letters are undiscernable. Now it's clear that you are drifting in space on board of another vessel. Something must've happened."#.to_string()
            )),

            FurnId::StorageShelf => Box::new(Furniture::new(
                "Shelves".to_string(), 
                "Regular storage shelves that you'd find for your ship in SpaceMart™ - everything for your galactical travel needs!".to_string(),
                vec![], // TODO: add something to the shelf
            )),
            FurnId::MessTable => Box::new(Furniture::new(
                "Tables".to_string(),
                "There's nothing on the tables. The tabletops have this withered look of such heavily used and cleaned surfaces that they can never get fully clean anymore.".to_string(),
                vec![],
            )),
            FurnId::Counter => Box::new(Furniture::new(
                "Counter".to_string(),
                "The counter is cluttered with various kitchen gadgets and utensils. A half-eaten plate of biscuits sits abandoned, as if the eater left in a hurry. A small, handwritten note peeks out from under the plate.".to_string(),
                vec![
                    EntityId::Item(ItemId::CounterNote),
                    EntityId::Item(ItemId::Biscuits),
                    EntityId::Item(ItemId::Plate),
                    EntityId::Furniture(FurnId::CoffeeMachine),
                    EntityId::Furniture(FurnId::FoodPrinter),
                ],
            )),
            FurnId::CoffeeMachine => Box::new(Furniture::new(
                "Coffee Machine".to_string(),
                "It's quite an expensive coffee machine. Considering that everything else in this room is cheap, it probably means that the captain is a big fan of coffee".to_string(),
                vec![],
            )),
            FurnId::FoodPrinter => Box::new(Furniture::new(
                "Food Printer".to_string(),
                "All the food that this machine prints tastes pretty much the same. Makes sense, because it's all made of the same surrogate.".to_string(),
                vec![EntityId::Item(ItemId::FoodSurrogateBottle)],
            )),

            _ => unimplemented!(),
            // other cases...
        }
    }
    
    pub fn create_item(id: ItemId) -> Box<dyn Entity> {
        match id {
            ItemId::Bucket => Box::new(Container::new(
                "Bucket".to_string(),
                "A regular blue bucket with big letters SM on it.".to_string(),
                vec![],
                Size::Medium,
            )),
            ItemId::CounterNote => Box::new(TextItem::new(
                "Note".to_string(),
                "A small note, with some scribbles on it".to_string(),
                "Captain! We left some for you, hope you find them when you are less busy! -J".to_string()
            )),
            ItemId::Biscuits => Box::new(Food::new(
                "Biscuits".to_string(),
                "A few small chocolate biscuits. They feel a bit dry already, but still smell good and appetizing.".to_string()
            )),
            ItemId::SpaceRation => Box::new(Food::new(
                "Space ration".to_string(),
                "The package looks as it's decades old. It's probably here since this vessel has been commissioned.".to_string()
            )),
            ItemId::WaterBottle => Box::new(Drink::new(
                "Bottle of water".to_string(),
                "It's full of de-mineralized and almost de-nucleotized water, \"mined from the finest ice asteroids \"".to_string()
            )),
            ItemId::FoodSurrogateBottle => Box::new(Food::new(
                "Biscuits".to_string(),
                "A large jug of greenish liquid. According to the label, contains all the vitamins, macro-, micro- and nano-elements a humanoid might need. Hmm.".to_string()
            )),
            ItemId::Plate => Box::new(Container::new(
                "Plate".to_string(),
                "Just a regular plate. Did you expect something else?".to_string(),
                vec![],
                Size::Small,
            )),

            _ => unimplemented!(),
            // other cases...
        }
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

    pub fn get_room_entities(&self, room_id: &RoomIdentifier) -> &Vec<EntityId> {
        if let Some(room) = self.rooms.get(room_id) {
            &room.entities
        } else {
            &Vec::new()
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