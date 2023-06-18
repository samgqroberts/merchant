use std::{
    io::{self},
    time::Duration,
};

use crossterm::{
    cursor::{MoveTo, MoveToNextLine},
    event::{poll, read, Event, KeyCode, KeyModifiers},
    execute,
    style::{style, Attribute, Color, PrintStyledContent, Stylize},
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
};

struct GameState {
    initialized: bool,
}

impl GameState {
    fn new() -> GameState {
        GameState { initialized: false }
    }
}

fn main_loop(game_state: &mut GameState) -> io::Result<(bool, Option<GameState>)> {
    // Wait up to 1s for some user event
    if poll(Duration::from_millis(1_000))? {
        // Read what even happened from the poll
        // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
        match read()? {
            Event::Key(event) => {
                // detect exit request
                if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c') {
                    return Ok((true, None));
                }
                // move forward game state
                if !game_state.initialized {
                    // initialize game
                    execute!(
                        io::stdout(),
                        // clear terminal
                        Clear(crossterm::terminal::ClearType::All),
                        // reset cursor position to top left
                        MoveTo(0, 0),
                        // write out new game state
                        PrintStyledContent(
                            "Game has begun."
                                .with(Color::White)
                                .on(Color::Red)
                                .attribute(Attribute::Bold)
                        ),
                    )?;
                    return Ok((false, Some(GameState { initialized: true })));
                }
                // user event had no effect
                Ok((false, None))
            }
            _ => Ok((false, None)),
        }
    } else {
        // Timeout expired, no event for 1s
        Ok((false, None))
    }
}

fn main() -> io::Result<()> {
    // initial splash screen
    execute!(
        io::stdout(),
        // clear terminal
        Clear(crossterm::terminal::ClearType::All),
        // reset cursor position to top left
        MoveTo(0, 0),
        // write out splash screen
        PrintStyledContent(
            "Merchant\n\nNavigate shifting markets and unreliable sources.\n\nBy samgqroberts"
                .with(Color::Yellow)
                .on(Color::Blue)
                .attribute(Attribute::Bold)
        ),
        // prompt user
        MoveToNextLine(2),
        PrintStyledContent(
            style("Press any key to begin")
                .with(Color::Blue)
                .on(Color::Yellow)
                .attribute(Attribute::Bold),
        ),
    )?;
    // set terminal into "non-canonical" mode so inputs are captured raw with no interpretation
    // https://docs.rs/crossterm/0.26.1/crossterm/terminal/index.html#raw-mode
    enable_raw_mode()?;
    // start main game loop, with fresh game state
    let mut game_state = GameState::new();
    loop {
        // perform main loop logic, detect and handle potential io error
        match main_loop(&mut game_state) {
            Err(e) => {
                // an error was encountered in main game loop
                println!("Error: {:?}\r", e);
            }
            Ok((should_exit, new_game_state)) => {
                // main loop may have told us user requested an exit
                if should_exit {
                    break;
                }
                // main loop may have given back a new game state
                if let Some(new_game_state) = new_game_state {
                    game_state = new_game_state
                }
            }
        }
    }
    // set terminal back to canonical mode before exiting
    disable_raw_mode()
}
