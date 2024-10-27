use std::borrow::Cow;

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumMessage};

use crate::world::room::Direction;

#[derive(strum_macros::Display, EnumIter, EnumMessage)]
pub enum Command {
    #[strum(message = "Look around or look at a specific object")]
    Look(Option<String>),

    #[strum(message = "Examine something in detail")]
    Examine(Option<String>),

    #[strum(message = "Open a door or container")]
    Open(String),

    #[strum(message = "Close a door or container")]
    Close(String),

    #[strum(message = "Move in a specific direction")]
    Go(Direction),

    #[strum(message = "Take an object")]
    Take(String),

    #[strum(message = "Drop an object")]
    Drop(String),

    #[strum(message = "Take an object from a container")]
    TakeFrom(String, String),

    #[strum(message = "Put an object into a container")]
    PutInto(String, String),

    #[strum(message = "Use an object")]
    Use(String),

    #[strum(message = "Enter a command or space")]
    Enter(String),

    #[strum(message = "Combine two items")]
    Combine(String, String),

    #[strum(message = "Push something")]
    Push(String),

    #[strum(message = "Pull something")]
    Pull(String),

    #[strum(message = "Turn something, like a knob or switch")]
    Turn(String),

    #[strum(message = "Read something, like a note")]
    Read(String),

    #[strum(message = "Eat something that is a food")]
    Eat(String),

    #[strum(message = "Check your inventory")]
    Inventory,

    #[strum(message = "Check your status or health")]
    Status,

    #[strum(message = "Show available commands")]
    Help,

    #[strum(message = "Save the game")]
    Save,

    #[strum(message = "Load the game")]
    Load,
}

pub fn parse(input: &str) -> Option<Command> {
    let input_lowercase = input.trim().to_lowercase();
    let words = sanitize_and_split(&input_lowercase);

    match words.iter().as_slice() {
        ["x", "room", "ls"] => Some(Command::Look(None)),
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

fn sanitize_and_split(input: &str) -> Vec<&str> {
    input
        .split_whitespace()
        .filter(|word| !is_article(word))
        .collect()
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
