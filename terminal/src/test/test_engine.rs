use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{cell::RefCell, str};

use crate::engine::{convert_key_event, UpdateSignal};
use crate::Engine;
use merchant_core::engine::UpdateResult;
use merchant_core::state::GameState;
use raw_format_ansi::raw_format_ansi;

pub struct TestEngine {
    writer_ref: RefCell<Vec<u8>>,
    game_state: GameState,
}

impl TestEngine {
    #[allow(unused_must_use)]
    pub fn from_game_state(mut game_state: GameState) -> UpdateResult<Self> {
        let writer = Vec::new();
        let writer_box: RefCell<Vec<u8>> = RefCell::from(writer);
        let mut engine = Engine::new(&writer_box);
        engine.render_scene_terminal(&mut game_state)?;
        Ok(Self {
            writer_ref: writer_box,
            game_state,
        })
    }

    pub fn get_current_formatted(&self) -> String {
        let buffer = String::from_utf8(self.writer_ref.borrow().clone()).unwrap();
        raw_format_ansi(&buffer)
    }

    pub fn expect(&self, expectation: &str) -> bool {
        let expectation = expectation.trim_matches('\n');
        let formatted = self.get_current_formatted();
        let result = formatted.contains(expectation);
        if !result {
            println!("----------------\n{}\n----------------", formatted);
        }
        result
    }

    pub fn nexpect(&self, expectation: &str) -> bool {
        let expectation = expectation.trim_matches('\n');
        let formatted = self.get_current_formatted();
        let result = !formatted.contains(expectation);
        if !result {
            println!("----------------\n{}\n----------------", formatted);
        }
        result
    }

    pub fn expect_full(&self, expectation: &str) -> String {
        let expectation = expectation.trim_matches('\n');
        let formatted = self.get_current_formatted();
        let result = formatted == *expectation;
        if !result {
            println!("----------------\n{}\n----------------", formatted);
        }
        expectation.to_string()
    }

    #[allow(unused_must_use)]
    pub fn key_event(&mut self, key_event: KeyEvent) -> UpdateResult<UpdateSignal> {
        self.writer_ref.borrow_mut().clear();
        let mut engine = Engine::new(&self.writer_ref);
        let update = engine.render_scene_terminal(&mut self.game_state)?;
        let signal = update(convert_key_event(key_event), &mut self.game_state)?;
        self.writer_ref.borrow_mut().clear();
        engine.render_scene_terminal(&mut self.game_state)?;
        Ok(signal)
    }

    pub fn charpress(&mut self, char: char) -> UpdateResult<UpdateSignal> {
        self.key_event(KeyEvent::new(KeyCode::Char(char), KeyModifiers::empty()))
    }

    pub fn enterpress(&mut self) -> UpdateResult<UpdateSignal> {
        self.key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()))
    }
}
