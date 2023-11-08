use std::any::Any;

use super::{Entity, EntityId, Item, ItemId};
use super::Size;
use super::Containable;
use crate::{impl_entity, impl_entity_containable};

impl_entity_containable!(Container);

pub struct Container {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    contains: Vec<EntityId>,
    size: Size,
}

impl Container {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String, contains: Vec<EntityId>, size: Size) -> Self {
        Container { id, name, aliases, description, contains, size }
    }
    pub fn as_container(entity: &dyn Entity) -> Option<&Container> {
        entity.as_any().downcast_ref::<Container>()
    }
    fn remove(&mut self, item: ItemId) -> Option<Item> {
        todo!()
    }

}

impl Containable for Container {
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