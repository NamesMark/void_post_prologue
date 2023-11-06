use std::collections::HashMap;
use std::cmp::max;
use rand::prelude::SliceRandom;

use crate::engine::state::GameState;
use crate::world::room::{Direction, Access, PassageType};
use crate::entity::{Entity, EntityId, self};
use crate::entity::item::{ItemId, Containable};


pub fn look(game_state: &GameState) -> String {
    let mut output: String = String::new(); 
    if let Some(room_attributes) = game_state.world.rooms.get(&game_state.current_room) {
        output += &room_attributes.full_description; 
        output.push_str("\nYou can see the following things: ");
        for entity_id in &room_attributes.entities { 
            if let Some(entity) = game_state.world.entities.get(entity_id) {
                output.push_str(&entity.name());
                output.push_str(", ");
            }
        }
        if output.ends_with(", ") {
            output.truncate(output.len() - 2);
        }
        output.push('.');

        output.push_str("\nYou can get to:");
        for (direction, passage_type, _room_id) in &room_attributes.connected_rooms {
            let passage_description = match passage_type {
                PassageType::Door => "a door to the",
                PassageType::Free => "continue to the",
            };
            output.push_str(&format!(" {} {}", passage_description, direction));
            output.push_str(", ");
        }
        if output.ends_with(", ") {
            output.truncate(output.len() - 2);
        }
        output.push('.');
    } else {
        output = "Room not found in the world. Are you in space? Oh crap".into();
    }
    output
}

pub fn look_at(game_state: &GameState, obj_name: &str) -> String {
    let obj_name = obj_name.to_lowercase();

    match find_entity_in_room(game_state, &obj_name) {
        Some(entity) => look_at_helper(game_state, entity),
        None => match find_entity_in_inventory(game_state, &obj_name) {
            Some(entity) => look_at_helper(game_state, entity),
            None => format!("There is no {} here to look at.", obj_name),
        },
    }
}

fn look_at_helper(game_state: &GameState, entity: &dyn Entity) -> String {
    let name = entity.name();
    let description = entity.description();
    let containable = entity.as_containable().is_some();

    let mut output = format!("You look at the {}:\n{}", name, description);
    
    if containable {
        if let Some(container) = entity.as_containable() {
            if !container.contains().is_empty() {
                output.push_str("\nIt contains: ");
                let contents: Vec<String> = container.contains().iter()
                    .filter_map(|id| game_state.world.entities.get(id))
                    .map(|e| e.name().to_string())
                    .collect();
                output.push_str(&contents.join(", "));
            }
        }
    }
    
    output
}

fn get_article(obj_name: &str) -> &str {
    if obj_name.ends_with('s') { "" } else { "the " }
}

