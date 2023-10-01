use crate::world::room::Direction;

pub enum Command {
    Look(Option<String>),  // Look at something specific
    Examine(Option<String>),  // Examine something in detail
    
    Open(String),         // Open something
    Close(String),        // Close something

    Go(Direction),        // Move in a direction
    Enter(String),        // Enter something
    
    // Interaction
    Take(String),         // Take an object
    Drop(String),         // Drop an object
    Use(String),          // Use an object
    Combine(String, String), // Combine two items
    Push(String),         // Push something
    Pull(String),         // Pull something
    Turn(String),         // Turn something (like a knob or switch)
    Read(String),         // Read something (like a note)
    
    // Communication
    // TalkTo(String),       // Talk to a character
    // Give(String, String), // Give an item to someone
    
    // Inventory & status
    Inventory,            // Check your items
    Status,               // Check player's status or health
    
    // Misc
    Help,                 // Show available commands
    Save,                 // Save the game
    Load,                 // Load the game

    // TODO: always can add more commands lol
}

pub fn parse(input: &str) -> Option<Command> {
    let parts: Vec<&str> = input.trim().to_lowercase().split_whitespace().collect();
    match parts.as_slice() {
        ["look", obj] | ["examine", obj] | ["x", obj] => Some(Command::Look(Some(obj.to_string()))),
        ["look"] | ["examine"] | ["x"] => Some(Command::Look(None)),

        ["open", obj] | ["o", obj] => Some(Command::Open(obj.to_string())),
        ["close", obj] | ["c", obj] => Some(Command::Close(obj.to_string())),

        ["take", obj] => Some(Command::Take(obj.to_string())),
        ["drop", obj] => Some(Command::Drop(obj.to_string())),
        ["use", obj] => Some(Command::Use(obj.to_string())),
        ["combine", obj1, "with", obj2] => Some(Command::Combine(obj1.to_string(), obj2.to_string())),
        ["push", obj] => Some(Command::Push(obj.to_string())),
        ["pull", obj] => Some(Command::Pull(obj.to_string())),
        ["turn", obj] => Some(Command::Turn(obj.to_string())),
        ["read", obj] => Some(Command::Read(obj.to_string())),
        // ["talk", "to", person] => Some(Command::TalkTo(person.to_string())),
        // ["give", obj, "to", person] => Some(Command::Give(obj.to_string(), person.to_string())),

        ["inventory"] | ["i"] => Some(Command::Inventory),
        ["status"] => Some(Command::Status),
        
        ["help"] | ["h"] => Some(Command::Help),
        ["save"] => Some(Command::Save),
        ["load"] => Some(Command::Load),

        ["north"] | ["n"] => Some(Command::Go(Direction::North)),
        ["east"] | ["e"] => Some(Command::Go(Direction::East)),
        ["south"] | ["s"] => Some(Command::Go(Direction::South)),
        ["west"] | ["w"] => Some(Command::Go(Direction::West)),
        ["up"] | ["u"] => Some(Command::Go(Direction::Up)),
        ["down"] | ["d"] => Some(Command::Go(Direction::Down)),
        _ => None,
    }
}