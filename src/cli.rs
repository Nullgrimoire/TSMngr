use std::env;

use crate::{ticket::Ticket, storage::{load_tickets, save_tickets}};
use crate::error::handle_db_err;

/// Handle CLI arguments. Returns `true` if a CLI command was executed.
pub fn handle_args() -> bool {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("ticket") => {
            match args.next().as_deref() {
                Some("new") => {
                    let title = match args.next() {
                        Some(t) => t,
                        None => {
                            eprintln!("Title required");
                            return true;
                        }
                    };
                    let description = args.next().unwrap_or_else(|| "".to_string());
                    let mut tickets = handle_db_err(load_tickets()).unwrap_or_default();
                    let ticket = Ticket::new(&title, &description);
                    tickets.push(ticket.clone());
                    let _ = handle_db_err(save_tickets(&tickets));
                    println!("Created ticket {} ({})", ticket.title, ticket.id);
                    true
                }
                Some("list") => {
                    let tickets = handle_db_err(load_tickets()).unwrap_or_default();
                    if tickets.is_empty() {
                        println!("No tickets found.");
                    } else {
                        for t in tickets {
                            println!("- {} ({}) [{}]", t.title, t.id, t.status);
                        }
                    }
                    true
                }
                Some("show") => {
                    if let Some(id) = args.next() {
                        let tickets = handle_db_err(load_tickets()).unwrap_or_default();
                        match tickets.iter().find(|t| t.id == id) {
                            Some(t) => {
                                println!("ID: {}", t.id);
                                println!("Title: {}", t.title);
                                println!("Description: {}", t.description);
                                println!("Status: {}", t.status);
                            }
                            None => println!("Ticket not found"),
                        }
                    } else {
                        eprintln!("ID required");
                    }
                    true
                }
                Some("seed") => {
                    use crate::storage::seed_sample_data;
                    if handle_db_err(seed_sample_data()).is_some() {
                        println!("Sample users and tickets inserted.");
                    }
                    true
                }
                _ => {
                    print_ticket_help();
                    true
                }
            }
        }
        _ => false,
    }
}

fn print_ticket_help() {
    println!("Ticket commands:");
    println!("  ticket new <title> <description>");
    println!("  ticket list");
    println!("  ticket show <id>");
    println!("  ticket seed");
    println!("  ticket help");
}