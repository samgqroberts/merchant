mod draw;
mod state;
#[cfg(test)]
mod test;
mod update;

use rand::{rngs::StdRng, SeedableRng};
use std::cell::RefCell;
use std::io::Stdout;
use std::{
    io::Write,
    io::{self, stdout},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use draw::Drawer;
use state::GameState;
use update::{update, UpdateError};

pub struct Engine<'a, Writer: Write> {
    drawer: Drawer<'a, Writer>,
}

impl<'a, Writer: Write> Engine<'a, Writer> {
    pub fn new(writer: &'a RefCell<Writer>) -> Self {
        Self {
            drawer: Drawer { writer },
        }
    }

    pub fn draw(&mut self, game_state: &GameState) -> io::Result<()> {
        self.drawer.draw_scene(game_state)
    }

    pub fn draw_and_prompt(
        &mut self,
        game_state: &GameState,
    ) -> Result<(bool, Option<GameState>), UpdateError> {
        // draw the game state
        self.draw(game_state)?;
        // Wait for any user event
        loop {
            // Wait up to 1s for some user event per loop iteration
            if poll(Duration::from_millis(1_000))? {
                // Read what even happened from the poll
                // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
                match read()? {
                    Event::Key(event) => {
                        // detect exit request
                        if event.modifiers == KeyModifiers::CONTROL
                            && event.code == KeyCode::Char('c')
                        {
                            return Ok((true, None));
                        }
                        // move forward game state
                        return update(event, game_state).map(|st| (false, st));
                    }
                    _ => continue,
                }
            } else {
                // Timeout expired, no event for 1s, wait for user input again
                continue;
            }
        }
    }

    pub fn exit_message(&mut self, msg: &[&str]) -> io::Result<()> {
        self.drawer.exit_message(msg)
    }
}

fn main() -> io::Result<()> {
    let stdout = stdout();
    // set terminal into "non-canonical" mode so inputs are captured raw with no interpretation
    // https://docs.rs/crossterm/0.26.1/crossterm/terminal/index.html#raw-mode
    enable_raw_mode()?;
    // start main game loop, which draws -> reads input -> updates state, with fresh game state
    let rng = StdRng::from_entropy();
    let mut game_state = GameState::new(rng);
    let writer: RefCell<Stdout> = RefCell::from(stdout);
    let mut engine = Engine::new(&writer);
    loop {
        match engine.draw_and_prompt(&mut game_state) {
            Err(e) => {
                // an error was encountered in main game loop
                let msg = format!("{:?}", e);
                let lines = msg.split("\\n").collect::<Vec<&str>>();
                engine.exit_message(&lines)?;
                break;
            }
            Ok((should_exit, new_game_state)) => {
                if should_exit {
                    // main loop told us user requested an exit
                    engine.exit_message(&["Thank you for playing!"])?;
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
