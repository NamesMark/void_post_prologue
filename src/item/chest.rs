impl Chest {
    pub fn new(max_size: u8) -> Self {
        let mut contents = Vec::new();
        for _ in 0..max_size {
            contents.push(Vec::new());
        }
        Chest {
            base: Item { id: 0, size: max_size }, 
            is_open: false,
            contents,
            max_size,
        }
    }
}

impl Container for Chest {
    fn put(&mut self, item: Item) -> Result<(), &'static str> {
        if item.size > self.max_size {
            return Err("The item is too large for this chest.");
        }

        let available_space = 10u32.pow((self.max_size - item.size) as u32);
        if self.contents[(item.size - 1) as usize].len() >= available_space as usize {
            return Err("There's no space for this item.");
        }

        self.contents[(item.size - 1) as usize].push(item);
        Ok(())
    }

    fn remove(&mut self, item_id: u32) -> Option<Item> {
        for items_of_size in &mut self.contents {
            if let Some(index) = items_of_size.iter().position(|item| item.id == item_id) {
                return Some(items_of_size.remove(index));
            }
        }
        None
    }
}

impl Openable for Chest {
    fn open(&mut self) -> Result<(), &'static str> {
        if (self.is_open) {
            return Err(any_of!(
                "It is already open.",
                "Dude, it's as open as I could open it!",
                "Again? No."
            ));
        } else {
            self.is_open = true;
            return Ok(());
        }
    }
    fn close(&mut self) -> Result<(), &'static str> {
        if (!self.is_open) {
            return Err(any_of!(
                "It is already closed.",
                "I really-really closed it!",
                "It already is closed!"
            ));
        } else {
            self.is_open = false;
            return Ok(());
        }
    }
}