use crate::{impl_entity, impl_entity_containable};
use super::{Entity, EntityId};
use crate::entity::item::{ItemId, Containable};

impl_entity!(Sink);

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