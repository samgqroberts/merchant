mod draw;
mod state;
mod update;

use std::{
    io::{self},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use draw::draw_scene;
use state::GameState;
use update::update;

fn main_loop(game_state: &GameState) -> io::Result<(bool, Option<GameState>)> {
    // draw the game state
    draw_scene(game_state)?;
    // Wait for any user event
    loop {
        // Wait up to 1s for some user event per loop iteration
        if poll(Duration::from_millis(1_000))? {
            // Read what even happened from the poll
            // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
            match read()? {
                Event::Key(event) => {
                    // detect exit request
                    if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c')
                    {
                        return Ok((true, None));
                    }
                    // move forward game state
                    if !game_state.initialized {
                        // initialize game
                        return Ok((false, Some(game_state.initialize())));
                    } else {
                        return update(event, game_state).map(|st| (false, st));
                    }
                }
                _ => continue,
            }
        } else {
            // Timeout expired, no event for 1s, wait for user input again
            continue;
        }
    }
}

fn main() -> io::Result<()> {
    // set terminal into "non-canonical" mode so inputs are captured raw with no interpretation
    // https://docs.rs/crossterm/0.26.1/crossterm/terminal/index.html#raw-mode
    enable_raw_mode()?;
    // start main game loop, which draws -> reads input -> updates state, with fresh game state
    let mut game_state = GameState::new();
    loop {
        match main_loop(&mut game_state) {
            Err(e) => {
                // an io error was encountered in main game loop
                println!("Error: {:?}\r", e);
            }
            Ok((should_exit, new_game_state)) => {
                if should_exit {
                    // main loop told us user requested an exit
                    break;
                }
                if let Some(new_game_state) = new_game_state {
                    // main loop gave back a new game state
                    game_state = new_game_state
                }
            }
        }
    }
    // set terminal back to canonical mode before exiting
    disable_raw_mode()
}
