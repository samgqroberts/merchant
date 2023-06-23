use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::{rngs::StdRng, SeedableRng};
use std::{cell::RefCell, io, str};

use crate::{state::GameState, test::raw_format_ansi::raw_format_ansi, update::update, Engine};

use super::captured_write::CapturedWrite;

pub struct TestEngine {
    writer_ref: RefCell<CapturedWrite>,
    game_state: GameState,
}

impl TestEngine {
    pub fn new() -> io::Result<Self> {
        let rng = StdRng::seed_from_u64(42);
        let game_state = GameState::new(rng);
        Self::from_game_state(game_state)
    }

    pub fn from_game_state(game_state: GameState) -> io::Result<Self> {
        let writer = CapturedWrite::new();
        let writer_box: RefCell<CapturedWrite> = RefCell::from(writer);
        let mut engine = Engine::new(&writer_box);
        engine.draw(&game_state)?;
        Ok(Self {
            writer_ref: writer_box,
            game_state,
        })
    }

    pub fn expect(&self, expectation: &str) -> io::Result<()> {
        let buffer = self.writer_ref.borrow().buffer.clone();
        let formatted = raw_format_ansi(&buffer);
        if formatted != expectation.to_owned() {
            println!("{}", formatted);
            println!("{}", expectation);
        }
        assert_eq!(formatted, expectation.to_owned());
        Ok(())
    }

    pub fn keypress(&mut self, key_code: KeyCode) -> io::Result<()> {
        self.writer_ref.borrow_mut().reset();
        self.game_state = update(
            KeyEvent::new(key_code, KeyModifiers::empty()),
            &self.game_state,
        )?
        .unwrap();
        let mut engine = Engine::new(&self.writer_ref);
        engine.draw(&self.game_state)?;
        Ok(())
    }

    pub fn charpress(&mut self, char: char) -> io::Result<()> {
        self.keypress(KeyCode::Char(char))
    }

    pub fn enterpress(&mut self) -> io::Result<()> {
        self.keypress(KeyCode::Enter)
    }
}
