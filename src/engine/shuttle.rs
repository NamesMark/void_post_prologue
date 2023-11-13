use crate::entity::furniture::main_terminal::MainTerminalCommand;

pub struct ShuttleState {
    main_engine_on: bool,
    main_engine_power: u8, // 0-63
    maneuver_vector: (i16, i16), // x, y, z (0-360 each)
    maneuver_power: u8, // 0-7
    fuel_tank: FuelTank,
    fuel_pump_on: bool,
}

#[derive(PartialEq)]
enum FuelTank {
    A,
    B,
}

impl ShuttleState {
    pub fn new() -> Self {
        ShuttleState {
            main_engine_on: false,
            main_engine_power: 0,
            maneuver_vector: (0, 0),
            maneuver_power: 0,
            fuel_tank: FuelTank::A,
            fuel_pump_on: false,
        }
    }

    pub fn handle_command(&mut self, command: MainTerminalCommand) -> Result<String, String> {
        match command {
            MainTerminalCommand::ActivateMainEngine => self.activate_main_engine(),
            MainTerminalCommand::DeactiveteMainEngine => self.deactivate_main_engine(),
            MainTerminalCommand::ActivateMainThrust => self.activate_main_thrust(),
            MainTerminalCommand::DeactiveteMainThrust => self.deactivate_main_thrust(),
            MainTerminalCommand::SetMainThrustLevel(level) => self.set_main_thrust_level(level),
            MainTerminalCommand::MainEngineStatus => self.main_engine_status(),

            MainTerminalCommand::SetManeurXVector(x) => self.set_maneuver_vector_x(x),
            MainTerminalCommand::SetManeurYVector(y) => self.set_maneuver_vector_y(y),
            MainTerminalCommand::ActivateManeuverEngines => self.activate_maneuver_engines(),
            MainTerminalCommand::SetManeuverThrustLevel(level) => self.set_maneuver_thrust_level(level),
            MainTerminalCommand::ManeuverEngineStatus => self.maneuver_engine_status(),

            MainTerminalCommand::SwitchToFuelTankA => self.switch_to_fuel_tank_a(),
            MainTerminalCommand::SwitchToFuelTankB => self.switch_to_fuel_tank_b(),
            MainTerminalCommand::ActivateFuelPump => self.activate_fuel_pump(),
            MainTerminalCommand::DeactivateFuelPump => self.deactivate_fuel_pump(),
            MainTerminalCommand::FuelSystemStatus => self.fuel_system_status(),
        }
    }

    fn activate_main_engine(&mut self) -> Result<String, String> {
        if self.fuel_tank == FuelTank::B && self.fuel_pump_on {
            self.main_engine_on = true;
            Ok("Main engine started.".to_string())
            
        } else {
            Err("Main engine start failure: no fuel.".to_string())
        }
    }

    fn deactivate_main_engine(&mut self) -> Result<String, String> {
        if self.main_engine_on {
            self.main_engine_on = false;
            Ok("Main engine deactivated.".to_string())
        } else {
            Err("Main engine is already off.".to_string())
        }
    }

