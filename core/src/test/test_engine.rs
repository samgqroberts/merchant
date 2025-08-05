use ansi_commands::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    frame::Frame,
};
use std::str;

use crate::{
    engine::{render_scene, UpdateResult, UpdateSignal},
    state::GameState,
};

pub struct TestEngine {
    frame: Frame,
    game_state: GameState,
}

impl TestEngine {
    #[allow(unused_must_use)]
    pub fn from_game_state(mut game_state: GameState) -> UpdateResult<Self> {
        let (frame, _) = render_scene(&mut game_state).unwrap();
        Ok(Self { frame, game_state })
    }

    pub fn get_current_formatted(&self) -> String {
        self.frame.render_raw().result
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
    pub fn keypress(&mut self, key_code: KeyCode) -> UpdateResult<UpdateSignal> {
        let (_, update) = render_scene(&mut self.game_state).unwrap();
        let signal = update(
            KeyEvent::new(key_code, KeyModifiers::empty()),
            &mut self.game_state,
        )?;
        let (frame, _) = render_scene(&mut self.game_state).unwrap();
        self.frame = frame;
        Ok(signal)
    }

    pub fn charpress(&mut self, char: char) -> UpdateResult<UpdateSignal> {
        self.keypress(KeyCode::Char(char))
    }

    pub fn enterpress(&mut self) -> UpdateResult<UpdateSignal> {
        self.keypress(KeyCode::Enter)
    }
}
