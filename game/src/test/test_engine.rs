use captured_write::CapturedWrite;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::{rngs::StdRng, SeedableRng};
use std::{cell::RefCell, str};

use crate::{
    engine::{Engine, UpdateResult},
    state::GameState,
};
use raw_format_ansi::raw_format_ansi;

pub struct TestEngine {
    writer_ref: RefCell<CapturedWrite>,
    game_state: GameState,
}

impl TestEngine {
    pub fn new() -> UpdateResult<Self> {
        let rng = StdRng::seed_from_u64(42);
        let game_state = GameState::new(rng);
        Self::from_game_state(game_state)
    }

    #[allow(unused_must_use)]
    pub fn from_game_state(game_state: GameState) -> UpdateResult<Self> {
        let writer = CapturedWrite::new();
        let writer_box: RefCell<CapturedWrite> = RefCell::from(writer);
        let mut engine = Engine::new(&writer_box);
        engine.draw_scene(&game_state)?;
        Ok(Self {
            writer_ref: writer_box,
            game_state,
        })
    }

    pub fn expect(&self, expectation: &str) -> UpdateResult<()> {
        let buffer = self.writer_ref.borrow().buffer.clone();
        let formatted = raw_format_ansi(&buffer);
        if formatted != expectation.to_owned() {
            println!("{}", formatted);
            println!("{}", expectation);
        }
        assert_eq!(formatted, expectation.to_owned());
        Ok(())
    }

    #[allow(unused_must_use)]
    pub fn keypress(&mut self, key_code: KeyCode) -> UpdateResult<()> {
        self.writer_ref.borrow_mut().reset();
        let mut engine = Engine::new(&self.writer_ref);
        let update = engine.draw_scene(&self.game_state)?;
        self.game_state = update(
            KeyEvent::new(key_code, KeyModifiers::empty()),
            &self.game_state,
        )?
        .unwrap();
        self.writer_ref.borrow_mut().reset();
        engine.draw_scene(&self.game_state)?;
        Ok(())
    }

    pub fn charpress(&mut self, char: char) -> UpdateResult<()> {
        self.keypress(KeyCode::Char(char))
    }

    pub fn enterpress(&mut self) -> UpdateResult<()> {
        self.keypress(KeyCode::Enter)
    }
}