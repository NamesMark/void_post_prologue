use strum_macros::EnumIter;
use std::any::Any;
use super::{Entity, EntityId};
use super::item::{ItemId, Containable};
use crate::{impl_entity, impl_entity_containable};

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
    BosunDesk,

    MainEngine,
    FuelTankA,
    FuelTankB,


    WarningSign,
    #[default]
    Dust
}

impl_entity!(Sink, MainTerminal);
impl_entity_containable!(Furniture);

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

impl Containable for Furniture {
    fn can_contain(&self, entity_id: EntityId) -> bool {
        !self.contains.contains(&entity_id)
    }
    fn put(&mut self, entity_id: EntityId) -> Result<(), String> {
        if self.can_contain(entity_id) {
            self.contains.push(entity_id);
            Ok(())
        } else {
            Err(format!("The {} cannot contain any more items.", self.name))
        }
    }
    fn remove(&mut self, entity_id: EntityId) -> Result<(), String> {
        if let Some(index) = self.contains.iter().position(|&id| id == entity_id) {
            self.contains.remove(index);
            Ok(())
        } else {
            Err(format!("{} is not in the {}.", entity_id, self.name))
        }
    }
    fn contains(&self) -> &Vec<EntityId> {
        &self.contains
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

enum MainTerminalCommand {
    ActivateMainEngines,
    ActivateManeurEngines,
    SwitchToFuelTankB,

}