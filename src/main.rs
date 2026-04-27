struct Item {
    id: u32,
    name: String,
    completed: bool,
    deleted: bool,
}

impl Item {
    fn new(id: u32, name: &str) -> Self {
        Item {
            id,
            name: name.to_string(),
            completed: false,
            deleted: false,
        }
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn is_completed(&self) -> bool {
        self.completed
    }

    fn is_deleted(&self) -> bool {
        self.deleted
    }
}



fn main() {
    println!("hellow world!");
    let item1 = Item::new(1, "learn Rust");
    let item2 = Item::new(2, "finish phase 1 & 2");

    println!("RTD Clone - Rust Todo CLI");
    println!();
    println!("Item {}: {}", item1.id(), item1.name());
    println!("  completed: {}", item1.is_completed());
    println!("  deleted: {}", item1.is_deleted());
    println!();
    println!("Item {}: {}", item2.id(), item2.name());
    println!("  completed: {}", item2.is_completed());
    println!("  deleted: {}", item2.is_deleted());
    
}