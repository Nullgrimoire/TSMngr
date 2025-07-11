//! Ticket data model and constructor.
//!
//! This module provides the [`Ticket`] struct which represents a support or task ticket in the system.
//! It also provides functionality to create a new ticket with a default status of "Open".

use serde::{Serialize, Deserialize};
use uuid::Uuid;

/// Represents a support or task ticket in the system.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    /// Unique identifier for the ticket.
    pub id: String,
    /// Title of the ticket.
    pub title: String,
    /// Detailed description of the ticket.
    pub description: String,
    /// Current status of the ticket. Defaults to "Open" when a ticket is created.
    pub status: String, // 🆕
}

impl Ticket {
    /// Create a new ticket with the given title and description.
    ///
    /// # Arguments
    ///
    /// * `title` - A string slice that holds the title of the ticket.
    /// * `description` - A string slice that holds the description of the ticket.
    ///
    /// # Examples
    ///
    /// ```
    /// let ticket = Ticket::new("Ticket Title", "Ticket Description");
    /// ```
    pub fn new(title: &str, description: &str) -> Self {
        Ticket {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: description.to_string(),
            status: "Open".to_string(), // 🆕 Default status
        }
    }
}
