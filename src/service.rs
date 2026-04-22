use crate::{
    model::Item, 
    storage::{self, StorageError, get_max_id}
};

use std::fmt::{Display, Formatter};

type Result<T> = std::result::Result<T, ServiceError>;

pub enum ServiceError {
    Storage(StorageError)
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ServiceError::*;
        match self {
            Storage(e) => writeln!(f, "Rtd service storage error: {}", e),
        }
    }
}

impl From<StorageError> for ServiceError {
    fn from(value: StorageError) -> Self {
        Self::Storage(value)
    }
}

pub fn add_item(name: &str) -> Result<String> {
    let max_id = get_max_id()?;
    let item = Item::new(
        max_id + 1,
        name,
    );
    storage::add_item(item.clone())?;
    Ok(format!("Added item: {name}"))
}

pub fn complete_item(id: u32) -> Result<String> {
    Ok(format!("Completed [{}]: \n", id))
}