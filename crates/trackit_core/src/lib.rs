use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub cycles: u8,
    pub started_at: Option<()>, // Option<DateTime>
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
