use strum_macros::{EnumIter, Display};
use crate::entity::{furniture::Furniture, EntityId};
use std::cmp::Ordering;

#[derive(EnumIter, Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub enum RoomIdentifier {
    Storage,
    NorthMess,
    Mess,
    SouthMess,
    MeetingRoom,
    Bridge,
    PassengersRoom,
    AirlockCorridor,
    AirlockA,
    OpenSpaceAirlockA,
    AirlockB,
    OpenSpaceAirlockB,
    TechCorridor,
    TechCorridorNorth,
    EngineRoom,
    BosunsRoom,
    CaptainsRoom,
    StorageHold,
    CrewCabins,

    StationAirlock
}

#[derive(Display, PartialEq, Clone, Copy)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Access {
    A,
    B,
    C,
    D,
    Broken,
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PassageType {
    Door,
    Free,
}


impl Access {
    fn value(&self) -> i32 {
        match self {
            Access::Broken => 6,
            Access::A => 5,
            Access::B => 4,
            Access::C => 3,
            Access::D => 2,
            Access::None => 0,
        }
    }
}

impl PartialOrd for Access {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Access {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}


pub struct RoomAttributes {
    pub room_identifier: RoomIdentifier,
    pub visited: bool,
    pub access: Access,
    pub short_description: String,
    pub full_description: String,
    pub first_thoughts: String,
    //pub potential_items: Vec<ItemBlueprint>,
    //pub furniture: Vec<Furniture>,
    pub entities: Vec<EntityId>,
    pub connected_rooms: Vec<(Direction, PassageType, RoomIdentifier)>,
}

// pub enum RoomBlueprint {
//     Storage(RoomAttributes),
//     Mess(RoomAttributes),
//     PassengersRoom(RoomAttributes),
//     BosunsRoom(RoomAttributes),
//     CaptainsRoom(RoomAttributes),
//     AirlockA(RoomAttributes),
//     StorageHold(RoomAttributes),
//     Engine(RoomAttributes),
//     Hallway(RoomAttributes),
//     Cockpit(RoomAttributes),
// }

// impl RoomBlueprint {
//     pub fn description(&self) -> &str {
//         match self {
//             RoomBlueprint::Storage(attributes) => &attributes.default_description,
//             RoomBlueprint::Mess(attributes) => &attributes.default_description,
//             RoomBlueprint::PassengersRoom(attributes) => &attributes.default_description,
//             RoomBlueprint::BosunsRoom(attributes) => &attributes.default_description,
//             RoomBlueprint::CaptainsRoom(attributes) => &attributes.default_description,
//             RoomBlueprint::AirlockA(attributes) => &attributes.default_description,
//             RoomBlueprint::StorageHold(attributes) => &attributes.default_description,
//             RoomBlueprint::Engine(attributes) => &attributes.default_description,
//             RoomBlueprint::Hallway(attributes) => &attributes.default_description,
//             RoomBlueprint::Cockpit(attributes) => &attributes.default_description,
//             _ => "Room not found."
//         }
//     }
// }