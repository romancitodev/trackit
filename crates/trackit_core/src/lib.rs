/// `[chrono]` Re-exports
pub mod chrono {
    pub use chrono::*;
}

use std::time::Duration;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Task {
    pub name: String,
    pub cycles: u8,
    pub started_at: Option<DateTime<Local>>,
    pub elapsed: Duration,
}

impl Task {
    pub fn new(name: String, cycles: u8) -> Task {
        Self {
            name,
            cycles,
            started_at: None,
            elapsed: Duration::ZERO,
        }
    }
}
