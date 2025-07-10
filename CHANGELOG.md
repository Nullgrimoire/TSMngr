# 📜 Changelog

All notable changes to this project will be documented in this file.

This project adheres to [Semantic Versioning](https://semver.org/).

---

## [v0.1.4] - 2025-07-10
### Added
- 🛠️ GitHub Actions workflow for automated cross-platform builds (Linux & Windows)
- 📦 Release artifacts are now zipped and uploaded on each version tag
- 📁 `README.md` and `LICENSE` included in release zip packages

### Fixed
- 🪟 Switched to PowerShell `Compress-Archive` for Windows compatibility
- 🧹 Removed previously tracked files now ignored by `.gitignore`

---

## [v0.1.3] - 2025-07-10
### Fixed
- ⚙️ Cleaned up tracked build artifacts and added `.gitignore` rules
- 🔧 Updated workflow to address missing shell syntax on Windows builds

---

## [v0.1.2] - 2025-07-10
### Fixed
- 📄 Corrected JSON structure in `tickets.json` file
- 🧪 Updated tests and validated release stability

---

## [v0.1.1] - 2025-07-09
### Added
- 🪵 Initial logging for ticket actions (create, update, delete)
- 💾 File-based storage for persistent tickets

---

## [v0.1.0] - 2025-07-09
### Added
- 🧙‍♂️ Initial public release of TSMngr CLI
- 🎫 Create, view, update, delete tickets via command line
- 📝 Export tickets to markdown
- 📂 JSON-backed persistent ticket storage
