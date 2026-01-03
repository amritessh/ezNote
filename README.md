# ezNote 

> Zero-friction note taking for developers who live in the terminal

[![Release](https://img.shields.io/github/v/release/amritessh/eznote?style=flat-square)](https://github.com/amritessh/eznote/releases)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)
[![Build](https://img.shields.io/github/actions/workflow/status/amritessh/eznote/release.yml?style=flat-square)](https://github.com/amritessh/eznote/actions)

Built with 🦀 Rust for maximum speed and reliability.

<p align="center">
  <img src="https://img.shields.io/badge/startup-<10ms-brightgreen?style=flat-square" alt="Startup Time">
  <img src="https://img.shields.io/badge/binary-~5MB-blue?style=flat-square" alt="Binary Size">
  <img src="https://img.shields.io/badge/dependencies-zero-orange?style=flat-square" alt="Dependencies">
</p>

---

## Why ezNote?

As developers, we live in the terminal. But capturing quick thoughts meant:
- Opening Notion (slow, context switch)
- Finding a text editor (where did I save that file?)
- Using pen and paper (gets lost immediately)

**ezNote solves this:** One command, instant capture, never lose a thought again.

```bash
ezn add "Fix authentication bug in prod" --tag urgent --priority high
```

Done. Back to coding in milliseconds.

---

## Features

- **Lightning Fast** - Sub-10ms startup, instant note capture
- **Zero Dependencies** - Single binary, no Rust toolchain needed
- **Full-Text Search** - Find notes instantly with SQLite FTS5
- **Smart Organization** - Tags and priorities for easy filtering
- **Statistics** - Track your note-taking habits
- **Local-First** - Your data stays on your machine, forever
- **Beautiful UI** - Colored output, clean terminal interface
- **Cross-Platform** - Works on macOS (Intel + ARM), Linux, Windows

---

## Installation

### Homebrew (Recommended - macOS/Linux)

```bash
brew tap amritesh/eznote
brew install eznote
```

### Quick Install Script (macOS/Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/amritesh/eznote/main/install.sh | bash
```

### Cargo (For Rust developers)

```bash
cargo install eznote
```

### Manual Installation

Download the latest binary for your platform from [Releases](https://github.com/amritesh/eznote/releases):

**macOS:**
```bash
# Download ezn-macos-aarch64 (M1/M2) or ezn-macos-x86_64 (Intel)
chmod +x ezn-*
sudo mv ezn-* /usr/local/bin/ezn
```

**Linux:**
```bash
# Download ezn-linux-x86_64 or ezn-linux-aarch64
chmod +x ezn-*
sudo mv ezn-* /usr/local/bin/ezn
```

**Windows:**
```bash
# Download ezn-windows-x86_64.exe
# Rename to ezn.exe and add to PATH
```

---

## Quick Start

```bash
# Add your first note
ezn add "My first note with ezNote!"

# Add a note with tags and priority
ezn add "Review architecture docs" --tag work --priority high

# List recent notes
ezn list

# Search your notes
ezn search "architecture"

# See today's activity
ezn today

# View statistics
ezn stats
```

---

## Commands

### Core Commands

| Command | Description | Example |
|---------|-------------|---------|
| `add <text>` | Create a new note | `ezn add "Deploy to production" --tag devops --priority urgent` |
| `list` | Show recent notes | `ezn list --today --tag work --limit 10` |
| `search <query>` | Full-text search | `ezn search "authentication bug"` |
| `show <id>` | Display note details | `ezn show 5` |
| `delete <id>` | Remove a note | `ezn delete 5` or `ezn delete 5 --force` |
| `today` | Show today's notes | `ezn today` |
| `stats` | Show statistics | `ezn stats` |

### Command Options

**`add` options:**
- `--tag <tag>` or `-t <tag>` - Add tags (can specify multiple times)
- `--priority <level>` or `-p <level>` - Set priority: `low`, `medium`, `high`, `urgent`

**`list` options:**
- `--today` - Show only today's notes
- `--tag <tag>` or `-t <tag>` - Filter by specific tag
- `--limit <n>` or `-l <n>` - Limit results (default: 20)

**`delete` options:**
- `--force` or `-f` - Skip confirmation prompt

---

## 🎯 Real-World Use Cases

### Daily Developer Workflow
```bash
# Morning standup
ezn add "Today: Auth bug, PR reviews, staging deploy" --tag standup

# Quick bug tracking during coding
ezn add "Safari login fails - check CORS headers" --tag bug --priority urgent

# Meeting notes
ezn add "Architecture sync: moving to microservices Q2" --tag meeting

# End of day review
ezn today
```

### Task & TODO Management
```bash
# Capture tasks as they come
ezn add "Setup CI/CD for new service" --tag todo --priority high
ezn add "Update API documentation" --tag todo --priority medium
ezn add "Review security audit report" --tag todo --priority urgent

# Filter and manage
ezn list --tag todo
ezn search "CI/CD"
```

### Learning & Research Notes
```bash
# Capture learning moments
ezn add "Rust ownership: each value has exactly one owner" --tag learning

# Save research references
ezn add "Check paper: 'Attention Is All You Need' for transformer architecture" --tag research

# Quick retrieval
ezn search "transformer"
ezn list --tag learning
```

### Interview Prep & Ideas
```bash
# Technical interview prep
ezn add "Binary search: O(log n), requires sorted array" --tag interview

# Startup ideas
ezn add "App idea: AI-powered commit message generator" --tag ideas --priority high

# Later review
ezn list --tag interview
ezn list --tag ideas
```

---

## Performance

Benchmarked on MacBook Pro M2:

- **Startup time:** <10ms (cold start)
- **Add note:** <5ms
- **Search 10,000 notes:** <50ms
- **List operations:** <20ms
- **Binary size:** ~3-5MB
- **Memory usage:** <10MB

---

## Data Storage

Your notes are stored locally in a single SQLite database:

- **macOS:** `~/Library/Application Support/eznote/notes.db`
- **Linux:** `~/.local/share/eznote/notes.db`
- **Windows:** `%APPDATA%\eznote\notes.db`

**Benefits:**
- Easy to backup (single file)
- Sync with any cloud storage (Dropbox, iCloud, Google Drive)
- Query directly with SQLite tools
- Export and migrate easily
- No vendor lock-in

---

## Development

### Build from Source

**Prerequisites:**
- Rust 1.70+ ([install from rustup.rs](https://rustup.rs))

**Build:**
```bash
git clone https://github.com/amritesh/eznote.git
cd eznote
cargo build --release
sudo cp target/release/ezn /usr/local/bin/
```


**Development Mode:**
```bash
cargo run -- add "Test note"
cargo run -- list
```

### Project Structure

```
eznote/
├── src/
│   ├── main.rs          # Entry point
│   ├── cli/             # CLI commands and output
│   ├── db/              # Database connection and schema
│   ├── models/          # Data models (Note, Tag, Priority)
│   ├── services/        # Business logic (CRUD, search)
│   └── utils/           # Helper functions
├── Cargo.toml           # Dependencies
└── README.md
```

---

## 🗺️ Roadmap

### ✅ Completed (v0.1.x)
- [x] Core note-taking functionality
- [x] Tags and priorities
- [x] Full-text search with SQLite FTS5
- [x] Statistics and analytics
- [x] Homebrew distribution
- [x] Cross-platform binaries (macOS, Linux, Windows)
- [x] Beautiful colored terminal UI

### 🚧 Coming Soon (v0.2.x)
- [ ] Edit notes in $EDITOR
- [ ] Archive/unarchive notes
- [ ] Export to Markdown, JSON, CSV
- [ ] Git context detection (auto-tag by repo/branch)
- [ ] Sync notes across devices
- [ ] Import from other note-taking tools

###  Future (v0.3.x+)
- [ ] Web UI for browsing notes
- [ ] Mobile companion app
- [ ] AI-powered suggestions and categorization
- [ ] Team collaboration features
- [ ] Plugin system for extensibility
- [ ] Integration with Obsidian, Notion, etc.

---

##  Contributing

Contributions are welcome! Whether it's:

-  Bug reports
-  Feature requests
-  Documentation improvements
-  Code contributions

**How to contribute:**

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

##  License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

##  Support & Feedback

-  **Found a bug?** [Open an issue](https://github.com/amritesh/eznote/issues/new?labels=bug)
-  **Feature request?** [Open an issue](https://github.com/amritesh/eznote/issues/new?labels=enhancement)
-  **Questions?** [Start a discussion](https://github.com/amritesh/eznote/discussions)
-  **Like ezNote?** Give us a star on GitHub!

---

## Acknowledgments

Built with amazing open-source tools:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [Clap](https://github.com/clap-rs/clap) - Command line argument parser
- [SQLite](https://www.sqlite.org/) - Embedded database
- [Colored](https://github.com/colored-rs/colored) - Terminal colors

Special thanks to the Rust community for excellent documentation and support.

---

## Connect

Built with ❤️ by [Amritesh Anand](https://github.com/amritessh)

- 🔗 [LinkedIn](https://linkedin.com/in/amritessh)
- 🌐 [Portfolio](https://amriteshanand.com)
- 🐦 [Twitter](https://twitter.com/amritessh)

---

<p align="center">
  <b>If ezNote helps you stay organized, please ⭐ star the repo!</b>
</p>

<p align="center">
  <sub>Made with Rust 🦀 | Powered by Coffee ☕</sub>
</p>
