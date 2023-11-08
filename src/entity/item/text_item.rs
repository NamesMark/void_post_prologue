use super::{Entity, EntityId, Item, ItemId};
use super::Size;
use super::Readable;
use crate::{impl_entity};

impl_entity!(TextItem);

pub struct TextItem {
    id: EntityId,
    name: String,
    aliases: Vec<String>,
    description: String,
    contents: String,
}

impl TextItem {
    pub fn new(id: EntityId, name: String, aliases: Vec<String>, description: String, contents: String) -> Self {
        TextItem { id, name, aliases, description, contents}
    }
}

impl Readable for TextItem {
    fn read(&mut self) -> Result<(), &'static str> {
        println!("You read the {}:", self.name);
        println!("{}", self.contents);

        Ok(())
    }
}