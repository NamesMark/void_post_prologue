use std::collections::HashMap;
use std::cmp::max;
use rand::prelude::SliceRandom;

use crate::engine::state::GameState;
use crate::world::room::{Direction, Access};
use crate::entity::{Entity, EntityId};
use crate::entity::item::ItemId;


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

    match find_entity_in_room(game_state, &obj_name) {
        Some(entity) => look_at_helper(article, entity.name(), entity.description()),
        None => match find_entity_in_inventory(game_state, &obj_name) {
            Some(entity) => look_at_helper(article, entity.name(), entity.description()),
            None => format!("There is no {} here to look at.", obj_name),
        },
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
            if get_player_access(game_state) < *game_state.world.get_room_access(&new_room) {
                return Err(format!("The door beeps with an unsatisfied tone."));
            }
            game_state.current_room = new_room;
            print_any!(
                "The door beeps with quiet acknowledgement and slides aside.",
                "*Shhhhht* - the door slides open.",
                "The door opens with no apparent effort from your side.",
                "The door opened so fast as if it predicted your intention."
            );
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

pub fn get_player_access(game_state: &mut GameState) -> Access {
    let mut highest_access = Access::None;

    for item_id in &game_state.inventory {
        highest_access = match item_id {
            ItemId::CaptainCard => highest_access.max(Access::A),
            ItemId::BosunCard => highest_access.max(Access::B),
            ItemId::AssistantCard => highest_access.max(Access::C),
            _ => highest_access,
        };
    }

    return highest_access
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

    if let Some(entity_ref) = find_entity_in_room(game_state, &obj_name) {
        let entity_id = entity_ref.get_id();

        // Remove from the room
        if let Some(room) = game_state.world.rooms.get_mut(&game_state.current_room) {
            room.entities.retain(|&e| e != entity_id);
        }

        // Add to the inventory
        match entity_id {
            EntityId::Item(item_id) => {
                game_state.inventory.push(item_id);

                if let Some(entity) = game_state.world.entities.get(&entity_id) {
                    format!("You pick up {}{} and look at it: {}", article, entity.name(), entity.description())
                } else {
                    format!("There seems to be a problem picking up the {}.", obj_name)
                }
            },
            _ => format!("You can't pick up the {}.", obj_name),
        }
    } else {
        format!("There is no {} here.", obj_name)
    }
}

fn find_entity_in_room<'a>(game_state: &'a GameState, obj_name: &str) -> Option<&'a dyn Entity> {
    let search_name = obj_name.to_lowercase();

    if let Some(room_entity_ids) = game_state.current_room_entities() {
        for entity_id in room_entity_ids {
            if let Some(entity) = game_state.world.entities.get(entity_id) {
                if entity.name().to_lowercase() == search_name {
                    return Some(entity.as_ref());
                }
            }
        }
    }

    None
}

fn find_entity_in_inventory<'a>(game_state: &'a GameState, obj_name: &str) -> Option<&'a dyn Entity> {
    let search_name = obj_name.to_lowercase();

    for item_id in &game_state.inventory {
        if let Some(entity) = game_state.world.entities.get(&EntityId::Item(*item_id)) {
            if entity.name().to_lowercase() == search_name {
                return Some(entity.as_ref());
            }
        }
    }

    None
}