use std::collections::HashMap;

use crate::engine::state::GameState;
use crate::world::room::Direction;
use crate::entity::{Entity, EntityId};

pub fn look(game_state: &GameState) -> String {
    // Fetch the room blueprint with RoomIdentifier
    if let Some(room_attributes) = game_state.world.rooms.get(&game_state.current_room) {
        room_attributes.full_description.clone()
    } else {
        "Room not found in the world. Are you in space? Oh crap".to_string()
    }
}

pub fn look_at(game_state: &GameState, obj_name: &str) -> String {
    let obj_name = obj_name.to_lowercase();
    let article = get_article(&obj_name);
    
    match find_entity(game_state, &obj_name) {
        Some(entity) => look_at_helper(article, entity.name(), entity.description()),
        None => format!("There is no {} here to look at.", obj_name),
    }
}

fn look_at_helper(article: &str, name: &str, description: &str) -> String {
    format!("You look at {}{}:\n{}", article, name, description)
}

fn get_article(obj_name: &str) -> &str {
    if obj_name.ends_with('s') { "" } else { "the " }
}

pub fn move_in_direction(game_state: &mut GameState, direction: Direction) -> Result<String, String> {
    match game_state.world.get_adjacent_room(&game_state.current_room, direction) {
        Some(new_room) => {
            game_state.current_room = new_room;
            if !game_state.was_current_room_visited() {
                game_state.world.set_visited(&game_state.current_room);
                Ok(format!("{}\n{}", game_state.current_room_first_thoughts(), game_state.current_room_description()))
            } else {
                Ok(game_state.current_room_description().to_string())
            }
        },
        None => Err(format!("Can't go in the direction of {}.", direction))
    }
}

pub fn open(game_state: &GameState, obj: &str) -> String {
    format!("You try to open the {}.", obj)
}

pub fn close(game_state: &GameState, obj: &str) -> String {
    format!("You try to close the {}.", obj)
}

pub fn pick_up(game_state: &mut GameState, obj_name: &str) -> String {
    let obj_name = obj_name.to_lowercase();
    let article = get_article(&obj_name);

    match find_entity(game_state, &obj_name) {
        Some(entity) => {
            // Add to the inventory
            //game_state.inventory.insert(entity.id());

            // Remove from the room
            if let Some(room) = game_state.world.rooms.get_mut(&game_state.current_room) {
                room.entities.retain(|&e| e != entity.id());
            }

            format!("You pick up {}{} and look at it: {}", article, entity.name(), entity.description())
        },
        None => format!("There is no {} here.", obj_name),
    }
}

fn find_entity<'a>(game_state: &'a GameState, obj_name: &str) -> Option<&'a dyn Entity> {
    let entity_ids = game_state.current_room_entities();

    for entity_id in entity_ids {
        if let Some(entity) = entity_map.get(entity_id) {
            if entity.name().to_lowercase() == obj_name {
                return Some(entity.as_ref());
            }
        }
    }

    None
}