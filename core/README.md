# merchant-core

The core logic library for the Merchant game, providing game state management, business logic, and rendering abstractions.

This crate implements all game logic and state management in a way that's agnostic to the deployment / rendering environment.

Clients of this crate can provide a `Renderer` implementation to render the game state in their environment.
