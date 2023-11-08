use std::any::Any;

use super::{Entity, EntityId, Item, ItemId};
use super::Size;
use super::Edible;
use crate::{impl_entity};

pub struct Food {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
}

impl Food {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String) -> Self {
        Food { id, name, aliases, description}
    }
}

impl Edible for Food {
    fn eat(&mut self) -> Result<(), &'static str> {
        //println!("You eat the {}.", self.name);
        Ok(())
    }
}

impl Entity for Food {
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
    fn as_edible(&self) -> Option<&dyn Edible> {
        Some(self)
    }
    fn as_edible_mut(&mut self) -> Option<&mut dyn Edible> { 
        Some(self)
    }
}