pub fn move_in_direction(game_state: &mut GameState, direction: Direction) -> Result<String, String> {
    match game_state.world.get_adjacent_room(&game_state.current_room, direction) {
        Some(new_room) => {
            if (new_room.1) == &PassageType::Door {
                if get_player_access(game_state) < *game_state.world.get_room_access(&(new_room.0)) {
                    return Err(format!("The door beeps with an unsatisfied tone."));
                }
                print_any!(
                    "The door beeps with quiet acknowledgement and slides aside.",
                    "*Shhhhht* - the door slides open.",
                    "The door opens with no apparent effort from your side.",
                    "The door opened so fast as if it predicted your intention."
                );
            }
            game_state.current_room = new_room.0;

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

pub fn get_player_access(game_state: &GameState) -> Access {
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

pub fn drop(game_state: &mut GameState, obj_name: &str) -> String {
    let obj_name = obj_name.to_lowercase();
    let article = get_article(&obj_name);

    if let Some(entity_ref) = find_entity_in_inventory(game_state, &obj_name) {
        let entity_id = entity_ref.get_id();

        if let EntityId::Item(item_id) = entity_id {
            // Remove from the inventory
            game_state.inventory.retain(|&e| e != item_id);

            // Add to the room
            if let Some(room) = game_state.world.rooms.get_mut(&game_state.current_room) {
                room.entities.push(entity_id);
            }

            if let Some(entity) = game_state.world.entities.get(&entity_id) {
                return format!("You've dropped {}{}.", article, obj_name);
            } else {
                return "There seems to be a problem dropping the item.".to_string();
            }
        } else {
            return format!("You can't drop the {}.", obj_name);
        }
    } else {
        format!("You don't have a {} to drop.", obj_name)
    }
}

pub fn put_into(game_state: &mut GameState, obj_name: &str, cont_name: &str) -> String {
    let obj_name = obj_name.to_lowercase();
    let article = get_article(&obj_name);

    if let Some(obj_entity_ref) = find_entity_in_inventory(game_state, &obj_name) {
        let obj_id = obj_entity_ref.get_id();

        if let Some(cont_entity_id) = find_containable_entity_in_room(game_state, &cont_name) {
            if cont_entity_id == obj_id {
                return format!("I cant put things into themselves, it would cause some kind of a paradox, I'm pretty sure. Good try, though!");
            }
            if let Some(container) = game_state.world.get_container_mut(cont_entity_id) {
                if container.can_contain(obj_id) {
                    game_state.inventory.retain(|&e| EntityId::Item(e) != obj_id);

                    if let Err(e) = container.put(obj_id) {
                        return e;
                    }

                    return format!("You've put {}{} into {}{}.", article, obj_name, get_article(cont_name), cont_name);
                } else {
                    return format!("{} cannot contain {}.", cont_name, obj_name);
                }
            } else {
                return format!("{} is not a container.", cont_name);
            }
        } else {
            if let Some(cont_entity_id) = find_containable_entity_in_inventory(game_state, &cont_name) {
                if let Some(container) = game_state.world.get_container_mut(cont_entity_id) {
                    if container.can_contain(obj_id) {
                        game_state.inventory.retain(|&e| EntityId::Item(e) != obj_id);
    
                        if let Err(e) = container.put(obj_id) {
                            return e;
                        }
    
                        return format!("You've put {}{} into {}{}.", article, obj_name, get_article(cont_name), cont_name);
                    } else {
                        return format!("{} cannot contain {}.", cont_name, obj_name);
                    }
                } else {
                    return format!("{} is not a container.", cont_name);
                }
            } else {
                return format!("You can't find {} to put things into.", cont_name);
            }
        }
    } else {
        return format!("You don't have {}.", obj_name);
    }
}

pub fn take_from_container(game_state: &mut GameState, item_name: &str, container_name: &str) -> String {
    let item_name = item_name.to_lowercase();
    let container_name = container_name.to_lowercase();

    if let Some(container_id) = find_containable_entity_in_room(game_state, &container_name) {
        let entity_names_and_ids: Vec<(EntityId, String)> = {
            if let Some(container) = game_state.world.get_container(container_id) {
                container.contains()
                    .iter()
                    .filter_map(|&id| {
                        if let Some(entity) = game_state.world.entities.get(&id) {
                            let name = entity.name().to_lowercase();
                            let aliases = entity.aliases().iter().map(|a| a.to_lowercase()).collect::<Vec<String>>();
                            if name == item_name || aliases.contains(&item_name) {
                                return Some((id, name));
                            }
                        }
                        None
                    })
                    .collect()
            } else {
                return format!("{} is not a container.", container_name);
            }
        };

        if let Some(&(entity_id, _)) = entity_names_and_ids.first() {
            if let Some(container) = game_state.world.get_container_mut(container_id) {
                match container.remove(entity_id) {
                    Ok(_) => {
                        if let EntityId::Item(item_id) = entity_id {
                            game_state.inventory.push(item_id);
                            format!("You take the {} from the {}.", item_name, container_name)
                        } else {
                            if let Some(entities) = game_state.world.get_room_entities_mut(&game_state.current_room) {
                                entities.push(entity_id);
                                format!("You take the {} from the {}, but you can't place it into the inventory. It's on the floor now.", item_name, container_name)
                            } else {
                                format!("Could not find the current room to return the entity.")
                            }
                        }
                    }
                    Err(e) => e.to_string(),
                }
            } else {
                format!("{} is not a container.", container_name)
            }
        } else {
            format!("The {} is not in the {}.", item_name, container_name)
        }
    } else {
        format!("There is no {} here to take things from.", container_name)
    }
}

fn find_entity_in_room<'a>(game_state: &'a GameState, obj_name: &str) -> Option<&'a dyn Entity> {
    let search_name = obj_name.to_lowercase();

    if let Some(room_entity_ids) = game_state.current_room_entities() {
        for entity_id in room_entity_ids {
            if let Some(entity) = game_state.world.entities.get(entity_id) {
                eprintln!("DEBUG: Comparing: '{}' with '{}'", entity.name().to_lowercase(), search_name);
                if entity.name().to_lowercase() == search_name {
                    return Some(entity.as_ref());
                }
                for alt_name in entity.aliases() {
                    eprintln!("DEBUG: Comparing: '{}' with '{}'", alt_name, search_name);
                    if *alt_name == search_name {
                        return Some(entity.as_ref());
                    }
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
            for alt_name in entity.aliases() {
                if *alt_name == search_name {
                    return Some(entity.as_ref());
                }
            }
        }
    }

    None
}

fn find_containable_entity_in_room(game_state: &GameState, cont_name: &str) -> Option<EntityId> {
    let search_name = cont_name.to_lowercase();

    if let Some(room_entity_ids) = game_state.current_room_entities() {
        for entity_id in room_entity_ids {
            if let Some(entity) = game_state.world.entities.get(entity_id) {
                if entity.name().to_lowercase() == search_name {
                    if entity.as_containable().is_some() {
                        return Some(*entity_id);
                    }
                }
            }
        }
    }
    None
}

fn find_containable_entity_in_inventory(game_state: &GameState, cont_name: &str) -> Option<EntityId> {
    let search_name = cont_name.to_lowercase();

    for item_id in &game_state.inventory {
        let entity_id = EntityId::Item(*item_id);
        if let Some(entity) = game_state.world.entities.get(&EntityId::Item(*item_id)) {
                if entity.name().to_lowercase() == search_name {
                    if entity.as_containable().is_some() {
                        return Some(entity_id);
                    }
                }
            }
        }
    None
}