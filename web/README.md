# merchant-web

The browser/web interface for the Merchant game, built on top of merchant-core.

## Overview

This crate provides the WebAssembly (WASM) version of the game, implementing:
- WASM bindings for merchant-core functionality
- HTML/CSS rendering of game screens
- Browser event handling (keyboard/mouse input)
- Web-specific game loop management

## Architecture

### Components

- **WASM Module** (`src/lib.rs`): Exposes core game functionality to JavaScript
- **Renderer** (`src/renderer.rs`): Translates terminal-commands from core into HTML/DOM operations
- **Input Handler**: Captures browser events and forwards them to the core engine
- **Frontend** (`merchant-web/`): React-based web application that hosts the WASM module

### Rendering Pipeline

1. Core engine produces screens with terminal-commands
2. WASM renderer interprets these commands
3. Commands are translated to HTML/CSS styling
4. Browser displays the game interface

## Building

```bash
# Build WASM package
wasm-pack build

# Install frontend dependencies
cd merchant-web && npm install

# Run development server
npm run dev

# Build for production
npm run build
```

## Features

- Full browser-based gameplay
- Responsive keyboard controls
- Terminal-like appearance using web technologies
- Cross-platform compatibility (any modern browser)
- No server required - runs entirely client-side

## Dependencies

- `merchant-core`: Game logic and state management
- `terminal-commands`: Terminal command abstractions
- `wasm-bindgen`: Rust/WASM interop
- `web-sys`: Browser API bindings
- React & TypeScript: Frontend framework

## Design

This crate acts as a browser-specific layer over the platform-agnostic core. It:
- Handles all web-specific concerns
- Translates between browser events and core game events
- Manages the WASM lifecycle and JavaScript interop
- Provides a terminal-like experience in the browser
- Maintains visual fidelity with the terminal version
