# 🧾 Ticket System Manager (Rust CLI)

A command-line ticket tracker built in Rust.  
Create, update, delete, and export tickets to markdown with persistent storage.

---

## 🚀 Features

- 🎟️ Create new tickets with title & description
- 📋 View all tickets
- 🔁 Update ticket status (Open, In Progress, Closed)
- 🗑️ Delete tickets
- 📄 Export tickets to `tickets.md`
- 💾 Auto-saves to `tickets.json`

---

## 🛠️ Installation

### 📦 Build from source:

```bash
git clone https://github.com/Nullgrimoire/TSMngr.git
cd TSMngr
cargo build --release
./target/release/tsmngr
```

Or just download the compiled binary from [Releases](#).

---

## 🧪 Sample Output

```
🎟️ Ticket System Manager
1️⃣ Create Ticket
2️⃣ View All Tickets
3️⃣ Update Ticket Status
4️⃣ Delete Ticket
5️⃣ Export Tickets to Markdown
6️⃣ Exit
```

---

## 📁 Data Files

- `tickets.json`: Saved ticket data
- `tickets.md`: Optional exported markdown

---

## ✨ Built With

- 🦀 Rust
- `serde`, `uuid`, `serde_json`
