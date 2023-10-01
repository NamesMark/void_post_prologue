use strum_macros::{EnumIter, Display};


#[derive(EnumIter, Eq, PartialEq, Hash)]
pub enum RoomIdentifier {
    Storage,
    Mess,
    PassengersRoom,
    BosunsRoom,
    CaptainsRoom,
    AirlockA,
    StorageHold,
    Engine,
    Hallway,
    Cockpit,
}

#[derive(Display)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

pub struct RoomAttributes {
    pub room_identifier: RoomIdentifier,
    pub default_description: String,
    pub first_thoughts: String,
    pub potential_items: Vec<ItemBlueprint>,
    pub connected_rooms: Vec<(Direction, RoomIdentifier)>,
}

pub enum RoomBlueprint {
    Storage(RoomAttributes),
    Mess(RoomAttributes),
    PassengersRoom(RoomAttributes),
    BosunsRoom(RoomAttributes),
    CaptainsRoom(RoomAttributes),
    AirlockA(RoomAttributes),
    StorageHold(RoomAttributes),
    Engine(RoomAttributes),
    Hallway(RoomAttributes),
    Cockpit(RoomAttributes),
}

impl RoomBlueprint {
    pub fn description(&self) -> &str {
        match self {
            RoomBlueprint::Storage(attributes) => &attributes.default_description,
            RoomBlueprint::Mess(attributes) => &attributes.default_description,
            RoomBlueprint::PassengersRoom(attributes) => &attributes.default_description,
            RoomBlueprint::BosunsRoom(attributes) => &attributes.default_description,
            RoomBlueprint::CaptainsRoom(attributes) => &attributes.default_description,
            RoomBlueprint::AirlockA(attributes) => &attributes.default_description,
            RoomBlueprint::StorageHold(attributes) => &attributes.default_description,
            RoomBlueprint::Engine(attributes) => &attributes.default_description,
            RoomBlueprint::Hallway(attributes) => &attributes.default_description,
            RoomBlueprint::Cockpit(attributes) => &attributes.default_description,
            _ => "Room not found."
        }
    }
}