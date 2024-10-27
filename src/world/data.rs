use super::room::{Access, Direction, RoomAttributes, RoomIdentifier};
use crate::entity::furniture::main_terminal::MainTerminal;
use crate::entity::furniture::navigation_computer::NavigationComputer;
use crate::entity::furniture::sink::Sink;
use crate::entity::furniture::{FurnId, Furniture};
use crate::entity::item::container::Container;
use crate::entity::item::food::Food;
use crate::entity::item::text_item::TextItem;
use crate::entity::item::{Containable, Drink, Edible, Item, ItemId, Readable, Size, Usable};
use crate::entity::{Entity, EntityId, PassiveEntity};
use crate::world::room::PassageType;
use std::collections::HashMap;

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
                    (Direction::East, PassageType::Door, RoomIdentifier::CrewCabins),
                    (Direction::South, PassageType::Free, RoomIdentifier::SouthMess)
                ],
            }
        );
        rooms.insert(RoomIdentifier::CrewCabins, 
            RoomAttributes {
                room_identifier: RoomIdentifier::CrewCabins,
                visited: false,
                access: Access::D,
                short_description: "Crew Cabins Corridor".to_string(),
                full_description: "A narrow hallway lined with personal quarters extends before you. The lighting is dim, flickering slightly, adding to the air of weary privacy that pervades this space.".to_string(),
                first_thoughts: "You wonder how many people called this place home? Has the same crew operated this vessel from the beginning, or did many generations change? There are probably countless stories and memories embedded in these walls, which you probably won't ever know.\nOn a more pressing issue, it would seem that the shuttle is completely empty. You guess that your main task should be to try to off the shuttle and reach the space outpost nearby. How would you do that?".to_string(),
                entities: vec![
                ],
                connected_rooms: vec![
                    (Direction::West, PassageType::Door, RoomIdentifier::Mess),
                    (Direction::North, PassageType::Door, RoomIdentifier::BosunsRoom)
                ],
            }
        );
        rooms.insert(RoomIdentifier::BosunsRoom, 
            RoomAttributes {
                room_identifier: RoomIdentifier::BosunsRoom,
                visited: false,
                access: Access::D,
                short_description: "Bosun's Command".to_string(),
                full_description: "This spartan room bears the mark of authority and order. A large, sturdy desk with neatly arranged tools and documents dominates the space. Personal effects are few but chosen with care, perhaps tokens of past voyages or loved ones afar.".to_string(),
                first_thoughts: "Ah, an unmistacable spartan style of a bosun. You've met people like this before.".to_string(),
                entities: vec![
                    EntityId::Furniture(FurnId::BosunDesk),
                ],
                connected_rooms: vec![
                    (Direction::South, PassageType::Door, RoomIdentifier::CrewCabins)
                ],
            }
        );
        rooms.insert(RoomIdentifier::CaptainsRoom, 
            RoomAttributes {
                room_identifier: RoomIdentifier::CaptainsRoom,
                visited: false,
                access: Access::B,
                short_description: "Captain's Quarters".to_string(),
                full_description: "Even though the room is a bit shabby for a captain, it still exudes a sense of quiet authority. There's a desk with a personal terminal, bookshelves, a neatly made bed, and a regular illuminator that nonetheless offers a breathtaking view of the stars. This is a sanctuary, a place for leadership, and introspection... with a help of a cheap AstraKefali brendi.".to_string(),
                first_thoughts: "Oh, a captain's room, cool! I'm sure there's something to steal here, he-he.".to_string(),
                entities: vec![
                    EntityId::Furniture(FurnId::BookShelves),
                    EntityId::Furniture(FurnId::CaptainsDesk),
                    EntityId::Furniture(FurnId::CaptainsIlluminator),
                ],
                connected_rooms: vec![
                    (Direction::East, PassageType::Door, RoomIdentifier::MeetingRoom)
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
        rooms.insert(RoomIdentifier::AirlockCorridor, 
            RoomAttributes {
                room_identifier: RoomIdentifier::AirlockCorridor,
                visited: false,
                access: Access::D,
                short_description: "This is a tight corridor.".to_string(),
                full_description: "This is a corridor that is used to enter and exit this vessel. It has two airlocks on the opposite ends of it.".to_string(),
                first_thoughts: "It feels kinda... fresh in here? Although I start to wonder where is everyone.".to_string(),
                entities: vec![
                ],
                connected_rooms: vec![
                    (Direction::North, PassageType::Door, RoomIdentifier::SouthMess),
                    (Direction::West, PassageType::Door, RoomIdentifier::AirlockA),
                    (Direction::East, PassageType::Door, RoomIdentifier::AirlockB),
                    (Direction::Down, PassageType::Door, RoomIdentifier::TechCorridor),
                    (Direction::South, PassageType::Door, RoomIdentifier::StorageHold),
                ],
            }
        );
        rooms.insert(RoomIdentifier::StorageHold, 
            RoomAttributes {
                room_identifier: RoomIdentifier::StorageHold,
                visited: false,
                access: Access::A,
                short_description: "Spacious room filled with shelves with various crates, boxes and barrels. There's an emergency locker in the corner.".to_string(),
                full_description: "This is a corridor that is used to enter and exit this vessel. It has two airlocks on the opposite ends of it.".to_string(),
                first_thoughts: "It feels kinda... fresh in here?".to_string(),
                entities: vec![
                    EntityId::Furniture(FurnId::EmergencyLocker),
                ],
                connected_rooms: vec![
                    (Direction::North, PassageType::Door, RoomIdentifier::AirlockCorridor),
                ],
            }
        );
        rooms.insert(
            RoomIdentifier::AirlockA,
            RoomAttributes {
                room_identifier: RoomIdentifier::AirlockA,
                visited: false,
                access: Access::D,
                short_description:
                    "You squeeze into a tiny square space used to get to and from the shuttle."
                        .to_string(),
                full_description: "".to_string(),
                first_thoughts: "".to_string(),
                entities: vec![EntityId::Furniture(FurnId::WarningSign)],
                connected_rooms: vec![
                    (
                        Direction::West,
                        PassageType::Door,
                        RoomIdentifier::OpenSpaceAirlockA,
                    ),
                    (
                        Direction::East,
                        PassageType::Door,
                        RoomIdentifier::AirlockCorridor,
                    ),
                ],
            },
        );
        rooms.insert(RoomIdentifier::OpenSpaceAirlockA, 
            RoomAttributes {
                room_identifier: RoomIdentifier::OpenSpaceAirlockA,
                visited: false,
                access: Access::D,
                short_description: "".to_string(),
                full_description: "You don't feel or hear anything except for your breath and the warm condensation on the space suit's mask.".to_string(),
                first_thoughts: "You tuck your legs and then straighten them with force to push yourself away from the shuttle. The station is so near. Just a few seconds of floating and you'll get there...".to_string(),
                entities: vec![
                ],
                connected_rooms: vec![
                    (Direction::West, PassageType::Free, RoomIdentifier::StationAirlock),
                ],
            }
        );
        rooms.insert(RoomIdentifier::StationAirlock, 
            RoomAttributes {
                room_identifier: RoomIdentifier::StationAirlock,
                visited: false,
                access: Access::D,
                short_description: "This is the Void Post 39 airlock.".to_string(),
                full_description: "The airlock of the station stands before you, a circular door etched with the scars of space travel - micrometeorite impacts and the wear of countless entries and exits. Inside, you can see the faint glow of emergency lighting, offering a warm contrast to the cold, unfeeling vacuum outside. The airlock promises a return to a semblance of normalcy, a brief respite from the endless expanse outside.".to_string(),
                first_thoughts: "As you approach the station's airlock, a wave of relief washes over you. The mechanical hiss of the airlock operating breaks the silence, grounding you back in a world where sound exists. 'You are finally here,' you think to yourself, as the doors begin to open, welcoming you into the station's embrace. The thought of safety, warmth, and perhaps answers to the myriad questions swirling in your head fills you with renewed purpose.".to_string(),
                entities: vec![
                ],
                connected_rooms: vec![
                ],
            }
        );
        rooms.insert(
            RoomIdentifier::AirlockB,
            RoomAttributes {
                room_identifier: RoomIdentifier::AirlockB,
                visited: false,
                access: Access::D,
                short_description:
                    "You squeeze into a tiny square space used to get to and from the shuttle."
                        .to_string(),
                full_description: "".to_string(),
                first_thoughts: "".to_string(),
                entities: vec![EntityId::Furniture(FurnId::WarningSign)],
                connected_rooms: vec![
                    (
                        Direction::East,
                        PassageType::Door,
                        RoomIdentifier::OpenSpaceAirlockB,
                    ),
                    (
                        Direction::West,
                        PassageType::Door,
                        RoomIdentifier::AirlockCorridor,
                    ),
                ],
            },
        );
        rooms.insert(
            RoomIdentifier::OpenSpaceAirlockB,
            RoomAttributes {
                room_identifier: RoomIdentifier::OpenSpaceAirlockB,
                visited: false,
                access: Access::D,
                short_description: "".to_string(),
                full_description: "".to_string(),
                first_thoughts: "".to_string(),
                entities: vec![],
                connected_rooms: vec![(
                    Direction::West,
                    PassageType::Door,
                    RoomIdentifier::AirlockB,
                )],
            },
        );

        rooms.insert(RoomIdentifier::TechCorridor, 
            RoomAttributes {
                room_identifier: RoomIdentifier::TechCorridor,
                visited: false,
                access: Access::D,
                short_description: "You enter a technical corridor in the lower deck of the shuttle.".to_string(),
                full_description: "This space is apparently used to access various systems related to the shuttle drive, energy and life systems.".to_string(),
                first_thoughts: "I've never been in a tight cave, but I imagine it feels something like this.".to_string(),
                entities: vec![
                ],
                connected_rooms: vec![
                    (Direction::Up, PassageType::Door, RoomIdentifier::AirlockCorridor),
                    (Direction::South, PassageType::Door, RoomIdentifier::EngineRoom),
                    (Direction::North, PassageType::Free, RoomIdentifier::TechCorridorNorth),
                ],
            }
        );
        rooms.insert(RoomIdentifier::EngineRoom, 
            RoomAttributes {
                room_identifier: RoomIdentifier::EngineRoom,
                visited: false,
                access: Access::D,
                short_description: "You enter the heart of the shuttle: it's engine room.".to_string(),
                full_description: "The room is full of different mechanisms and machines, most of which are a mystery to you. But you are pretty sure that the biggest thing in the middle of the room is the main thrust engine.".to_string(),
                first_thoughts: "It smells of grease and soot. One would think the engines would smell differently in the space age.".to_string(),
                entities: vec![
                    EntityId::Furniture(FurnId::MainEngine),
                ],
                connected_rooms: vec![
                    (Direction::North, PassageType::Free, RoomIdentifier::TechCorridor),
                ],
            }
        );
        rooms.insert(
            RoomIdentifier::TechCorridorNorth,
            RoomAttributes {
                room_identifier: RoomIdentifier::TechCorridorNorth,
                visited: false,
                access: Access::D,
                short_description: "The north section of the same technical corridor.".to_string(),
                full_description: "".to_string(),
                first_thoughts: "".to_string(),
                entities: vec![],
                connected_rooms: vec![(
                    Direction::South,
                    PassageType::Free,
                    RoomIdentifier::TechCorridor,
                )],
            },
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
                full_description: "The walls of this compact room are lined with a few outdated screens and control panels. It's designed for quick, efficient meetings. There's no furniture due to the lack of space.".to_string(),
                first_thoughts: "Ah, a meeting and navigation room. Looks like shuttles like this don't have any space on the bridge to accomodate this functionality. There's certainly something useful here to get me out and on the outpost.".to_string(),
                entities: vec![
                    EntityId::Furniture(FurnId::NavigationComputer),
                ],
                connected_rooms: vec![
                    (Direction::North, PassageType::Door, RoomIdentifier::Bridge),
                    (Direction::South, PassageType::Door, RoomIdentifier::NorthMess),
                    (Direction::West, PassageType::Door, RoomIdentifier::CaptainsRoom),
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
                "This is the main terminal of the shuttle. The message on the display says: \"There was a problem with your payment. Your subscription to the ShuttleControlOS has been suspended. Please top up your account to regain control of the {$$shuttle_name$$}.\"".to_string(),
                vec![],
            )),
            FurnId::NavigationComputer => Box::new(NavigationComputer::new(
                EntityId::Furniture(FurnId::NavigationComputer),
                "Navigation computer".to_string(),
                vec!["terminal".to_string(), "computer".to_string()],
                "This simple terminal blinks at you with a multitude of colorful lights as if in a friendly jest. From the flickering start charts and trajectory data, it is obvious that it's used to plot the routes of deep-space travel, as well as calculate the approach trajectories to dock various stations and spaceships, which is the primary use of a shuttle. The interface looks quite user-friendly, you are sure you'd be able to operate it.".to_string(),
                vec![],
            )),
            FurnId::WarningSign => Box::new(PassiveEntity::new(
                EntityId::Furniture(FurnId::WarningSign),
                "Warning sign".to_string(), 
                vec!["sign".to_string()],
                "A worn sign is bolted to the wall. On it you see a figure clutching hands at their neck, their face is blue. I wonder what could it mean?".to_string()
                //A stark warning sign is bolted to the wall, its edges worn and paint peeling from age. It depicts a figure in stark black against a bright yellow background, hands clutched at the throat in a universal gesture of asphyxiation. Below the grim pictogram, faded letters offer a silent admonishment: 'Caution: Vacuum Zone – Use Appropriate Life Support Equipment'. The message is clear - without a suit, death is both certain and swift.
            )),
            FurnId::MainEngine => Box::new(PassiveEntity::new(
                EntityId::Furniture(FurnId::MainEngine),
                "Main engine".to_string(), 
                vec!["engine".to_string()],
                "The Main Engine of the shuttle, a compact module of practical engineering, is bolted firmly to the rear compartment. You are pretty sure that at some point its surface was shiny, with bright yellow details. Now all of it has a uniform dirty-grey color, with smudges of soot and grease. It's not the clean, high-tech wonder you might find on larger or more expensive vessels, but it's the heart of this shuttle, dependable and resilient. It looks fully functional.".to_string()
            )),
            FurnId::BosunDesk => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::BosunDesk),
                "Bosun's desk".to_string(), 
                vec!["desk".to_string(), "bosun desk".to_string(), "bosun's desk".to_string()],
                "A robust and no-nonsense piece of furniture, the Bosun's Desk stands as a testament to practicality over aesthetics. The surface is littered with charts, navigational tools, and the occasional personal memento. Each drawer looks to be meticulously labeled, and the desk's well-worn edges suggest years of service and countless hours of diligent work.".to_string(),
                vec![EntityId::Item(ItemId::BosunCard)],
            )),
            FurnId::BookShelves => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::BookShelves),
                "Bookshelves".to_string(),
                vec!["shelves".to_string(), "book shelves".to_string()],
                "In almost any captain's room you'd expect to see a bookshelf that's collection of knowledge and memories, filled with volumes of space navigation, astrophysics, and historical logs. Alas, this one contains mostly works of fiction and lewd love stories, with a scattering of random old, dusty technical manuals. (you'd be surprised if the captain actually read any of the dusty stuff). An... interesting choice of literature. Offers an insight into the captain's personal life, and it looks like he couldn't care less about what anyone would think.".to_string(),
                vec![EntityId::Item(ItemId::ShuttleManual)],
            )),
            FurnId::CaptainsIlluminator => Box::new(PassiveEntity::new(
                EntityId::Furniture(FurnId::CaptainsIlluminator),
                "Illuminator".to_string(), 
                vec!["window".to_string()],
                r#"You are stunned by the breathtaking view of the stars. There's nothing else visible from this illuminator, only the infinite vastness of the universe."#.to_string()
            )),
            FurnId::CaptainsDesk => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::CaptainsDesk),
                "Captain's desk".to_string(), 
                vec!["desk".to_string(), "captain desk".to_string(), "captain's desk".to_string()],
                "What used to be an elegant, yet functional desk for the most important person in the shuttle, now seems quite ordinary and unimportant. Maybe it would feel different with an actual captain behind it.".to_string(),
                vec![EntityId::Item(ItemId::CaptainCard)],
            )),

            FurnId::FuelTankA => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::FuelTankA),
                "Fuel tank A".to_string(), 
                vec!["tank".to_string(), "tank a".to_string(), "fuel tank".to_string()],
                "The screen says 'Active Tank. Low fuel. Please contact the Gerbertt support team in case you experience problems switching to the reserve tank.'.".to_string(),
                vec![],
            )),
            FurnId::FuelTankB => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::FuelTankB),
                "Fuel tank B".to_string(), 
                vec!["tank".to_string(), "tank b".to_string(), "fuel tank".to_string()],
                "The screen says 'Inactive. Full.".to_string(),
                vec![],
            )),
            FurnId::EmergencyLocker => Box::new(Furniture::new(
                EntityId::Furniture(FurnId::EmergencyLocker),
                "Emergency locker".to_string(), 
                vec!["locker".to_string()],
                "The plaque here says 'Use in case of emergencies. Don't forget to help yourself first before helping your crewmate!'".to_string(),
                vec![EntityId::Item(ItemId::SpaceSuit)],
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
                "This rectangle piece of light-grey plastic bears the name 'Alexis Jericho - Staff Assistant'. The text is printed directly over a stylized 'D', embossed in the background. It's clearly not yours, but you are pretty sure you won't get anywhere on the shuttle without some form of identification. The edges are worn smooth from handling, suggesting a history of frequent use.".to_string(),    
                Size::Small,
            )),
            ItemId::BosunCard => Box::new(Item::new(
                EntityId::Item(ItemId::BosunCard),
                "Bosun Card".to_string(),
                vec!["card".to_string(), "access card".to_string(), "bosun's card".to_string()],
                "The Bosun Card feels heavy with authority, a sturdy plastic keycard emblazoned with the title 'Bosun' in authoritative block letters. Below the title, a holographic strip winks with secure coding, promising access to the ship's vital operational areas. It's the kind of card that opens more doors than just the physical ones.".to_string(),
                Size::Small,
            )),
            ItemId::CaptainCard => Box::new(Item::new(
                EntityId::Item(ItemId::CaptainCard),
                "Captain Card".to_string(),
                vec!["card".to_string(), "access card".to_string(), "captain's card".to_string()],
                "This Captain Card is the pinnacle of any ship's hierarchy, with its crisp edges and the gilded 'Captain' inscription that seems to command respect on its own. A faint scent of leather clings to it, as if it has spent most of its time in the pocket of someone decisive. Its clearance is unmatched.".to_string(),
                Size::Small,
            )),
            ItemId::SpaceSuit => Box::new(Item::new(
                EntityId::Item(ItemId::SpaceSuit),
                "Space Suit".to_string(),
                vec![
                    "suit".to_string(),
                    "spacesuit".to_string()
                    ],
                "A typical space suit to be safe on short space walks.".to_string(),
                Size::Medium,
            )),
            ItemId::ShuttleManual => Box::new(TextItem::new(
                EntityId::Item(ItemId::ShuttleManual),
                "Shuttle manual".to_string(),
                vec!["manual".to_string(),],
                "A paperback instructional book. The cover is damaged, but you can discern 'XM-86 shuttle user manual'".to_string(),
                "Page 86... Main terminal... Manual override commands... \n
                    ...\n
                    01. Main thrust engine commands:\n
                    - 01::00::00 main engine shut down \n
                    - 01::00::01 main engine start \n
                    - 01::04::00 main engine thrust on\n
                    - 01::04::01 main engine thrust off\n
                    - 01::05::<power> specify thruster power level from 0 to 63\n
                    - 01::09::00 main engine status\n
                    02. Maneuvre engine commands:\n
                    - 02::02::00::XXX specify x vector (0-360)\n
                    - 02::02::01::YYY specify y vector (0-360)\n
                    - 02::04::00 maneuvre engine thrust on\n
                    - 02::04::01 maneuvre engine thrust off\n
                    - 02::05::<power> specify maneuvre thrust power from 0 to 7\n
                    - 02::09::00 maneuvre engine status\n
                    07. Fuel system commands:\n
                    - 07::00::00 turn the fuel pump off\n
                    - 07::00::01 turn the fuel pump on\n
                    - 07::01::00 switch to fuel tank A\n
                    - 07::01::01 switch to fuel tank B\n
                    - 07::09::00 fuel system status

                ".to_string()
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
    pub fn get_room_entities_mut(
        &mut self,
        room_id: &RoomIdentifier,
    ) -> Option<&mut Vec<EntityId>> {
        self.rooms.get_mut(room_id).map(|room| &mut room.entities)
    }

    pub fn get_room_access(&self, room_id: &RoomIdentifier) -> &Access {
        if let Some(room) = self.rooms.get(room_id) {
            &room.access
        } else {
            &Access::None
        }
    }

    pub fn get_adjacent_room(
        &self,
        room_id: &RoomIdentifier,
        direction: Direction,
    ) -> Option<(RoomIdentifier, &PassageType)> {
        if let Some(room_attributes) = self.rooms.get(&room_id) {
            for (dir, passageType, adjacent_room_id) in &room_attributes.connected_rooms {
                if *dir == direction {
                    return Some((*adjacent_room_id, passageType));
                }
            }
        }

        None
    }

    pub fn get_containable(&self, entity_id: EntityId) -> Option<&dyn Containable> {
        self.entities
            .get(&entity_id)
            .and_then(|entity| entity.as_containable())
    }

    pub fn get_containable_mut(&mut self, entity_id: EntityId) -> Option<&mut dyn Containable> {
        self.entities
            .get_mut(&entity_id)
            .and_then(|entity| entity.as_containable_mut())
    }

    pub fn get_edible(&self, entity_id: EntityId) -> Option<&dyn Edible> {
        self.entities
            .get(&entity_id)
            .and_then(|entity| entity.as_edible())
    }

    pub fn get_edible_mut(&mut self, entity_id: EntityId) -> Option<&mut dyn Edible> {
        self.entities
            .get_mut(&entity_id)
            .and_then(|entity| entity.as_edible_mut())
    }

    pub fn get_readable(&self, entity_id: EntityId) -> Option<&dyn Readable> {
        self.entities
            .get(&entity_id)
            .and_then(|entity| entity.as_readable())
    }

    pub fn get_readable_mut(&mut self, entity_id: EntityId) -> Option<&mut dyn Readable> {
        self.entities
            .get_mut(&entity_id)
            .and_then(|entity| entity.as_readable_mut())
    }

    pub fn get_usable_mut(&mut self, entity_id: EntityId) -> Option<&mut dyn Usable> {
        self.entities
            .get_mut(&entity_id)
            .and_then(|entity| entity.as_usable_mut())
    }
}
