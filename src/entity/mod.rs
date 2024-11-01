pub mod furniture;
pub mod item;

use furniture::FurnId;
use item::{Containable, Edible, ItemId, Readable, Usable};
use std::any::Any;
use strum_macros::{Display, EnumIter};

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
    fn as_containable(&self) -> Option<&dyn Containable> {
        None
    }
    fn as_containable_mut(&mut self) -> Option<&mut dyn Containable> {
        None
    }
    fn as_edible(&self) -> Option<&dyn Edible> {
        None
    }
    fn as_edible_mut(&mut self) -> Option<&mut dyn Edible> {
        None
    }
    fn as_readable(&self) -> Option<&dyn Readable> {
        None
    }
    fn as_readable_mut(&mut self) -> Option<&mut dyn Readable> {
        None
    }
    fn as_usable(&self) -> Option<&dyn Usable> {
        None
    }
    fn as_usable_mut(&mut self) -> Option<&mut dyn Usable> {
        None
    }
}

pub struct PassiveEntity {
    pub id: EntityId,
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
}

impl PassiveEntity {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String) -> Self {
        PassiveEntity {
            id,
            name,
            aliases,
            description,
        }
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
                fn as_containable_mut(&mut self) -> Option<&mut dyn crate::entity::Containable> {
                    Some(self)
                }
            }
        )*
    }
}

impl_entity!(PassiveEntity);
