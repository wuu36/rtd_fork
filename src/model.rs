
use std::fmt;

#[derive(Clone)]
pub struct Item {
    id: u32,
    name: String,
}

impl  Item {
    pub fn new(
        id: u32,
        name: &str,
    ) -> Self {
        Item { 
            id, 
            name: name.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ParseItemError(String);

impl fmt::Display for ParseItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl std::error::Error for ParseItemError {}