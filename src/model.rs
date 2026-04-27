pub struct Item {
    id: u32,
    name: String,
    completed: bool,
    deleted: bool,
}

impl Item {
    pub fn new(id: u32, name: &str) -> Self {
        Item {
            id,
            name: name.to_string(),
            completed: false,
            deleted: false,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    pub fn is_deleted(&self) -> bool {
        self.deleted
    }
}