mod engine;
#[macro_use]
mod macros;
mod components;
mod logging;
mod state;
#[cfg(test)]
mod test;

use engine::{Engine, UpdateSignal};
use logging::initialize_logging;
use rand::{rngs::StdRng, SeedableRng};
use std::cell::RefCell;
use std::io::Stdout;
use std::io::{self, stdout};
use tracing::{error, info, span, Level};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use state::GameState;

fn main() -> io::Result<()> {
    initialize_logging();
    let stdout = stdout();
    // set terminal into "non-canonical" mode so inputs are captured raw with no interpretation
    // https://docs.rs/crossterm/0.26.1/crossterm/terminal/index.html#raw-mode
    enable_raw_mode()?;
    // initialize game state with RNG
    let rng = StdRng::from_entropy();
    let mut game_state = GameState::new_std_rng(rng);
    // initialize game engine, pointing it to write to stdout
    let writer: RefCell<Stdout> = RefCell::from(stdout);
    let mut engine = Engine::new(&writer);
    // start main game loop which draws -> reads input -> updates state
    loop {
        let span = span!(Level::INFO, "gameloop");
        let _enter = span.enter();
        info!("enter gameloop");
        match engine.draw_and_prompt(&mut game_state) {
            Err(e) => {
                // an error was encountered in main game loop
                error!("an error was encountered in the main game loop: {:?}", e);
                let msg = format!("{:?}", e);
                let lines = msg.split("\\n").collect::<Vec<&str>>();
                engine.exit_message(&lines)?;
                break;
            }
            Ok(signal) => {
                match signal {
                    UpdateSignal::Continue => {
                        // do nothing, loop again
                    }
                    UpdateSignal::Quit => {
                        // main loop told us user requested an exit
                        info!("should_exit indicated, exiting");
                        engine.exit_message(&["Thank you for playing!"])?;
                        break;
                    }
                    UpdateSignal::Restart => {
                        let rng = StdRng::from_entropy();
                        game_state = GameState::new_std_rng(rng);
                    }
                }
            }
        }
    }
    // set terminal back to canonical mode before exiting
    disable_raw_mode()
}
