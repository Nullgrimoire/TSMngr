use rusqlite::{params, Connection};
use crate::ticket::Ticket;

const DB_PATH: &str = "tickets.db";

/// Returns a connection to the SQLite database. This helper centralizes
/// creation of the `rusqlite` connection so callers don't have to repeat the
/// open logic everywhere.
pub fn get_connection() -> Connection {
    Connection::open(DB_PATH).expect("Failed to open DB")
}

pub fn init_db() {
    let conn = get_connection();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create users table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tickets (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create tickets table");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            ticket_id TEXT NOT NULL,
            action TEXT NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create logs table");
}

pub fn save_tickets(tickets: &Vec<Ticket>) {
    let mut conn = get_connection();
    let tx = conn.transaction().expect("Failed to begin transaction");
    tx.execute("DELETE FROM tickets", []).expect("Failed to clear table");
    {
        let mut stmt = tx
            .prepare("INSERT INTO tickets (id, title, description, status) VALUES (?1, ?2, ?3, ?4)")
            .expect("Failed to prepare statement");
        for t in tickets {
            stmt.execute(params![t.id, t.title, t.description, t.status])
                .expect("Failed to insert ticket");
        }
    }
    tx.commit().expect("Failed to commit transaction");
}

pub fn load_tickets() -> Vec<Ticket> {
    init_db();
    let conn = get_connection();
    let mut stmt = conn
        .prepare("SELECT id, title, description, status FROM tickets")
        .expect("Failed to prepare query");
    let ticket_iter = stmt
        .query_map([], |row| {
            Ok(Ticket {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                status: row.get(3)?,
            })
        })
        .expect("Failed to map query");

    ticket_iter.map(|t| t.unwrap()).collect()
}