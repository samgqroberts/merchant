# terminal-commands

A lightweight library providing crossterm-like terminal control commands without requiring the crossterm crate as a dependency.

Rendering (consumption) of the commands established here is left to clients, per their target environment.

The API here is designed to be as close to crossterm's API as possible, so rendering for the terminal environment via crossterm is straightforward, but other environments are now possible.
