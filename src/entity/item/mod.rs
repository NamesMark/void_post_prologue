pub mod container;
pub mod food;
pub mod text_item;

use crate::{impl_entity, impl_entity_containable};
use std::any::Any;
use strum_macros::{EnumIter, Display};
use super::{Entity, EntityId};

#[derive(EnumIter, Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ItemId {
    Bucket,
    SpaceRation,
    WaterBottle,
    EmptyBottle,
    SecretBottle,
    CounterNote,
    Biscuits,
    Plate,
    FoodSurrogateBottle,
    Fork,
    LuckyCoin,
    
    
    AssistantCard,
    BosunCard,
    CaptainCard,

    SpaceSuit,

    #[default]
    Dust
}

pub enum Size {
    Small,
    Medium,
    Large,
}

#[derive(Display, PartialEq, Eq)]
pub enum Liquid {
    Water,
    Coffee,
    Fuel,
    Air,
}

impl_entity!(Item, Drink, SecretBottle);

pub struct Item {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    size: Size,
    // weight: f32,
    // weight_distribution: f32,
    // long: bool
}

impl Item {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String, size: Size) -> Self {
        Item { id, name, aliases, description, size }
    }
}



pub struct SecretBottle {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    contains: Option<EntityId>,
    liquid: Liquid,
}

impl SecretBottle {
    fn put(&mut self, item: ItemId) -> Result<(), &'static str> {
        todo!()
    }

    fn remove(&mut self) -> Result<(), String> {
        if self.liquid == Liquid::Air {
            Err("Can't get to it!".to_string())
        } else {
            if self.contains.is_some() {
                println!("You manage to get the {} out of {}.", self.liquid, self.name);
                self.liquid = Liquid::Air;
                //self.contains.take();
                Ok(())
            } else {
                Err("There's nothing there!".to_string())
            }
        }
    }
}

impl LiquidContainable for SecretBottle {
    fn fill(&mut self, liquid: Liquid) -> Result<(), String> {
        if self.liquid == Liquid::Air {
            self.liquid = liquid;
            Ok(())
        } else {
            Err(format!("The {} is already full", self.name))
        }
    }

    fn pour(&mut self) -> Result<(), String> {
        if self.liquid == Liquid::Air {
            Err(format!("The {} is already empty", self.name))
        } else {
            self.liquid = Liquid::Air;
            Ok(())
        }
    }

    fn drink(&mut self) -> Result<(), String> {
        if self.liquid == Liquid::Air {
            Err(format!("The {} is empty", self.name))
        } else {
            println!("You drink the {} out of {}.", self.liquid, self.name);
            self.liquid = Liquid::Air;
            Ok(())
        }
    }
}



#[derive(Debug)]
pub struct Drink {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
}

impl Drink {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String) -> Self {
        Drink { id, name, aliases, description}
    }
}

impl Drinkable for Drink {
    fn drink(&mut self) -> Result<(), &'static str> {
        println!("You drink the {}.", self.name);

        Ok(())
    }
}

pub struct LiquidContainer {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    liquid: Liquid,
    //amount: u8,
}

impl LiquidContainer {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String, liquid: Liquid) -> Self {
        LiquidContainer { id, name, aliases, description, liquid}
    }
}

impl LiquidContainable for LiquidContainer {
    fn fill(&mut self, liquid: Liquid) -> Result<(), String> {
        if self.liquid == Liquid::Air {
            self.liquid = liquid;
            Ok(())
        } else {
            Err(format!("The {} is already full", self.name))
        }
    }

    fn pour(&mut self) -> Result<(), String> {
        if self.liquid == Liquid::Air {
            Err(format!("The {} is already empty", self.name))
        } else {
            self.liquid = Liquid::Air;
            Ok(())
        }
    }

    fn drink(&mut self) -> Result<(), String> {
        if self.liquid == Liquid::Air {
            Err(format!("The {} is empty", self.name))
        } else {
            println!("You drink the {} out of {}.", self.liquid, self.name);
            self.liquid = Liquid::Air;
            Ok(())
        }
    }
}



pub trait Containable {
    fn can_contain(&self, entity_id: EntityId) -> bool;
    fn put(&mut self, entity_id: EntityId) -> Result<(), String>;
    fn remove(&mut self, entity_id: EntityId) -> Result<(), String>;
    fn contains(&self) -> &Vec<EntityId>;
}

pub trait LiquidContainable {
    fn fill(&mut self, liquid: Liquid) -> Result<(), String>;
    fn drink(&mut self) -> Result<(), String>;
    fn pour(&mut self) -> Result<(), String>;
}


pub trait Openable {
    fn open(&mut self) -> Result<(), &'static str>;
    fn close(&mut self) -> Result<(), &'static str>;
}

pub trait Flammable {
    fn ignite(&mut self) -> Result<(), &'static str>;
    fn extinguish(&mut self) -> Result<(), &'static str>;
}

pub trait Edible {
    fn eat(&mut self) -> Result<(), &'static str>;
}
pub trait Drinkable {
    fn drink(&mut self) -> Result<(), &'static str>;
}
pub trait Readable {
    fn read(&mut self) -> Result<(), &'static str>;
}