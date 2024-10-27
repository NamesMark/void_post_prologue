use std::any::Any;

use super::Readable;
use super::Size;
use super::{Entity, EntityId, Item, ItemId};
use crate::impl_entity;

pub struct TextItem {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    contents: String,
}

impl TextItem {
    pub fn new(
        id: EntityId,
        name: String,
        aliases: Vec<String>,
        description: String,
        contents: String,
    ) -> Self {
        TextItem {
            id,
            name,
            aliases,
            description,
            contents,
        }
    }
}

impl Readable for TextItem {
    fn read(&mut self) -> Result<&String, &'static str> {
        // println!("You read the {}:", self.name);
        // println!("{}", self.contents);
        Ok((&self.contents))
    }
}

impl Entity for TextItem {
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
    fn as_readable(&self) -> Option<&dyn Readable> {
        Some(self)
    }
    fn as_readable_mut(&mut self) -> Option<&mut dyn Readable> {
        Some(self)
    }
}
