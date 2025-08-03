# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Merchant is a terminal UI game - a clone of the classic Drug Wars game, reimagined as a merchant ship trading simulation set in the 18th century. The project is written in Rust with a web-based WASM version.

## Architecture

The codebase is organized as a Rust workspace with the following crates:

- **game/** - Main terminal game application with the game loop and renderer
- **core/** - Core game logic including state management, components, and game engine
- **ansi-commands/** - Terminal control and ANSI escape sequence handling
- **web/** - WebAssembly/WASM version of the game for browser deployment
- **captured_write/** - Utility for capturing write operations
- **raw_format_ansi/** - ANSI formatting utilities

### Key Architectural Components

- **Game State** (`core/src/state/`) - Manages the game's state including locations, goods, inventory, and RNG
- **Engine** (`game/src/engine.rs` and `core/src/engine.rs`) - Handles the main game loop, input processing, and state updates
- **Components** (`core/src/components/`) - UI components like screens, frames, and text rendering
- **Renderer** (`game/src/renderer.rs`) - Handles terminal rendering using crossterm

The game follows a classic game loop pattern:
1. Draw current state to terminal
2. Read user input
3. Update game state based on input
4. Return control signals (Continue, Quit, Restart)

## Development Commands

### Building
```bash
# Build all workspace crates
cargo build

# Build release version (optimized for size)
cargo build --release

# Build specific crate
cargo build -p merchant
```

### Testing
```bash
# Run all tests in workspace
cargo test

# Run tests for specific crate
cargo test -p merchant-core

# Run a specific test
cargo test test_name

# Run tests with output displayed
cargo test -- --nocapture
```

### Linting
```bash
# Run clippy on all workspace members
cargo clippy --all

# Run clippy and auto-fix issues
cargo clippy --fix
```

### Running the Game
```bash
# Run the terminal game
cargo run --release

# Run from the game directory
cd game && cargo run
```

### Web Version
```bash
# Build WASM package
cd web && wasm-pack build

# Run the web frontend development server
cd web/merchant-web && npm install && npm run dev
```

## Testing Approach

The project uses integration tests located in `test/` directories within crates. Tests use a custom test engine and RNG for deterministic testing. When writing tests:
- Use the test utilities in `core/src/test/` and `game/src/test/`
- Tests can capture and verify rendered output
- Use deterministic RNG seeds for reproducible tests

## Logging

The application uses the `tracing` crate for structured logging. Logs are written to `debug.log` when running the game. The logging system is initialized in `game/src/logging.rs` and `core/src/logging.rs`.