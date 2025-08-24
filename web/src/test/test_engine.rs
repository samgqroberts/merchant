use std::str;
use terminal_commands::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::html_engine::html_render_scene;
use crate::html_renderer::HtmlRenderOutput;
use crate::test::raw_parse_html::parse_html_to_raw_text;
use merchant_core::engine::{UpdateFn, UpdateResult};
use merchant_core::state::GameState;

pub struct TestEngine {
    render_result: HtmlRenderOutput,
    game_state: GameState,
    update: Box<UpdateFn>,
}

impl TestEngine {
    #[allow(unused_must_use)]
    pub fn from_game_state(game_state: GameState) -> UpdateResult<Self> {
        let (render_result, update) = html_render_scene(&game_state).unwrap();
        Ok(Self {
            render_result,
            game_state,
            update,
        })
    }

    pub fn get_current_formatted(&self) -> String {
        parse_html_to_raw_text(&self.render_result.html)
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

    #[allow(unused_must_use)]
    pub fn keypress(&mut self, key_code: KeyCode) -> UpdateResult<()> {
        // replace this engine's update with a dummy update function
        // so we can use (consume) the actual update function
        let update = std::mem::replace(&mut self.update, Box::new(|_, _| Ok(())));
        let _ = update(
            KeyEvent::new(key_code, KeyModifiers::empty()),
            &mut self.game_state,
        )
        .unwrap();
        let (render_result, update) = html_render_scene(&self.game_state).unwrap();
        self.render_result = render_result;
        self.update = update;
        Ok(())
    }

    pub fn charpress(&mut self, char: char) -> UpdateResult<()> {
        self.keypress(KeyCode::Char(char))
    }

    pub fn enterpress(&mut self) -> UpdateResult<()> {
        self.keypress(KeyCode::Enter)
    }
}
