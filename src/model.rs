use chrono::{Local, TimeZone};
use std::str::FromStr;
use std::error::Error;
use std::fmt::Display;
use std::num::ParseIntError;
use std::str::ParseBoolError;

pub struct Item {
    id: u32,
    name: String,
    completed: bool,
    deleted: bool,
    created_at: Option<i64>,
    completed_at: Option<i64>,
    deleted_at: Option<i64>,
}

/// CSV 字段数量
const ITEM_COUNT: usize = 7;
/// 用于转义 CSV 中的逗号
const COMMA_FAKE: &str = "<@^_fake_comma_$#>";
/// 用于转义 CSV 中的换行
const NEWLINE_FAKE: &str = "<@^_fake_newline_$#>";

impl Item {
    pub fn new(id: u32, name: &str) -> Self {
        let now = Local::now().timestamp();
        Item {
            id,
            name: name.to_string(),
            completed: false,
            deleted: false,
            created_at: Some(now),
            completed_at: None,
            deleted_at: None,
        }
    }

    /// 创建完整的 Item (用于反序列化)
    pub fn new_full(
        id: u32,
        name: &str,
        completed: bool,
        deleted: bool,
        created_at: Option<i64>,
        completed_at: Option<i64>,
        deleted_at: Option<i64>,
    ) -> Self {
        Item {
            id,
            name: name.to_string(),
            completed,
            deleted,
            created_at,
            completed_at,
            deleted_at,
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

    pub fn created_at(&self) -> Option<i64> {
        self.created_at
    }

    pub fn completed_at(&self) -> Option<i64> {
        self.completed_at
    }

    pub fn deleted_at(&self) -> Option<i64> {
        self.deleted_at
    }
}

/// Serialization
impl ToString for Item {
    fn to_string(&self) -> String {
        let name = self
            .name
            .replace(',', COMMA_FAKE)
            .replace(r"\n", NEWLINE_FAKE);

        let created_at = timestamp_to_raw_string(self.created_at);
        let completed_at = timestamp_to_raw_string(self.completed_at);
        let deleted_at = timestamp_to_raw_string(self.deleted_at);

        format!(
            "{},{},{},{},{},{},{}",
            self.id, name, self.completed, self.deleted, created_at, completed_at, deleted_at
        )
    }
}

impl FromStr for Item {
    type Err = ParseItemError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();

        // 检查字段数量
        if parts.len() != ITEM_COUNT {
            return Err(ParseItemError(format!(
                "Expected {} fields, got {}",
                ITEM_COUNT,
                parts.len()
            )));
        }

        // 解析各字段
        let id = parts[0].parse::<u32>()?;

        // 还原转义字符
        let name = parts[1]
            .replace(COMMA_FAKE, ",")
            .replace(NEWLINE_FAKE, "\n");

        let completed = parts[2].parse::<bool>()?;
        let deleted = parts[3].parse::<bool>()?;

        let created_at = str_to_timestamp(parts[4])?;
        let completed_at = str_to_timestamp(parts[5])?;
        let deleted_at = str_to_timestamp(parts[6])?;

        Ok(Item::new_full(
            id,
            &name,
            completed,
            deleted,
            created_at,
            completed_at,
            deleted_at,
        ))
    }
}

/// 
#[derive(Debug)]
pub struct ParseItemError(String);

impl Display for ParseItemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse item error: {}", self.0)
    }
}

impl Error for ParseItemError {}

impl From<ParseIntError> for ParseItemError {
    fn from(e: ParseIntError) -> Self {
        ParseItemError(e.to_string())
    }
}

impl From<ParseBoolError> for ParseItemError {
    fn from(e: ParseBoolError) -> Self {
        ParseItemError(e.to_string())
    }
}

/// 字符串转时间戳 (用于反序列化)
fn str_to_timestamp(s: &str) -> std::result::Result<Option<i64>, ParseItemError> {
    if s.is_empty() {
        Ok(None)
    } else {
        Ok(Some(s.parse::<i64>()?))
    }
}

fn timestamp_to_raw_string(timestamp: Option<i64>) -> String {
    match timestamp {
        Some(t) => t.to_string(),
        None => String::new(),
    }
}

/// 时间戳格式化为可读日期 (用于显示)
pub fn timestamp_to_datetime_string(timestamp: Option<i64>) -> String {
    match timestamp {
        Some(t) => {
            // 使用 chrono 0.4 的新 API
            Local.timestamp_opt(t, 0)
                .single()
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_default()
        }
        None => String::new(),
    }
}