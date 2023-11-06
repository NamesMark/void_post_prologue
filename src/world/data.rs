use std::collections::HashMap;
use super::room::{RoomAttributes, RoomBlueprint, RoomIdentifier, Direction, Access};
use crate::entity::furniture::{Furniture, FurnId, Sink, MainTerminal};
use crate::entity::item::{Item, ItemId, Container, Food, Drink, TextItem, Size};
use crate::entity::{Entity, EntityId, PassiveEntity};
use crate::world::room::PassageType;

use strum::IntoEnumIterator;

pub struct World {
    pub rooms: HashMap<RoomIdentifier, RoomAttributes>,
    pub entities: HashMap<EntityId, Box<dyn Entity>>,
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
                access: Access::None,
                short_description: "It is quite a small room. One of the shorter walls has a door. There are storage shelves to the right. Opposite of the door is an illuminator.".to_string(),
                full_description: "Looks like it's used to store janitorial and other miscellaneous items. Mops, buckets, sanitizers and other janitorial equipment are haphazardly put together seemingly without any system.".to_string(),
                first_thoughts: "You wake up on the steel floor. If you laid there any longer, you'd probably get arthritis, you think. Now, where are we? Let's stand up and see.".to_string(),
                entities: vec![
                    EntityId::Furniture(FurnId::Illuminator),
                    EntityId::Furniture(FurnId::StorageShelf),
                    EntityId::Item(ItemId::Bucket),
                ],
                connected_rooms: vec![(Direction::East, PassageType::Door, RoomIdentifier::NorthMess)],
            }
        );
        rooms.insert(RoomIdentifier::NorthMess, 
            RoomAttributes {
                room_identifier: RoomIdentifier::NorthMess,
                visited: false,
                access: Access::D,
                short_description: "You enter a dimly lit medium-sized room, with some tables and a small counter. Looks like the lights are in the emergency power saving mode.".to_string(),
                full_description: "This room has half a dozen tables with benches. This looks like the place where the crew would have their meals, and get together for some friendly banter.".to_string(),
                first_thoughts: "Smells of... biscuits? It feels like I haven't eaten anything for a century. Where is this smell coming from?".to_string(),
                entities: vec![
                    EntityId::Furniture(FurnId::MessTable),
                    EntityId::Item(ItemId::SpaceRation),
                ],
                connected_rooms: vec![
                    (Direction::West, PassageType::Door, RoomIdentifier::Storage),
                    (Direction::North, PassageType::Door, RoomIdentifier::MeetingRoom),
                    (Direction::South, PassageType::Free, RoomIdentifier::Mess)
                ],
            }
        );
        rooms.insert(RoomIdentifier::Mess, 
            RoomAttributes {
                room_identifier: RoomIdentifier::Mess,
                visited: false,
                access: Access::D,
                short_description: "You walk to the middle of the room. Now you stand near a small counter with various machines.".to_string(),
                full_description: "In front of you is a small counter with various machines, most likely used for cooking and other canteen-related activities.".to_string(),
                first_thoughts: "The smell is stronger! I think it's here.".to_string(),
                entities: vec![
                    EntityId::Furniture(FurnId::MessTable),
                    EntityId::Furniture(FurnId::Counter),
                ],
                connected_rooms: vec![
                    (Direction::North, PassageType::Free, RoomIdentifier::NorthMess),
                    (Direction::South, PassageType::Free, RoomIdentifier::SouthMess)
                ],
            }
        );

        rooms.insert(RoomIdentifier::SouthMess, 
            RoomAttributes {
                room_identifier: RoomIdentifier::SouthMess,
                visited: false,
                access: Access::D,
                short_description: "You are at the southern wall of this room. There's a door farther south, and another one to the west.".to_string(),
                full_description: "".to_string(),
                first_thoughts: "".to_string(),
                entities: vec![
                ],
                connected_rooms: vec![
                    (Direction::North, PassageType::Free, RoomIdentifier::Mess),
                    (Direction::South, PassageType::Door, RoomIdentifier::AirlockCorridor),
                    (Direction::West, PassageType::Door, RoomIdentifier::PassengersRoom),
                ],
            }
        );
        rooms.insert(RoomIdentifier::Bridge, 
            RoomAttributes {
                room_identifier: RoomIdentifier::Mess,
                visited: false,
                access: Access::A,
                short_description: "You are at the bridge. It's the brain of any ship, all the most important controls are here.".to_string(),
                full_description: "".to_string(),
                first_thoughts: "I bet this room is the key to getting off this tincan!".to_string(),
                entities: vec![
                    EntityId::Furniture(FurnId::MainTerminal),
                ],
                connected_rooms: vec![(Direction::South, PassageType::Door, RoomIdentifier::MeetingRoom)],
            }
        );
        rooms.insert(RoomIdentifier::MeetingRoom, 
            RoomAttributes {
                room_identifier: RoomIdentifier::Mess,
                visited: false,
                access: Access::B,
                short_description: "Meeting room".to_string(),
                full_description: "".to_string(),
                first_thoughts: "".to_string(),
                entities: vec![
                ],
                connected_rooms: vec![
                    (Direction::North, PassageType::Door, RoomIdentifier::Bridge),
                    (Direction::South, PassageType::Door, RoomIdentifier::NorthMess)
                ],
            }
        );
        rooms.insert(RoomIdentifier::PassengersRoom, 
            RoomAttributes {
                room_identifier: RoomIdentifier::PassengersRoom,
                visited: false,
                access: Access::D,
                short_description: "You enter quite a stylishly decorated and mostly clean room.".to_string(),
                full_description: "".to_string(),
                first_thoughts: "It was probably reserved for passengers of status. Doesn't seem like it was used much... for a long time.".to_string(),
                //potential_items: vec![], 
                entities: vec![

                ],
                connected_rooms: vec![(Direction::East, PassageType::Door, RoomIdentifier::Mess)],
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
                EntityId::Furniture(FurnId::Illuminator),
                "Illuminator".to_string(), 
                vec!["window".to_string()],
                r#"Wow, the view is beautiful. You can see a dimply lit large station floating not so far away - the side turned towards you has letters 'Vo.. 9', the rest of the letters are undiscernable. Now it's clear that you are drifting in space on board of another vessel. Something must've happened."#.to_string()
            )),

            FurnId::StorageShelf => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::StorageShelf),
                "Shelves".to_string(),
                vec!["shelve".to_string()],
                "Regular storage shelves that you'd find for your ship in SpaceMart™ - everything for your galactical travel needs!".to_string(),
                vec![], // TODO: add something to the shelf
            )),
            FurnId::MessTable => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::MessTable),
                "Tables".to_string(),
                vec!["table".to_string()],
                "There's nothing on the tables. The tabletops have this withered look of such heavily used and cleaned surfaces that they can never get fully clean anymore.".to_string(),
                vec![],
            )),
            FurnId::Counter => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::Counter),
                "Counter".to_string(),
                vec!["countertop".to_string()],
                "The counter is cluttered with various kitchen gadgets and utensils. A half-eaten plate of biscuits sits abandoned, as if the eater left in a hurry. A small, handwritten note peeks out from under the plate.".to_string(),
                vec![
                    EntityId::Item(ItemId::CounterNote),
                    EntityId::Item(ItemId::Biscuits),
                    EntityId::Item(ItemId::Plate),
                    EntityId::Furniture(FurnId::CoffeeMachine),
                    EntityId::Furniture(FurnId::FoodPrinter),
                    EntityId::Furniture(FurnId::Sink),
                ],
            )),
            FurnId::CoffeeMachine => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::CoffeeMachine),
                "Coffee Machine".to_string(),
                vec!["coffemaker".to_string(), "machine".to_string()],
                "It's quite an expensive coffee machine. Considering that everything else in this room is cheap, it probably means that the captain is a big fan of coffee".to_string(),
                vec![],
            )),
            FurnId::FoodPrinter => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::FoodPrinter),
                "Food Printer".to_string(),
                vec!["printer".to_string()],
                "All the food that this machine prints tastes pretty much the same. Makes sense, because it's all made of the same surrogate.".to_string(),
                vec![EntityId::Item(ItemId::FoodSurrogateBottle)],
            )),
            FurnId::Sink => Box::new(Sink::new(
                EntityId::Furniture(FurnId::Sink),
                "Sink".to_string(),
                vec!["kitchen sink".to_string(), "basin".to_string()],
                "The water from the tap is supposed to be potable... mostly.".to_string(),
                vec![EntityId::Item(ItemId::Fork)],
            )),
            FurnId::MainTerminal => Box::new(MainTerminal::new(
                EntityId::Furniture(FurnId::MainTerminal),
                "Main terminal".to_string(),
                vec!["terminal".to_string(), "control terminal".to_string()],
                "".to_string(),
                vec![EntityId::Item(ItemId::Fork)],
            )),



            // other cases...

            FurnId::Dust => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::Dust),
                "Dust".to_string(),
                vec![],
                "Heavy dust in the corners of this room.".to_string(),
                vec![],
            )),
            //_ => unimplemented!(),
        }
    }
    
    pub fn create_item(id: ItemId) -> Box<dyn Entity> {
        match id {
            ItemId::Bucket => Box::new(Container::new(
                EntityId::Item(ItemId::Bucket),
                "Bucket".to_string(),
                vec!["blue bucket".to_string()],
                "A regular blue bucket with big letters SM on it.".to_string(),
                vec![],
                Size::Medium,
            )),
            ItemId::CounterNote => Box::new(TextItem::new(
                EntityId::Item(ItemId::CounterNote),
                "Note".to_string(),
                vec![],
                "A small note, with some scribbles on it".to_string(),
                "Captain! We left some for you, hope you find them when you are less busy! -J".to_string()
            )),
            ItemId::Biscuits => Box::new(Food::new(
                EntityId::Item(ItemId::Biscuits),
                "Biscuits".to_string(),
                vec!["cupcakes".to_string(), "cakes".to_string()],
                "A few small chocolate biscuits. They feel a bit dry already, but still smell good and appetizing.".to_string()
            )),
            ItemId::SpaceRation => Box::new(Food::new(
                EntityId::Item(ItemId::SpaceRation),
                "Space ration".to_string(),
                vec!["ration".to_string()],
                "The package looks as it's decades old. It's probably here since this vessel has been commissioned.".to_string()
            )),
            ItemId::WaterBottle => Box::new(Drink::new(
                EntityId::Item(ItemId::WaterBottle),
                "Bottle of water".to_string(),
                vec!["bottle".to_string(), "water".to_string()],
                "It's full of de-mineralized and almost de-nucleotized water, \"mined from the finest ice asteroids \"".to_string()
            )),
            ItemId::FoodSurrogateBottle => Box::new(Food::new(
                EntityId::Item(ItemId::FoodSurrogateBottle),
                "Food surrogate bottle".to_string(),
                vec!["food surrogate".to_string(), "surrogate".to_string(), "bottle".to_string()],
                "A large jug of greenish liquid. According to the label, contains all the vitamins, macro-, micro- and nano-elements a humanoid might need. Hmm.".to_string()
            )),
            ItemId::Plate => Box::new(Container::new(
                EntityId::Item(ItemId::Plate),
                "Plate".to_string(),
                vec!["dish".to_string()],
                "Just a regular plate. Did you expect something else?".to_string(),
                vec![],
                Size::Small,
            )),
            ItemId::Fork => Box::new(Item::new(
                EntityId::Item(ItemId::Fork),
                "Fork".to_string(),
                vec![],
                "It has three prongs. Technically, it probably should be called a small trident?".to_string(),
                Size::Small,
            )),
            ItemId::EmptyBottle => Box::new(Item::new(
                EntityId::Item(ItemId::EmptyBottle),
                "Empty Bottle".to_string(),
                vec!["bottle".to_string()],
                "Just a regular transparent bottle that used to contain something.".to_string(),
                Size::Small,
            )),
            ItemId::SecretBottle => Box::new(Item::new(
                EntityId::Item(ItemId::SecretBottle),
                "Secret Bottle".to_string(),
                vec!["bottle".to_string()],
                "".to_string(),
                Size::Small,
            )),
            ItemId::LuckyCoin => Box::new(Item::new(
                EntityId::Item(ItemId::LuckyCoin),
                "Lucky Coin".to_string(),
                vec!["coin".to_string()],
                "".to_string(),
                Size::Small,
            )),
            ItemId::AssistantCard => Box::new(Item::new(
                EntityId::Item(ItemId::AssistantCard),
                "Assistant Card".to_string(),
                vec!["card".to_string()],
                "".to_string(),
                Size::Small,
            )),
            ItemId::BosunCard => Box::new(Item::new(
                EntityId::Item(ItemId::BosunCard),
                "Bosun Card".to_string(),
                vec!["card".to_string()],
                "".to_string(),
                Size::Small,
            )),
            ItemId::CaptainCard => Box::new(Item::new(
                EntityId::Item(ItemId::CaptainCard),
                "Captain Card".to_string(),
                vec!["card".to_string()],
                "".to_string(),
                Size::Small,
            )),





            // other cases...


            ItemId::Dust => Box::new(Item::new(
                EntityId::Item(ItemId::Dust),
                "A layer of dust".to_string(),
                vec![],
                "Nothing much to say here".to_string(),
                Size::Small,
            )),
            //_ => unimplemented!(),
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

    pub fn get_room_entities(&self, room_id: &RoomIdentifier) -> Option<&Vec<EntityId>> {
        self.rooms.get(room_id).map(|room| &room.entities)
    }
    pub fn get_room_entities_mut(&mut self, room_id: &RoomIdentifier) -> Option<&mut Vec<EntityId>> {
        self.rooms.get_mut(room_id).map(|room| &mut room.entities)
    }

    pub fn get_room_access(&self, room_id: &RoomIdentifier) -> &Access {
        if let Some(room) = self.rooms.get(room_id) {
            &room.access
        } else {
            &Access::None
        }
    }

    pub fn get_adjacent_room(&self, room_id: &RoomIdentifier, direction: Direction) -> Option<(RoomIdentifier, &PassageType)> {
        if let Some(room_attributes) = self.rooms.get(&room_id) {
            for (dir, passageType, adjacent_room_id) in &room_attributes.connected_rooms {
                if *dir == direction {
                    return Some((*adjacent_room_id, passageType));
                }
            }
        }

        None 
    }

    pub fn get_container(&self, entity_id: EntityId) -> Option<&Container> {
        self.entities.get(&entity_id).and_then(|entity| {
            entity.as_any().downcast_ref::<Container>()
        })
    }
    pub fn get_container_mut(&mut self, entity_id: EntityId) -> Option<&mut Container> {
        self.entities.get_mut(&entity_id).and_then(|entity| {
            entity.as_any_mut().downcast_mut::<Container>()
        })
    }
}