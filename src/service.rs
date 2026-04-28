
use crate::model::{timestamp_to_datetime_string};
use crate::{Item, storage::{self, StorageError}};

#[derive(Debug)]
pub enum ServiceError {
    Storage(StorageError),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::Storage(e) => write!(f, "storage error: {}", e),
        }
    }
}

impl std::error::Error for ServiceError {}

impl From<StorageError> for ServiceError {
    fn from(e: StorageError) -> Self {
        ServiceError::Storage(e)
    }
}
    


/// add new todo project
pub fn add_item(name: &str) -> Result<String, ServiceError> {
    let max_id = storage::get_max_id()?;
    let new_id = max_id + 1; 

    let item = Item::new(new_id, name);

    storage::add_item(&item)?;
    Ok(format!("added [{}]: {}", item.id(), item.name()))
}

pub fn list_uncompleted() -> Result<String, ServiceError> {
    // let content = storage::read_all()?;
    let items = storage::get_all()?;

    // 过滤未完成且未删除的项目
    let uncompleted: Vec<&Item> = items
        .iter()
        .filter(|item| !item.is_completed() && !item.is_deleted())
        .collect();

    if uncompleted.is_empty() {
        return Ok("no uncompleted item.".to_string());
    }

    // if content.is_empty() {
    //     return Ok("no items yet.".to_string());
    // }

    let mut output = String::new();
    // output.push_str("Todo List:\n");
    output.push_str("uncompleted Todo:\n");
    
    for item in uncompleted {
        let created = timestamp_to_datetime_string(item.created_at());
        output.push_str(&format!(
            "  [{}] {} (created: {})\n",
            item.id(),
            item.name(),
            created
        ));
    }

    // for line in content.lines() {
    //     if line.is_empty() {
    //         continue;
    //     }

    //     let parts: Vec<&str> = line.split(',').collect();
    //     if parts.len() >= 4 {
    //         let id = parts[0].parse().unwrap_or(0);
    //         let name = parts[1];
    //         let completed = parts[2].parse().unwrap_or(false);
    //         let deleted = parts[3].parse().unwrap_or(false);

    //         if !completed && !deleted {
    //             output.push_str(&format!(" [{}] {}\n", id, name));
    //         }
    //     }
    // }
    
    Ok(output)
}

pub fn list_all() -> Result<String, ServiceError> {
    let items = storage::get_all()?;

    if items.is_empty() {
        return Ok("no items.".to_string());
    }

    let mut output = String::new();
    output.push_str("All Todo:\n");

    for item in items.iter() {
        let status: &str = if item.is_deleted() {
            "🗑️ deleted"
        } else if item.is_completed() {
            "✅ completed"
        } else {
            "🔲 pending"
        };

        output.push_str(&format!("  [{}] {} - {}\n", item.id(), item.name(), status));
    }

    Ok(output)
}