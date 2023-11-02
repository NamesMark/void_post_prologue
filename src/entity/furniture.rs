use strum_macros::EnumIter;
use super::{Entity, EntityId};
use crate::impl_entity;

#[derive(EnumIter, Default, Debug, PartialEq, Eq, Hash)]
pub enum FurnId {
    Illuminator,
    StorageShelf,
    MessTable,
    Counter,
    CoffeeMachine,
    FoodPrinter,
    
    #[default]
    Nothing
}

impl_entity!(Furniture);

pub struct Furniture {
    name: String,
    description: String,
    contains: Vec<EntityId>, 
}

impl Furniture {
    pub fn new(name: String, description: String, contains: Vec<EntityId>) -> Self {
        Furniture { name, description, contains }
    }
}
