pub enum Command {
    Look,
    Open,
    Close,
    Go(Direction),
    Enter,


    // TODO: always can add more commands lol
}

pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

pub fn parse(input: &str) -> Option<Command> {
    match input.trim().to_lowercase().as_str() {
        "look" | "examine" | "x" => Some(Command::Look),
        "open" | "o" => Some(Command::Open),
        "close" | "c" => Some(Command::Close),
        "north" | "n" => Some(Command::Go(Direction::North)),
        "east" | "e" => Some(Command::Go(Direction::East)),
        "south" | "s" => Some(Command::Go(Direction::South)),
        "west" | "w" => Some(Command::Go(Direction::West)),
        "up" | "u" => Some(Command::Go(Direction::Up)),
        "down" | "d" => Some(Command::Go(Direction::Down)),

        // TODO: always can add more commands lol
        _ => None,
    }
}