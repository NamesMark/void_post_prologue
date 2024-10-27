use std::any::Any;

use super::{Entity, EntityId};
use crate::entity::item::{Containable, ItemId, Usable};
use crate::{impl_entity, impl_entity_containable};

impl_entity!(MainTerminal);

pub struct MainTerminal {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    contains: Vec<EntityId>,
}

impl MainTerminal {
    pub fn new(
        id: EntityId,
        name: String,
        aliases: Vec<String>,
        description: String,
        contains: Vec<EntityId>,
    ) -> Self {
        MainTerminal {
            id,
            name,
            aliases,
            description,
            contains,
        }
    }

    pub fn enter_command() {
        todo!()
    }
}

pub enum MainTerminalCommand {
    ActivateMainEngine,
    DeactiveteMainEngine,
    ActivateMainThrust,
    DeactiveteMainThrust,
    SetMainThrustLevel(u8),
    MainEngineStatus,

    SetManeurXVector(i16),
    SetManeurYVector(i16),
    ActivateManeuverEngines,
    SetManeuverThrustLevel(u8),
    ManeuverEngineStatus,

    SwitchToFuelTankA,
    SwitchToFuelTankB,
    ActivateFuelPump,
    DeactivateFuelPump,
    FuelSystemStatus,
}

impl MainTerminalCommand {
    pub fn from_string(command_str: &str) -> Result<MainTerminalCommand, &'static str> {
        match command_str {
            "01::00::00" => Ok(MainTerminalCommand::DeactiveteMainEngine),
            "01::00::01" => Ok(MainTerminalCommand::ActivateMainEngine),
            "01::04::00" => Ok(MainTerminalCommand::ActivateMainThrust),
            "01::04::01" => Ok(MainTerminalCommand::DeactiveteMainThrust),
            cmd if cmd.starts_with("01::05::") => {
                let power = cmd[8..].parse::<u8>().map_err(|_| "Invalid power level")?;
                Ok(MainTerminalCommand::SetMainThrustLevel(power))
            }
            "01::09::00" => Ok(MainTerminalCommand::MainEngineStatus),

            cmd if cmd.starts_with("02::02::00::") => {
                let x_vector = cmd[12..].parse::<i16>().map_err(|_| "Invalid X vector")?;
                Ok(MainTerminalCommand::SetManeurXVector(x_vector))
            }
            cmd if cmd.starts_with("02::02::01::") => {
                let y_vector = cmd[12..].parse::<i16>().map_err(|_| "Invalid Y vector")?;
                Ok(MainTerminalCommand::SetManeurYVector(y_vector))
            }
            "02::04::00" => Ok(MainTerminalCommand::ActivateManeuverEngines),
            cmd if cmd.starts_with("02::05::") => {
                let power = cmd[8..].parse::<u8>().map_err(|_| "Invalid power level")?;
                Ok(MainTerminalCommand::SetManeuverThrustLevel(power))
            }
            "02::09::00" => Ok(MainTerminalCommand::ManeuverEngineStatus),

            "07::00::00" => Ok(MainTerminalCommand::DeactivateFuelPump),
            "07::00::01" => Ok(MainTerminalCommand::ActivateFuelPump),
            "07::01::00" => Ok(MainTerminalCommand::SwitchToFuelTankA),
            "07::01::01" => Ok(MainTerminalCommand::SwitchToFuelTankB),
            "07::09::00" => Ok(MainTerminalCommand::FuelSystemStatus),

            _ => Err("Invalid command"),
        }
    }
}

// impl Usable for MainTerminal {
//     fn r#use(&mut self, command: String) -> Result<String, &'static str> {
//         let parsed_command = MainTerminalCommand::from_string(&command)?;

//         Ok(format!("Command executed: {}", command))
//     }
// }
