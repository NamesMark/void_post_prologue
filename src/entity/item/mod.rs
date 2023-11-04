use crate::impl_entity;
use strum_macros::{EnumIter, Display};
use super::EntityId;

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
    
    #[default]
    Nothing
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

impl_entity!(Item, Container, Food, Drink, TextItem, SecretBottle);

pub struct Item {
    id: EntityId,
    name: String,
    description: String,
    size: Size,
    // weight: f32,
    // weight_distribution: f32,
    // long: bool
}

impl Item {
    pub fn new(id: EntityId, name: String, description: String, size: Size) -> Self {
        Item { id, name, description, size }
    }
}

pub struct Container {
    id: EntityId,
    name: String,
    description: String,
    contains: Vec<EntityId>,
    size: Size,
}

impl Container {
    pub fn new(id: EntityId, name: String, description: String, contains: Vec<EntityId>, size: Size) -> Self {
        Container { id, name, description, contains, size }
    }
}

impl Containable for Container {
    fn put(&mut self, item: ItemId) -> Result<(), &'static str> {
        todo!()
    }
    fn remove(&mut self, item: ItemId) -> Option<Item> {
        todo!()
    }
}

pub struct SecretBottle {
    id: EntityId,
    name: String,
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


pub struct TextItem {
    id: EntityId,
    name: String,
    description: String,
    contents: String,
}

impl TextItem {
    pub fn new(id: EntityId, name: String, description: String, contents: String) -> Self {
        TextItem { id, name, description, contents}
    }
}

impl Readable for TextItem {
    fn read(&mut self) -> Result<(), &'static str> {
        println!("You read the {}:", self.name);
        println!("{}", self.contents);

        Ok(())
    }
}

pub struct Food {
    id: EntityId,
    name: String,
    description: String,
}

impl Food {
    pub fn new(id: EntityId, name: String, description: String) -> Self {
        Food { id, name, description}
    }
}

impl Edible for Food {
    fn eat(&mut self) -> Result<(), &'static str> {
        println!("You eat the {}.", self.name);

        Ok(())
    }
}

#[derive(Debug)]
pub struct Drink {
    id: EntityId,
    name: String,
    description: String,
}

impl Drink {
    pub fn new(id: EntityId, name: String, description: String) -> Self {
        Drink { id, name, description}
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
    description: String,
    liquid: Liquid,
    //amount: u8,
}

impl LiquidContainer {
    pub fn new(id: EntityId, name: String, description: String, liquid: Liquid) -> Self {
        LiquidContainer { id, name, description, liquid}
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
    fn put(&mut self, item: ItemId) -> Result<(), &'static str>;
    fn remove(&mut self, item: ItemId) -> Option<Item>;
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

// pub struct Chest {
//     base: Item,
//     is_open: bool,
//     contents: Vec<Vec<Item>>, // each sub-vector represents items of a certain size.
//     max_size: u8,
//     pub fn new(max_size: u8) -> Self {
//         let mut contents = Vec::new();
//         for _ in 0..max_size {
//             contents.push(Vec::new());
//         }
//         Chest {
//             base: Item { id: 0, size: max_size }, 
//             is_open: false,
//             contents,
//             max_size,
//         }
//     }
// }

// impl Container for Chest {
//     fn put(&mut self, item: Item) -> Result<(), &'static str> {
//         if item.size > self.max_size {
//             return Err("The item is too large for this chest.");
//         }

//         let available_space = 10u32.pow((self.max_size - item.size) as u32);
//         if self.contents[(item.size - 1) as usize].len() >= available_space as usize {
//             return Err("There's no space for this item.");
//         }

//         self.contents[(item.size - 1) as usize].push(item);
//         Ok(())
//     }

//     fn remove(&mut self, item_id: u32) -> Option<Item> {
//         for items_of_size in &mut self.contents {
//             if let Some(index) = items_of_size.iter().position(|item| item.id == item_id) {
//                 return Some(items_of_size.remove(index));
//             }
//         }
//         None
//     }
// }

// impl Openable for Chest {
//     fn open(&mut self) -> Result<(), &'static str> {
//         if (self.is_open) {
//             return Err(any_of!(
//                 "It is already open.",
//                 "Dude, it's as open as I could open it!",
//                 "Again? No."
//             ));
//         } else {
//             self.is_open = true;
//             return Ok(());
//         }
//     }
//     fn close(&mut self) -> Result<(), &'static str> {
//         if (!self.is_open) {
//             return Err(any_of!(
//                 "It is already closed.",
//                 "I really-really closed it!",
//                 "It already is closed!"
//             ));
//         } else {
//             self.is_open = true;
//             return Ok(());
//         }
//     }
// }