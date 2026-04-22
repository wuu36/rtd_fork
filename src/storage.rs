use std::fmt::Display;
use crate::model::Item;



type Result<T> = std::result::Result<T, StorageError>;

pub enum StorageError {
    FileHandle(u32),
    ParseItem(u32),
    ItemNoExist(u32),
}

impl Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
            StorageError::FileHandle(id) => write!(f, "File handle error for id: {}", id),
            StorageError::ParseItem(id) => write!(f, "Parse item error for id: {}", id),
            StorageError::ItemNoExist(id) => write!(f, "Item not exist: {}", id),
        }
    }
}



pub fn get_max_id() -> Result<u32> {
    Ok(0)
}

pub fn add_item(item: Item) -> Result<()> {
    Ok(())
}