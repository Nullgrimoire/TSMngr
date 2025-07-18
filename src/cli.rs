//! Command-line interface (CLI) for ticket management.

use std::env;

use crate::{ticket::Ticket, storage::{load_tickets, save_tickets}};
use crate::error::handle_db_err;
use crate::utils::{prompt, export_to_markdown};

/// Handle CLI arguments. Returns `true` if a CLI command was executed.
///
/// Supported commands:
/// - ticket new <title> <description>
/// - ticket list
/// - ticket show <id>
/// - ticket update <id>
/// - ticket delete <id>
/// - ticket export
/// - ticket seed
/// - ticket help
pub fn handle_args() -> bool {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("ticket") => {
            match args.next().as_deref() {
                Some("new") => { handle_new(&mut args); true }
                Some("list") => { handle_list(); true }
                Some("show") => { handle_show(&mut args); true }
                Some("update") => { handle_update(&mut args); true }
                Some("delete") => { handle_delete(&mut args); true }
                Some("export") => { handle_export(); true }
                Some("seed") => { handle_seed(); true }
                _ => { print_ticket_help(); true }
            }
        }
        _ => false,
    }
}

fn handle_new(args: &mut impl Iterator<Item=String>) {
    let title = match args.next() {
        Some(t) => t,
        None => {
            eprintln!("Title required");
            return;
        }
    };
    let description = args.next().unwrap_or_else(|| "".to_string());
    let mut tickets = handle_db_err(load_tickets()).unwrap_or_default();
    let ticket = Ticket::new(&title, &description);
    tickets.push(ticket.clone());
    let _ = handle_db_err(save_tickets(&tickets));
    println!("Created ticket {} ({})", ticket.title, ticket.id);
}

fn handle_list() {
    let tickets = handle_db_err(load_tickets()).unwrap_or_default();
    if tickets.is_empty() {
        println!("No tickets found.");
    } else {
        for t in tickets {
            println!("- {} ({}) [{}]", t.title, t.id, t.status);
        }
    }
}

fn handle_show(args: &mut impl Iterator<Item=String>) {
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
}

fn handle_update(args: &mut impl Iterator<Item=String>) {
    if let Some(id) = args.next() {
        let mut tickets = handle_db_err(load_tickets()).unwrap_or_default();
        if let Some(ticket) = tickets.iter_mut().find(|t| t.id == id) {
            println!("Current status: {}", ticket.status);
            println!("Select new status:");
            println!("1. Open");
            println!("2. In Progress");
            println!("3. Closed");
            let status_choice = prompt("Choice: ");
            let new_status = match status_choice.trim() {
                "1" => "Open",
                "2" => "In Progress",
                "3" => "Closed",
                _ => {
                    println!("Invalid status.");
                    return;
                }
            };
            ticket.status = new_status.to_string();
            let _ = handle_db_err(save_tickets(&tickets));
            println!("Ticket status updated.");
        } else {
            println!("Ticket not found");
        }
    } else {
        eprintln!("ID required");
    }
}

fn handle_delete(args: &mut impl Iterator<Item=String>) {
    if let Some(id) = args.next() {
        let mut tickets = handle_db_err(load_tickets()).unwrap_or_default();
        let orig_len = tickets.len();
        tickets.retain(|t| t.id != id);
        if tickets.len() < orig_len {
            let _ = handle_db_err(save_tickets(&tickets));
            println!("Ticket deleted.");
        } else {
            println!("Ticket not found");
        }
    } else {
        eprintln!("ID required");
    }
}

fn handle_export() {
    let tickets = handle_db_err(load_tickets()).unwrap_or_default();
    export_to_markdown(&tickets);
    println!("Tickets exported to tickets.md");
}

fn handle_seed() {
    use crate::storage::seed_sample_data;
    if handle_db_err(seed_sample_data()).is_some() {
        println!("Sample users and tickets inserted.");
    }
}

/// Print help for all available ticket CLI commands.
fn print_ticket_help() {
    println!("Ticket commands:");
    println!("  ticket new <title> <description>");
    println!("  ticket list");
    println!("  ticket show <id>");
    println!("  ticket update <id>");
    println!("  ticket delete <id>");
    println!("  ticket export");
    println!("  ticket seed");
    println!("  ticket help");
}