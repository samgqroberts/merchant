mod engine;
mod state;
#[cfg(test)]
mod test;

use engine::Engine;
use rand::{rngs::StdRng, SeedableRng};
use std::cell::RefCell;
use std::io::Stdout;
use std::io::{self, stdout};

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use state::GameState;

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
