pub mod item;
pub mod furniture;

use furniture::FurnId;
use item::ItemId;
use strum_macros::EnumIter;

#[derive(EnumIter, Default, Debug, PartialEq, Eq, Hash)]
pub enum EntityId {
    Furniture(FurnId),
    Item(ItemId),
    #[default]
    Nothing,
}

pub trait Entity {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

pub struct PassiveEntity {
    pub name: String,
    pub description: String,
}

impl PassiveEntity {
    pub fn new(name: String, description: String) -> Self {
        PassiveEntity { name, description}
    }
}

#[macro_export]
macro_rules! impl_entity {
    ($($type:ty),*) => {
        $(impl crate::entity::Entity for $type {
            fn name(&self) -> &str {
                &self.name
            }
        
            fn description(&self) -> &str {
                &self.description
            }
        })*
    };
}

impl_entity!(PassiveEntity);