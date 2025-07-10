use rusqlite::{params, Connection};
use crate::ticket::Ticket;

const DB_PATH: &str = "tickets.db";

pub fn init_db() {
    let conn = Connection::open(DB_PATH).expect("Failed to open DB");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tickets (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    ).expect("Failed to create table");
}

pub fn save_tickets(tickets: &Vec<Ticket>) {
    let mut conn = Connection::open(DB_PATH).expect("Failed to open DB");
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
    let conn = Connection::open(DB_PATH).expect("Failed to open DB");
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