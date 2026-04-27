use crate::{Item, storage};



pub fn add_item(name: &str) -> Result<String, String> {
    let item = Item::new(1, name);
    storage::add_item(&item)?;
    Ok(format!("added [{}]: {}", item.id(), item.name()))
}

pub fn list_uncompleted() -> Result<String, String> {
    // TODO: phase 6
    Ok("no items yet.".to_string())
}