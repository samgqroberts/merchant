# terminal-commands

A lightweight library providing crossterm-like terminal control commands without requiring the crossterm crate as a dependency.

## Purpose

This crate defines a set of terminal control commands that mirror crossterm's API, but as pure data structures. This allows different rendering backends to interpret these commands according to their needs, without being tied to a specific terminal manipulation library.

## Design

The crate provides:
- Command enums that represent terminal operations (cursor movement, styling, clearing, etc.)
- No direct terminal I/O - commands are just data
- A minimal API surface that can be consumed by various renderer implementations

## Usage

Renderers can consume these commands and translate them to:
- ANSI escape sequences for terminal output
- HTML/CSS for web-based rendering
- Any other display format

This separation of concerns allows the core game engine to speak in abstract terminal commands while keeping rendering logic isolated in specialized crates.

## Example

```rust
use terminal_commands::{Command, CursorCommand};

// Create commands without performing I/O
let commands = vec![
    Command::Cursor(CursorCommand::MoveTo(10, 5)),
    Command::Print("Hello, World!".to_string()),
];

// Renderer interprets commands (implementation-specific)
// renderer.execute(commands);
```
