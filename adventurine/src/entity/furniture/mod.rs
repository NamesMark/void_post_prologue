pub mod main_terminal;
pub mod navigation_computer;
pub mod sink;

use std::any::Any;
use strum_macros::EnumIter;

use super::item::Containable;
use super::{Entity, EntityId};
use crate::impl_entity_containable;

#[derive(EnumIter, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum FurnId {
    Illuminator,
    StorageShelf,
    MessTable,
    Counter,
    CoffeeMachine,
    FoodPrinter,
    Sink,
    BosunDesk,
    BookShelves,
    CaptainsIlluminator,
    CaptainsDesk,

    NavigationComputer,
    MainTerminal,

    MainEngine,
    FuelTankA,
    FuelTankB,
    EmergencyLocker,

    WarningSign,
    #[default]
    Dust,
}

impl_entity_containable!(Furniture);

pub struct Furniture {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    contains: Vec<EntityId>,
}

impl Furniture {
    pub fn new(
        id: EntityId,
        name: String,
        aliases: Vec<String>,
        description: String,
        contains: Vec<EntityId>,
    ) -> Self {
        Furniture {
            id,
            name,
            aliases,
            description,
            contains,
        }
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
