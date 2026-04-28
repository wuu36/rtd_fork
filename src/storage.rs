use crate::model::Item;
use std::io::Read;
use std::io::Write;
use std::ptr::read;
use std::{env, fs::OpenOptions, path::Path};
use std::fs::File;

const CSV_FILE_NAME: &str = ".rtd.csv";

#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    EnvVar(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::Io(e) => write!(f, "IO error: {}", e),
            StorageError::EnvVar(msg) => write!(f, "Enviroment variable error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}
    
impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        StorageError::Io(e)
    }
}

/// get CSV path
fn get_csv_path() -> Result<String, StorageError> {
    let home = env::var("HOME").or_else(|_| env::var("USERPROFILE"))
        .map_err(|_| StorageError::EnvVar("Cannot find home directory".to_string()))?;
    Ok(format!("{}/{}", home, CSV_FILE_NAME))
}

pub fn add_item(item: &Item) -> Result<(), StorageError> {
    let path= get_csv_path()?;
    let path= Path::new(&path);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    // writeln!(file, "{}", format_csv_line(item))?;
    writeln!(file, "{}", item.to_string())?;

    Ok(())
}

pub fn read_all() -> Result<String, StorageError> {
    let path = get_csv_path()?;
    let path = Path::new(&path);

    if !path.exists() {
        return Ok(String::new());
    }

    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

pub fn get_all() -> Result<Vec<Item>, StorageError> {
    let content = read_all()?;
    let items: Vec<Item> = content
        .lines()
        .filter_map(|line| line.parse::<Item>().ok())
        .collect();

    Ok(items)
}

pub fn get_max_id() -> Result<u32, StorageError> {
    let content = read_all()?;

    let max_id: u32 = content
        .lines()
        .filter_map(|line| {
            // 简单解析第一列 (id)
            line.split(',')
                .next()
                .and_then(|s| s.parse::<u32>().ok())
        })
        .max()
        .unwrap_or(0);

    Ok(max_id)
}

fn format_csv_line(item: &Item) -> String {
    format!(
        "{},{},{},{},0,0,0",
        item.id(),
        item.name(),
        item.is_completed(),
        item.is_deleted()
    )
}