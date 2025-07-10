# 🧾 Ticket System Manager (TSMngr)

A command-line ticket tracker built in Rust.

Create, view, update, delete, and export tickets to Markdown with persistent file-based storage.

---

## 🚀 Download

Precompiled binaries available for:

- 🐧 **Linux**
- 🪟 **Windows**

📥 **[Latest Releases →](https://github.com/Nullgrimoire/TSMngr/releases)**

Just download the appropriate `.zip`, extract it, and run the binary (`tsmngr` or `tsmngr.exe`).

---

## ✨ Features

- 🎫 Create new tickets with title & description
- 📋 View all tickets in a list
- 🛠️ Update or delete specific tickets
- 🧾 Export to `tickets.md` for backup/sharing
- 💾 SQlite persistent storage
- 🧙‍♂️ GitHub Actions builds for Windows & Linux

---

## 📦 Usage

Run from terminal:

```bash
./tsmngr
```

On Windows:

```powershell
.\tsmngr.exe
```

---

## 🛠 Build from Source

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
