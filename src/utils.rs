//! Utility functions shared between main.rs and cli.rs

use crate::ticket::Ticket;
use std::io::{self, Write};
use std::fs::File;

/// Prompt the user for input with a message and return the trimmed response.
pub fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

/// Export the given tickets to a Markdown file (tickets.md).
pub fn export_to_markdown(tickets: &[Ticket]) {
    let mut file = File::create("tickets.md").expect("Failed to create file");
    writeln!(file, "# 🎟️ Ticket List\n").unwrap();
    for (i, ticket) in tickets.iter().enumerate() {
        writeln!(file, "## {}. {}\n", i + 1, ticket.title).unwrap();
        writeln!(file, "- **ID**: `{}`", ticket.id).unwrap();
        writeln!(file, "- **Status**: {}", ticket.status).unwrap();
        writeln!(file, "- **Description**: {}", ticket.description).unwrap();
        writeln!(file).unwrap(); // blank line
    }
    if tickets.is_empty() {
        writeln!(file, "_No tickets found._").unwrap();
    }
}
