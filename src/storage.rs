use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use crate::ticket::Ticket;

const FILE_PATH: &str = "tickets.json";

pub fn save_tickets(tickets: &Vec<Ticket>) {
    let json = serde_json::to_string_pretty(tickets).unwrap();
    let mut file = File::create(FILE_PATH).expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write to file");
}

pub fn load_tickets() -> Vec<Ticket> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();
    }

    let mut file = OpenOptions::new()
        .read(true)
        .open(FILE_PATH)
        .expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read file");

    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}
