# merchant

The terminal/CLI interface for the Merchant game, built on top of merchant-core.

## Overview

This crate provides the executable terminal game application, implementing:
- Terminal I/O and event handling using crossterm
- Game loop management
- ANSI command interpretation and terminal rendering
- User input processing (keyboard events)

## Architecture

### Components

- **Main Loop** (`src/main.rs`): Entry point and game initialization
- **Engine** (`src/engine.rs`): Terminal-specific game loop implementation
- **Renderer** (`src/renderer.rs`): Translates ansi-commands from core into actual terminal output via crossterm
- **Input Handler**: Captures keyboard events and forwards them to the core engine

### Rendering Pipeline

1. Core engine produces screens with ansi-commands
2. Renderer interprets these commands
3. Crossterm executes terminal operations
4. Terminal displays the game interface

## Running the Game

```bash
# Run from this directory
cargo run --release

# Or from workspace root
cargo run -p merchant --release
```

## Features

- Full terminal UI with color support
- Responsive keyboard controls
- Clean screen management and cursor control
- Graceful shutdown and terminal state restoration
- Debug logging to `debug.log`

## Dependencies

- `merchant-core`: Game logic and state management
- `ansi-commands`: Terminal command abstractions
- `crossterm`: Terminal manipulation and input handling
- `tracing`: Structured logging

## Design

This crate acts as a thin terminal-specific layer over the platform-agnostic core. It:
- Handles all terminal-specific concerns
- Translates between crossterm events and core game events
- Manages the terminal lifecycle (raw mode, alternate screen, etc.)
- Provides the runtime environment for the console version of the game