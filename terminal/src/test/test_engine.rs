use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{cell::RefCell, str};

use crate::engine::{convert_key_event, render_scene_to_writer, TerminalUpdateFn, UpdateSignal};
use merchant_core::engine::UpdateResult;
use merchant_core::state::GameState;
use raw_format_ansi::raw_format_ansi;

pub struct TestEngine {
    writer_ref: RefCell<Vec<u8>>,
    game_state: GameState,
    update: Box<TerminalUpdateFn>,
}

impl TestEngine {
    pub fn from_game_state(game_state: GameState) -> UpdateResult<Self> {
        let writer = Vec::new();
        let writer_box: RefCell<Vec<u8>> = RefCell::from(writer);
        let update = render_scene_to_writer(&mut *writer_box.borrow_mut(), &game_state).unwrap();
        Ok(Self {
            writer_ref: writer_box,
            game_state,
            update,
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

    pub fn expect_full(&self, expectation: &str) -> String {
        let expectation = expectation.trim_matches('\n');
        let formatted = self.get_current_formatted();
        let result = formatted == *expectation;
        if !result {
            println!("----------------\n{}\n----------------", formatted);
        }
        expectation.to_string()
    }

    pub fn key_event(&mut self, key_event: KeyEvent) -> UpdateResult<UpdateSignal> {
        self.writer_ref.borrow_mut().clear();
        let update = std::mem::replace(
            &mut self.update,
            Box::new(|_, _| Ok(UpdateSignal::Continue)),
        );
        let signal = update(convert_key_event(key_event), &mut self.game_state).unwrap();
        self.update =
            render_scene_to_writer(&mut *self.writer_ref.borrow_mut(), &self.game_state).unwrap();
        Ok(signal)
    }

    pub fn charpress(&mut self, char: char) -> UpdateResult<UpdateSignal> {
        self.key_event(KeyEvent::new(KeyCode::Char(char), KeyModifiers::empty()))
    }

    pub fn enterpress(&mut self) -> UpdateResult<UpdateSignal> {
        self.key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()))
    }
}
