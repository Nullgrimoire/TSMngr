use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: String, // 🆕
}

impl Ticket {
    pub fn new(title: &str, description: &str) -> Self {
        Ticket {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: description.to_string(),
            status: "Open".to_string(), // 🆕 Default status
        }
    }
}
