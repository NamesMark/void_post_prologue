use std::borrow::Cow;

use crate::world::room::Direction;

pub enum Command {
    Look(Option<String>),    // Look at something specific
    Examine(Option<String>), // Examine something in detail

    Open(String),  // Open something
    Close(String), // Close something

    Go(Direction), // Move in a direction
    //Enter(String),        // Enter something

    // Interaction
    Take(String),             // Take an object
    Drop(String),             // Drop an object
    TakeFrom(String, String), // Take from a container
    PutInto(String, String),  // Put into a container
    Use(String),              // Use an object
    Enter(String),            // Enter a command
    Combine(String, String),  // Combine two items
    Push(String),             // Push something
    Pull(String),             // Pull something
    Turn(String),             // Turn something (like a knob or switch)
    Read(String),             // Read something (like a note)
    Eat(String),              // Eat something that's a food

    // Communication
    // TalkTo(String),       // Talk to a character
    // Give(String, String), // Give an item to someone

    // Inventory & status
    Inventory, // Check your items
    Status,    // Check player's status or health

    // Misc
    Help, // Show available commands
    Save, // Save the game
    Load, // Load the game

          // TODO: always can add more commands lol
}

pub fn parse(input: &str) -> Option<Command> {
    let sanitized_words = sanitize_and_split(input);
    let words: Vec<&str> = sanitized_words.iter().map(|s| s.as_str()).collect();

    match words.iter().as_slice() {
        ["x", "room"] => Some(Command::Look(None)),
        ["look", obj] | ["examine", obj] | ["x", obj] => Some(Command::Look(Some(obj.to_string()))),
        ["look"] | ["examine"] | ["x"] => Some(Command::Look(None)),

        ["open", obj] | ["o", obj] => Some(Command::Open(obj.to_string())),
        ["close", obj] | ["c", obj] => Some(Command::Close(obj.to_string())),

        ["take", obj] | ["get", obj] | ["pick", "up", obj] => Some(Command::Take(obj.to_string())),
        ["take", obj, "from", cont]
        | ["get", obj, "from", cont]
        | ["pick", "up", obj, "from", cont]
        | ["pick", obj, "from", cont]
        | ["retrieve", obj, "from", cont] => {
            Some(Command::TakeFrom(obj.to_string(), cont.to_string()))
        }
        ["put", obj, "into", cont] | ["place", obj, "into", cont] => {
            Some(Command::PutInto(obj.to_string(), cont.to_string()))
        }

        ["drop", obj] => Some(Command::Drop(obj.to_string())),
        ["use", obj] => Some(Command::Use(obj.to_string())),
        ["enter", command] => Some(Command::Enter(command.to_string())),
        //["combine", obj1, "with", obj2] => Some(Command::Combine(obj1.to_string(), obj2.to_string())),
        //["push", obj] => Some(Command::Push(obj.to_string())),
        //["pull", obj] => Some(Command::Pull(obj.to_string())),
        ["turn", obj] => Some(Command::Turn(obj.to_string())),
        ["read", obj] => Some(Command::Read(obj.to_string())),
        // ["talk", "to", person] => Some(Command::TalkTo(person.to_string())),
        // ["give", obj, "to", person] => Some(Command::Give(obj.to_string(), person.to_string())),
        ["eat", obj] | ["consume", obj] => Some(Command::Eat(obj.to_string())),

        ["inventory"] | ["i"] => Some(Command::Inventory),
        ["status"] => Some(Command::Status),

        ["help"] | ["h"] => Some(Command::Help),
        ["save"] => Some(Command::Save),
        ["load"] => Some(Command::Load),

        ["north"] | ["n"] | ["go", "north"] | ["go", "n"] => Some(Command::Go(Direction::North)),
        ["east"] | ["e"] | ["go", "east"] | ["go", "e"] => Some(Command::Go(Direction::East)),
        ["south"] | ["s"] | ["go", "south"] | ["go", "s"] => Some(Command::Go(Direction::South)),
        ["west"] | ["w"] | ["go", "west"] | ["go", "w"] => Some(Command::Go(Direction::West)),
        ["up"] | ["u"] | ["go", "up"] | ["go", "u"] => Some(Command::Go(Direction::Up)),
        ["down"] | ["d"] | ["go", "down"] | ["go", "d"] => Some(Command::Go(Direction::Down)),
        _ => None,
    }
}

fn sanitize_and_split(input: &str) -> Vec<String> {
    let input_lowercase = input.trim().to_lowercase();
    // TODO: should be possible to optimize
    let parts = input_lowercase
        .split_whitespace()
        .filter(|word| !is_article(word))
        .map(|w| w.to_string())
        .collect();
    parts
}

const ARTICLES: [&str; 3] = ["the", "a", "an"];

fn is_article(word: &str) -> bool {
    ARTICLES.iter().any(|&a| a == word)
}

fn strip_articles_and_join(words: &[&str]) -> String {
    words
        .iter()
        .filter(|&word| is_article(word))
        .cloned()
        .collect::<Vec<&str>>()
        .join(" ")
}
