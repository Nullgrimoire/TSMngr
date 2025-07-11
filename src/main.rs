mod ticket;
mod storage;
mod cli;
mod error;

use ticket::Ticket;
use storage::{init_db, load_tickets, save_tickets};

use std::io::{self, Write};


fn main() {
    let _ = init_db();
    if cli::handle_args() {
        return;
    }

    let mut tickets = match load_tickets() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to load tickets: {}", e);
            Vec::new()
        }
    };

    loop {
        clear_screen();
        println!("🎟️ Ticket System Manager");
        println!("1️⃣ Create Ticket");
        println!("2️⃣ View All Tickets");
        println!("3️⃣ Update Ticket Status");
        println!("4️⃣ Delete Ticket");
        println!("5️⃣ Export Tickets to Markdown");
        println!("6️⃣ Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                clear_screen();
                println!("\n🆕 Create a Ticket\n-------------------");
                let title = prompt("Enter ticket title: ");
                let description = prompt("Enter ticket description: ");
                let ticket = Ticket::new(&title, &description);
                println!("\n✅ Ticket created: {:?}", ticket);
                pause();
                tickets.push(ticket);
                let _ = save_tickets(&tickets);
            },
            "2" => {
                clear_screen();
                if tickets.is_empty() {
                    println!("📭 No tickets found.");
                } else {
                    println!("\n📋 All Tickets\n-------------------");
                    for (i, ticket) in tickets.iter().enumerate() {
                        println!("{}. {} [{}]", i + 1, ticket.title, color_status(&ticket.status));
                    }
                }
                pause();
            },
            "3" => {
                clear_screen();
                if tickets.is_empty() {
                    println!("📭 No tickets to update.");
                    continue;
                }
                println!("\n🔁 Update Ticket Status\n-------------------");
                for (i, ticket) in tickets.iter().enumerate() {
                    println!("{}. {} [{}]", i + 1, ticket.title, color_status(&ticket.status));
                }
                let input = prompt("Enter ticket number: ");
                let index = match input.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= tickets.len() => num - 1,
                    _ => {
                        println!("❌ Invalid selection.");
                        continue;
                    }
                };
                println!("\nSelect new status:");
                println!("1. Open");
                println!("2. In Progress");
                println!("3. Closed");
                let status_choice = prompt("Choice: ");
                let new_status = match status_choice.trim() {
                    "1" => "Open",
                    "2" => "In Progress",
                    "3" => "Closed",
                    _ => {
                        println!("❌ Invalid status.");
                        continue;
                    }
                };
                tickets[index].status = new_status.to_string();
                let _ = save_tickets(&tickets);
                println!("\n✅ Ticket updated.");
                pause();
            },
            "4" => {
                clear_screen();
                if tickets.is_empty() {
                    println!("📭 No tickets to delete.");
                    pause();
                    continue;
                }
                println!("\n🗑️ Delete a Ticket\n-------------------");
                for (i, ticket) in tickets.iter().enumerate() {
                    println!("{}. {} [{}]", i + 1, ticket.title, color_status(&ticket.status));
                }
                let input = prompt("Enter ticket number to delete: ");
                let index: usize = match input.trim().parse::<usize>() {
                    Ok(num) if num > 0 && num <= tickets.len() => num - 1,
                    _ => {
                        println!("❌ Invalid selection.");
                        pause();
                        continue;
                    }
                };
                let removed = tickets.remove(index);
                let _ = save_tickets(&tickets);
                println!("\n🗑️ Deleted: {} [{}]", removed.title, color_status(&removed.status));
                pause();
            },
            "5" => {
                clear_screen();
                export_to_markdown(&tickets);
                println!("📄 Tickets exported to tickets.md");
                pause();
            },
            "6" => {
                println!("💾 Saving and exiting...");
                let _ = save_tickets(&tickets);
                break;
            },
            _ => {
                println!("❌ Invalid option.");
                pause();
            }
        }
    }
}

fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

//utility functions
fn clear_screen() {
    // Works on Unix (Linux/macOS)
    print!("\x1B[2J\x1B[1;1H");
}
fn color_status(status: &str) -> String {
    match status {
        "Open" => format!("\x1b[32m{}\x1b[0m", status),         // Green
        "In Progress" => format!("\x1b[33m{}\x1b[0m", status),   // Yellow
        "Closed" => format!("\x1b[31m{}\x1b[0m", status),        // Red
        _ => status.to_string(),
    }
}
fn pause() {
    println!("\nPress Enter to return to menu...");
    let _ = std::io::stdin().read_line(&mut String::new());
}

//Markdown Export
use std::fs::File;

fn export_to_markdown(tickets: &[Ticket]) {
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
