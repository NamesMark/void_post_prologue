use strum_macros::EnumIter;
use super::{Entity, EntityId};
use crate::impl_entity;

#[derive(EnumIter, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FurnId {
    Illuminator,
    StorageShelf,
    MessTable,
    Counter,
    CoffeeMachine,
    FoodPrinter,
    Sink,
    #[default]
    Nothing
}

impl_entity!(Furniture, Sink);

pub struct Furniture {
    id: EntityId,
    name: String,
    description: String,
    contains: Vec<EntityId>, 
}

impl Furniture {
    pub fn new(id: EntityId, name: String, description: String, contains: Vec<EntityId>) -> Self {
        Furniture { id, name, description, contains }
    }
}

pub struct Sink {
    id: EntityId,
    name: String,
    description: String,
    contains: Vec<EntityId>, 
    tap_is_on: bool,
}

impl Sink {
    pub fn new(id: EntityId, name: String, description: String, contains: Vec<EntityId>) -> Self {
        Sink { id, name, description, contains, tap_is_on: false }
    }
}