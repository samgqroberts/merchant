# merchant-core

The core logic library for the Merchant game, providing game state management, business logic, and rendering abstractions.

## Overview

This crate contains:
- **Game State**: Complete game state including locations, goods, inventory, player stats, and events
- **Game Logic**: All game mechanics, trading rules, event generation, and state transitions
- **Rendering Logic**: Screen composition and layout that transforms game state into displayable screens

## Architecture

### State Management
The game state (`src/state/`) contains all runtime data:
- Player inventory and finances
- Current location and available destinations
- Market prices and goods
- Random events and their effects
- Game configuration and difficulty settings

### Rendering Pipeline
The rendering system (`src/components/`) produces screen representations:
1. Takes current game state as input
2. Generates screen layouts and UI components
3. Outputs commands using the `terminal-commands` crate
4. Renderers interpret these commands for their target platform

### Platform Independence
By expressing all display operations through `terminal-commands`, the core remains decoupled from specific rendering backends. This allows:
- Terminal rendering via crossterm (in the `game` crate)
- Web rendering via HTML/CSS (in the `web` crate)
- Potential future renderers without core logic changes

## Usage

```rust
use merchant_core::{State, Engine, Event};

// Initialize game state
let mut state = State::new();

// Process game events
let event = Event::UserInput(/* ... */);
let control_signal = Engine::process_event(&mut state, event);

// Generate screen for current state
let screen = Engine::render(&state);
// Screen contains terminal-commands that renderers interpret
```

## Design Principles

- **Pure Logic**: No I/O operations, only data transformations
- **Renderer Agnostic**: All display operations expressed as abstract commands
- **Testable**: Deterministic game logic with configurable RNG for testing
- **Modular**: Clear separation between state, logic, and presentation
