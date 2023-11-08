use crate::{impl_entity, impl_entity_containable};
use super::{Entity, EntityId};
use crate::entity::item::{ItemId, Containable};

impl_entity!(MainTerminal);

pub struct MainTerminal {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    contains: Vec<EntityId>, 
}

impl MainTerminal {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String, contains: Vec<EntityId>) -> Self {
        MainTerminal { id, name, aliases, description, contains}
    }

    pub fn enter_command() {
        todo!()
    }
}

enum MainTerminalCommand {
    ActivateMainEngines,
    ActivateManeurEngines,
    SwitchToFuelTankB,

}