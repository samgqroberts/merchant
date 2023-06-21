mod draw;
mod state;
mod update;

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
use update::update;

pub struct Engine<'a, Writer: Write> {
    drawer: Drawer<'a, Writer>,
}

impl<'a, Writer: Write> Engine<'a, Writer> {
    pub fn new(writer: &'a mut Box<Writer>) -> Self {
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
    ) -> io::Result<(bool, Option<GameState>)> {
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

    pub fn exit_message(&mut self) -> io::Result<()> {
        self.drawer.exit_message()
    }
}

fn main() -> io::Result<()> {
    let stdout = stdout();
    // set terminal into "non-canonical" mode so inputs are captured raw with no interpretation
    // https://docs.rs/crossterm/0.26.1/crossterm/terminal/index.html#raw-mode
    enable_raw_mode()?;
    // start main game loop, which draws -> reads input -> updates state, with fresh game state
    let mut game_state = GameState::new();
    let mut writer: Box<Stdout> = Box::from(stdout);
    let mut engine = Engine::new(&mut writer);
    loop {
        match engine.draw_and_prompt(&mut game_state) {
            Err(e) => {
                // an io error was encountered in main game loop
                println!("Error: {:?}\r", e);
            }
            Ok((should_exit, new_game_state)) => {
                if should_exit {
                    // main loop told us user requested an exit
                    engine.exit_message()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::str;
    use strip_ansi_escapes::strip;

    // Helper for execute tests to confirm flush
    #[derive(Default, Debug, Clone)]
    pub(self) struct FakeWrite {
        pub buffer: String,
        pub flushed: bool,
    }

    impl FakeWrite {
        fn new() -> Self {
            Self {
                buffer: "".to_owned(),
                flushed: false,
            }
        }
    }

    impl io::Write for FakeWrite {
        fn write(&mut self, content: &[u8]) -> io::Result<usize> {
            let content = str::from_utf8(content)
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            self.buffer.push_str(content);
            self.flushed = false;
            Ok(content.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            self.flushed = true;
            Ok(())
        }
    }

    #[test]
    fn thing() -> io::Result<()> {
        let game_state = GameState::new();
        let writer = FakeWrite::new();
        let mut writer_box: Box<FakeWrite> = Box::from(writer);
        let mut engine = Engine::new(&mut writer_box);
        engine.draw(&game_state)?;
        assert_eq!(
            str::from_utf8(&strip(writer_box.buffer).unwrap()).unwrap(),
            "MerchantNavigate shifting markets and unreliable sources.By samgqrobertsPress any key to begin".to_owned()
        );
        Ok(())
    }
}