    fn activate_main_thrust(&mut self) -> Result<String, String> {
        if self.main_engine_on && self.main_engine_power > 0 {
            if self.main_engine_power > 0 && self.main_engine_power <= 10 {
                Err("You are mesmerized by the station becoming smaller and smaller as the shuttle slowly drifts further into the deep space, until it completely disappears. You've used up the rest of the fuel, and there's no way back. 
			
                You'll live for days, maybe weeks, while the life support systems will be slowly shutting off. Maybe there's hope that some ship will still pick you up, maybe not. But you aren't in a good spot for sure.
                
                You've navigated away from the station, and you are completely lost in space.".to_string())
            } else {
                Err("You are thrown in the captains chair. Stars become smears, the Gs feel like an ogre's foot on your chest. You are getting farther and farther into deep space at an alarming rate. 
			
                Suddenly, the accelleration stops. You realize the fuel ran out. The lights blink once again and turn off.
                
                This is the end.".to_string())
            }
            //std::process::exit(0);
            //Ok("Main thrust activated.".to_string())
        } else {
            Err("Cannot activate thrust. Ensure main engine is on and power is set.".to_string())
        }
    }

    fn deactivate_main_thrust(&mut self) -> Result<String, String> {
        // this does nothing intentionally.
        Ok("Main thrust deactivated.".to_string())
    }

    fn set_main_thrust_level(&mut self, level: u8) -> Result<String, String> {
        if level > 63 {
            Err("Invalid thrust level. Must be between 0 and 63.".to_string())
        } else {
            self.main_engine_power = level;
            Ok(format!("Main engine power set to {}.", level))
        }
    }

    fn main_engine_status(&self) -> Result<String, String> {
        let status = format!(
            "Main engine is {}. Power level: {}.",
            if self.main_engine_on { "on" } else { "off" },
            self.main_engine_power
        );
        Ok(status)
    }

    fn set_maneuver_vector_x(&mut self, x: i16) -> Result<String, String> {
        if x < 0 || x > 360 {
            Err("Invalid X vector. Must be between 0 and 360.".to_string())
        } else {
            self.maneuver_vector.0 = x;
            Ok(format!("Maneuver X vector set to {}.", x))
        }
    }

    fn set_maneuver_vector_y(&mut self, y: i16) -> Result<String, String> {
        if y < 0 || y > 360 {
            Err("Invalid Y vector. Must be between 0 and 360.".to_string())
        } else {
            self.maneuver_vector.1 = y;
            Ok(format!("Maneuver Y vector set to {}.", y))
        }
    }

    fn set_maneuver_thrust_level(&mut self, level: u8) -> Result<String, String> {
        if level > 7 {
            Err("Invalid thrust level for maneuver engines. Must be between 0 and 7.".to_string())
        } else {
            self.maneuver_power = level;
            Ok(format!("Maneuver thrust level set to {}.", level))
        }
    }

    fn maneuver_engine_status(&self) -> Result<String, String> {
        let status = format!(
            "Maneuver engines are {}. X vector: {}. Y vector: {}. Power level: {}.",
            if self.maneuver_power > 0 { "on" } else { "off" },
            self.maneuver_vector.0,
            self.maneuver_vector.1,
            self.maneuver_power
        );
        Ok(status)
    }

    fn switch_to_fuel_tank_a(&mut self) -> Result<String, String> {
        self.fuel_tank = FuelTank::A;
        Ok("Switched to fuel tank A.".to_string())
    }

    fn activate_fuel_pump(&mut self) -> Result<String, String> {
        self.fuel_pump_on = true;
        Ok("Fuel pump activated.".to_string())
    }

    fn deactivate_fuel_pump(&mut self) -> Result<String, String> {
        self.fuel_pump_on = false;
        Ok("Fuel pump deactivated.".to_string())
    }

    fn fuel_system_status(&self) -> Result<String, String> {
        let status = format!(
            "Fuel tank: {}. Fuel pump is {}.",
            match self.fuel_tank {
                FuelTank::A => "A",
                FuelTank::B => "B",
            },
            if self.fuel_pump_on { "on" } else { "off" }
        );
        Ok(status)
    }

    pub fn activate_maneuver_engines(&mut self) -> Result<String, String> {
        let x_vector = self.maneuver_vector.0;
        let y_vector = self.maneuver_vector.1;
        let power = self.maneuver_power;
        if self.fuel_tank != FuelTank::B || !self.fuel_pump_on {
            return Err("Maneuver engine start failure: No fuel.".to_string());
        }

        if x_vector < 0 || x_vector > 360 || y_vector < 0 || y_vector > 360 {
            return Err("Invalid vector setting: Out of range. Must be between 0 and 360.".to_string());
        }

        if power < 0 || power > 7 {
            return Err("Invalid power setting for maneuver engines.".to_string());
        }

        if self.is_correct_approach_for_docking() {
            Ok("You have successfully maneuvered the shuttle for docking. Congratulations, you've docked at the station!".to_string())
        } else {
            self.check_failed_maneuvering()
        }
    }

    fn is_correct_approach_for_docking(&self) -> bool {
        (self.maneuver_vector.0 >= 340 && self.maneuver_vector.0 <= 350) &&
        ((self.maneuver_vector.1 >= 350 && self.maneuver_vector.1 <= 360) ||
        (self.maneuver_vector.1 >= 0 && self.maneuver_vector.1 <= 10)) &&
        self.maneuver_power >= 1 && self.maneuver_power <= 5
    }

    fn check_failed_maneuvering(&self) -> Result<String, String> {
        // Checks for various failed maneuvering scenarios
        if self.maneuver_vector.0 < 181 || self.maneuver_vector.0 > 359 || self.maneuver_vector.1 < 240 || self.maneuver_vector.1 > 320 {
            Err("You carefully navigate the shuttle... away from the station. Oh no! You have no idea where the station went, as it's no longer visible in any of the view ports. Now you'll die from cold as the shuttle tumbles through the void, no habitable worlds or trade routes for lightyears around you.
				
            You lost.".to_string())
        } else {
            Err("As if in slow motion, the shuttle floats towards the station. Your hope that the approach vector is correct is quickly crushed - just as the shuttle got crushed against the station. 
            
            You lost.".to_string())
        }
    }


    fn switch_to_fuel_tank_b(&mut self) -> Result<String, String> {
        self.fuel_tank = FuelTank::B;
        Ok("Switched to fuel tank B.".to_string())
    }

}