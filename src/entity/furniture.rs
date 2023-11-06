use strum_macros::EnumIter;
use std::any::Any;
use super::{Entity, EntityId};
use super::item::{ItemId, Containable};
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
    MainTerminal,
    #[default]
    Dust
}

impl_entity!(Furniture, Sink, MainTerminal);

pub struct Furniture {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    contains: Vec<EntityId>, 
}

impl Furniture {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String, contains: Vec<EntityId>) -> Self {
        Furniture { id, name, aliases, description, contains }
    }
}

pub struct Sink {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    contains: Vec<EntityId>, 
    tap_is_on: bool,
}

impl Sink {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String, contains: Vec<EntityId>) -> Self {
        Sink { id, name, aliases, description, contains, tap_is_on: false }
    }
}

pub struct MainTerminal {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    contains: Vec<EntityId>, 
}

impl MainTerminal {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String, contains: Vec<EntityId>) -> Self {
        MainTerminal { id, name, aliases, description, contains}
    }

    pub fn enter_command() {
        todo!()
    }
}