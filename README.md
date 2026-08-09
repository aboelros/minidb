# MiniDB

MiniDB is a serious systems/database engineering portfolio project. It is a fully custom relational database engine written entirely from scratch in Rust, without relying on any underlying existing database engines. 

## Features

* **Custom Lexer & Parser**: Implements a recursive descent parser to interpret complex SQL queries (CREATE, INSERT, SELECT, UPDATE, DELETE, BEGIN, COMMIT, ROLLBACK).
* **Binary Serialization**: Converts relational tuples directly into binary offsets mapped sequentially for on-disk layouts.
* **ACID Transaction Foundation**: Employs standard Concurrency Control frameworks (Two-Phase Locking, Buffer Pool Managers, Log Sequence Numbers).
* **Native Desktop Interface**: Instead of just a terminal CLI, the engine runs natively underneath a sleek React/Tauri desktop application frontend.

## Architecture & Tech Stack

- **Core Engine (Backend)**: Built natively in `Rust`, leveraging `clap` and `thiserror` to construct robust CLI interactions and predictable type-safe query execution.
- **Desktop Wrapper**: Tauri framework.
- **Frontend UI**: React + Vite, designed with modern dark mode and frosted glassmorphism visual aesthetics.

## How to Run

### Prerequisite
You need `rustup` (with the MSVC or GNU toolchain) and `Node.js` installed.

1. Clone the repository.
2. Open your terminal and navigate to the UI folder:
   ```bash
   cd ui-tauri
   ```
3. Install frontend dependencies:
   ```bash
   npm install
   ```
4. Run the desktop application in development mode:
   ```bash
   npm run tauri dev
   ```

A desktop window will open displaying the MiniDB Explorer, allowing you to directly query the database engine in memory!

## 🛠️ Creator & Architect

This system was designed, structured, and engineered entirely by:
- **Lead Developer:** Mahmoud Aboelros (19yo)
- **Location:** Cairo, Egypt
- **Email Contact:** [mahmoudaboelros12358@gmail.com](mailto:mahmoudaboelros12358@gmail.com)
- **Connect on Instagram:** [@arnold19i](https://instagram.com/arnold19i)

*All Rights Reserved © 2026 Mahmoud Aboelros*
