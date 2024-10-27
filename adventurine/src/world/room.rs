use crate::entity::EntityId;
use std::cmp::Ordering;
use strum_macros::{Display, EnumIter};

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

    StationAirlock,
}

#[derive(Default, Display, PartialEq, Clone, Copy)]
pub enum Direction {
    #[default]
    Stay,
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
    pub entities: Vec<EntityId>,
    pub connected_rooms: Vec<(Direction, PassageType, RoomIdentifier)>,
}
