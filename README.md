# 🧾 Ticket System Manager (TSMngr)

A command-line ticket tracker built in Rust.

Create, view, update, delete, and export tickets to Markdown with persistent file-based storage or SQLite database.

---

## 🚀 Download

Precompiled binaries available for:

- 🐧 **Linux**
- 🦟 **Windows**

📥 **[Latest Releases →](https://github.com/Nullgrimoire/TSMngr/releases)**

Just download the appropriate `.zip`, extract it, and run the binary (`tsmngr` or `tsmngr.exe`).

---

## ✨ Features

- 🎫 Create new tickets with title & description
- 📋 View all tickets in a list
- 🛠️ Update or delete specific tickets
- 📤 Export to `tickets.md` for backup/sharing
- 💾 SQlite persistent storage
- 🧑‍💻 Command-line interface (CLI) for scripting and automation
- 🧑‍🎤 Interactive menu for easy use
- 🧑‍🔬 Seed sample data for testing
- 🧑‍💻 All ticket actions available via CLI:
  - `ticket new <title> <description>`
  - `ticket list`
  - `ticket show <id>`
  - `ticket update <id>`
  - `ticket delete <id>`
  - `ticket export`
  - `ticket seed`
  - `ticket help`

---

## 📦 Usage

Run interactively from terminal:

```bash
./tsmngr
```

Or use CLI commands directly:

```bash
./tsmngr ticket new "Title" "Description"
./tsmngr ticket list
./tsmngr ticket show <id>
./tsmngr ticket update <id>
./tsmngr ticket delete <id>
./tsmngr ticket export
./tsmngr ticket seed
./tsmngr ticket help
```

On Windows:

```powershell
.\tsmngr.exe
```

---

## 🛠️ Build from Source

```bash
cargo build --release
```

Binary output will be in:

```text
./target/release/
```

---

## 📁 Data Storage

- Ticket data is saved in `tickets.db`
- Exports go to `tickets.md`

---

## 📜 License

MIT © Nullgrimoire
