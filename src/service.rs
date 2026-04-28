
use crate::model::timestamp_to_datetime_string;
use crate::{Item, storage::{self, StorageError}};

type Result<T> = std::result::Result<T, ServiceError>;

#[derive(Debug)]
pub enum ServiceError {
    Storage(StorageError),
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::Storage(e) => write!(f, "Service error: {}", e),
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
pub fn add_item(name: &str) -> Result<String> {
    let max_id = storage::get_max_id()?;
    let new_id = max_id + 1; 

    let item = Item::new(new_id, name);

    storage::add_item(&item)?;
    Ok(format!("added [{}]: {}", item.id(), item.name()))
}

/// complete item
pub fn complete_item(id: u32) -> Result<String> {
    let item = storage::get_item_by_id(id)?;
    let updated = item.with_completed(true);
    storage::update_item(updated)?;

    Ok(format!("complete [{}]: {}", id, item.name()))
}

/// cancel finished Todo
pub fn uncomplete_item(id: u32) -> Result<String> {
    let item = storage::get_item_by_id(id)?;
    let updated = item.with_completed(false);
    storage::update_item(updated)?;

    Ok(format!("uncompleted [{}]: {}", id, item.name()))
}

/// list uncompleted items
pub fn list_uncompleted() -> Result<String> {
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

    let mut result = "Uncompleted Todo:\n\n".to_string();
    for item in uncompleted {
        result += &format_item(&item);
        result += "\n";
    }
    Ok(result)
}

/// delete todo
pub fn delete_item(id: u32) -> Result<String> {
    let item = storage::get_item_by_id(id)?;
    let updated = item.with_deleted(true);
    storage::update_item(updated)?;

    Ok(format!("deleted [{}]: {}", id, item.name()))
}

/// restore todo form trash
pub fn restore_item(id: u32) -> Result<String> {
    let item = storage::get_item_by_id(id)?;
    let updated = item.with_deleted(false);
    storage::update_item(updated)?;

    Ok(format!("restore [{}]: {}", id, item.name()))
}

/// list completed items
pub fn list_completed() -> Result<String> {
    let items = storage::get_all()?;

    let completed: Vec<Item> = items
        .into_iter()
        .filter(|item| !item.is_deleted() && item.is_completed())
        .collect();

    if completed.is_empty() {
        return  Ok("no completed items.".to_string());
    }

    let mut result = "completed Todo:\n\n".to_string();
    for item in completed {
        result += &format_item(&item);
        result += "\n";
    }
    Ok(result)
}

/// list deleted items
pub fn list_deleted() -> Result<String> {
    let items = storage::get_all()?;

    let deleted: Vec<Item> = items
        .into_iter()
        .filter(|item| item.is_deleted())
        .collect();

    if deleted.is_empty() {
        return Ok("trash is empty.".to_string());
    }

    let mut result = "deleted Todo (Trash):\n\n".to_string();
    for item in deleted {
        result += &format_item(&item);
        result += "\n";
    }
    Ok(result)
}

pub fn destory_item(id: u32) -> Result<String> {
    storage::delete_item(id)?;
    Ok(format!("destory [{}]", id))
}

pub fn destory_deleted() -> Result<String> {
    let items = storage::get_all()?;
    let deleted_ids: Vec<u32> = items
        .iter()
        .filter(|item| item.is_deleted())
        .map(|item| item.id())
        .collect();

    if deleted_ids.is_empty() {
        return Ok("trash is empty, nothing to destory.".to_string());
    }

    let count = deleted_ids.len();
    for id in &deleted_ids {
        storage::delete_item(*id)?;
    }

    Ok(format!("destoryed {} items from trash.", count))
}

pub fn clear() -> Result<String> {
    let items = storage::get_all()?;

    if items.is_empty() {
        return Ok("no items to clear.".to_string());
    }

    let count = items.len();
    for item in items {
        storage::delete_item(item.id())?;
    }
    
    Ok(format!("cleared {} items.", count))
}

/// list all Todo
pub fn list_all() -> Result<String> {
    let items = storage::get_all()?;

    if items.is_empty() {
        return Ok("no items.".to_string());
    }

    let mut result = "all Todo:\n\n".to_string();
    for item in items {
        result += &format_item(&item);
        result += "\n";
    }
    Ok(result)
}

/// format single Item
fn format_item(item: &Item) -> String {
    // 状态图标
    let status_icon = if item.is_deleted() {
        "🗑️"
    } else if item.is_completed() {
        "✅"
    } else {
        "🔲"
    };

    // 时间信息
    let created = timestamp_to_datetime_string(item.created_at());
    let completed = timestamp_to_datetime_string(item.completed_at());
    let deleted = timestamp_to_datetime_string(item.deleted_at());
    
    let mut result = format!("{:3} {} {}\n", item.id(), status_icon, item.name());

    if !created.is_empty() {
        result += &format!("    Created: {}\n", created);
    }
    if !completed.is_empty() {
        result += &format!("    Completed: {}\n", completed);
    }
    if !deleted.is_empty() {
        result += &format!("    Deleted: {}\n", deleted);
    }

    result
}