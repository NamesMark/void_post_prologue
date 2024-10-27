use std::any::Any;

use super::{Entity, EntityId};
use crate::entity::item::{Containable, ItemId, Usable};
use crate::{impl_entity, impl_entity_containable};

pub struct NavigationComputer {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    contains: Vec<EntityId>,
}

impl NavigationComputer {
    pub fn new(
        id: EntityId,
        name: String,
        aliases: Vec<String>,
        description: String,
        contains: Vec<EntityId>,
    ) -> Self {
        NavigationComputer {
            id,
            name,
            aliases,
            description,
            contains,
        }
    }

    // pub fn enter_command() {
    //     todo!()
    // }
}

impl Entity for NavigationComputer {
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

    fn as_usable(&self) -> Option<&dyn Usable> {
        Some(self)
    }
    fn as_usable_mut(&mut self) -> Option<&mut dyn Usable> {
        Some(self)
    }
}

impl Usable for NavigationComputer {
    fn r#use(&mut self) -> Result<String, &'static str> {
        Ok("In a few clicks you are able to get the terminal to output the correct approach vector to dock the station: 'X344 Y351 P3 /END'.".to_string())
    }
}
