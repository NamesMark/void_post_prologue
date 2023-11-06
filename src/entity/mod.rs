pub mod item;
pub mod furniture;

use std::any::Any;


use furniture::FurnId;
use item::{ItemId, Containable};
use strum_macros::{EnumIter, Display};

#[derive(EnumIter, Default, Debug, Display, PartialEq, Eq, Hash, Clone, Copy)]
pub enum EntityId {
    Item(ItemId),
    Furniture(FurnId),
    #[default]
    Dust,
}

pub trait Entity {
    fn get_id(&self) -> EntityId;
    fn name(&self) -> &str;
    fn aliases(&self) -> &Vec<String>;
    fn description(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_containable(&self) -> Option<&dyn Containable>;
}

pub struct PassiveEntity {
    pub id: EntityId,
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
}

impl PassiveEntity {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String) -> Self {
        PassiveEntity { id, name, aliases, description}
    }
}

#[macro_export]
macro_rules! impl_entity {
    ($($type:ty),*) => {
        $(impl crate::entity::Entity for $type {
            fn get_id(&self) -> EntityId {
                self.id
            }

            fn name(&self) -> &str {
                &self.name
            }

            fn aliases(&self) -> &Vec<String> {
                &self.aliases
            }
        
            fn description(&self) -> &str {
                &self.description
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
            fn as_containable(&self) -> Option<&dyn crate::entity::Containable> {
                None
            }
        })*
    };
}

#[macro_export]
macro_rules! impl_entity_containable {
    ($($type:ty),*) => {
        $(
            impl crate::entity::Entity for $type {
                fn get_id(&self) -> EntityId {
                    self.id
                }
    
                fn name(&self) -> &str {
                    &self.name
                }
    
                fn aliases(&self) -> &Vec<String> {
                    &self.aliases
                }
            
                fn description(&self) -> &str {
                    &self.description
                }
    
                fn as_any(&self) -> &dyn Any {
                    self
                }
                fn as_any_mut(&mut self) -> &mut dyn Any {
                    self
                }
                fn as_containable(&self) -> Option<&dyn crate::entity::Containable> {
                    Some(self)
                }
            }
        )*
    }
}

impl_entity!(PassiveEntity);