pub mod chrono {
    pub use chrono::*;
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Task {
    pub name: String,
    pub cycles: u8,
    pub started_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(name: String, cycles: u8) -> Task {
        Self {
            name,
            cycles,
            started_at: None,
        }
    }
}
