use std::str;
use terminal_commands::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    render_raw, Commands,
};

use crate::{
    engine::{render_scene, UpdateResult},
    state::GameState,
};

pub struct TestEngine {
    commands: Commands,
    game_state: GameState,
}

impl TestEngine {
    #[allow(unused_must_use)]
    pub fn from_game_state(game_state: GameState) -> UpdateResult<Self> {
        let (frame, _) = render_scene(&game_state).unwrap();
        Ok(Self {
            commands: frame,
            game_state,
        })
    }

    pub fn get_current_formatted(&self) -> String {
        render_raw(&self.commands).result
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
    pub fn keypress(&mut self, key_code: KeyCode) -> UpdateResult<()> {
        let (_, update) = render_scene(&self.game_state).unwrap();
        let signal = update(
            KeyEvent::new(key_code, KeyModifiers::empty()),
            &mut self.game_state,
        )?;
        let (frame, _) = render_scene(&self.game_state).unwrap();
        self.commands = frame;
        Ok(signal)
    }

    pub fn charpress(&mut self, char: char) -> UpdateResult<()> {
        self.keypress(KeyCode::Char(char))
    }

    pub fn enterpress(&mut self) -> UpdateResult<()> {
        self.keypress(KeyCode::Enter)
    }
}
