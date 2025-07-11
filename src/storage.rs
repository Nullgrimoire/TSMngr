use rusqlite::{params, Connection};
use crate::ticket::Ticket;
use std::error::Error;

const DB_PATH: &str = "tickets.db";

/// Returns a connection to the SQLite database. This helper centralizes
/// creation of the `rusqlite` connection so callers don't have to repeat the
/// open logic everywhere.
pub fn get_connection() -> Result<Connection, Box<dyn Error>> {
    Connection::open(DB_PATH).map_err(|e| e.into())
}

pub fn init_db() -> Result<(), Box<dyn Error>> {
    let conn = get_connection()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tickets (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ticket_id TEXT NOT NULL,
            action TEXT NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn save_tickets(tickets: &Vec<Ticket>) -> Result<(), Box<dyn Error>> {
    let mut conn = get_connection()?;
    let tx = conn.transaction()?;
    tx.execute("DELETE FROM tickets", [])?;
    {
        let mut stmt = tx.prepare("INSERT INTO tickets (id, title, description, status) VALUES (?1, ?2, ?3, ?4)")?;
        for t in tickets {
            stmt.execute(params![t.id, t.title, t.description, t.status])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn load_tickets() -> Result<Vec<Ticket>, Box<dyn Error>> {
    init_db()?;
    let conn = get_connection()?;
    let mut stmt = conn.prepare("SELECT id, title, description, status FROM tickets")?;
    let ticket_iter = stmt.query_map([], |row| {
        Ok(Ticket {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            status: row.get(3)?,
        })
    })?;
    let tickets = ticket_iter.filter_map(|t| t.ok()).collect();
    Ok(tickets)
}

pub fn seed_sample_data() -> Result<(), Box<dyn Error>> {
    let conn = get_connection()?;
    // Insert sample users
    conn.execute(
        "INSERT OR IGNORE INTO users (id, name, email) VALUES (?1, ?2, ?3)",
        params!["user1", "Alice Example", "alice@example.com"],
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO users (id, name, email) VALUES (?1, ?2, ?3)",
        params!["user2", "Bob Example", "bob@example.com"],
    )?;
    // Insert sample tickets
    conn.execute(
        "INSERT OR IGNORE INTO tickets (id, title, description, status) VALUES (?1, ?2, ?3, ?4)",
        params!["ticket1", "Sample Ticket 1", "This is a sample ticket.", "Open"],
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO tickets (id, title, description, status) VALUES (?1, ?2, ?3, ?4)",
        params!["ticket2", "Sample Ticket 2", "Another sample ticket.", "In Progress"],
    )?;
    Ok(())
}