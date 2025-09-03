use std::cell::RefCell;

use merchant_core::{
    components::ScreenCenteredText,
    engine::{render_scene, UpdateFn},
    state::GameState,
};
use terminal_commands::{comp, event::KeyEvent};
use wasm_bindgen::prelude::*;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{HtmlDivElement, KeyboardEvent, Window};

use crate::{
    html_renderer::{render_to_html, HtmlRenderOutput},
    log, log_error,
};

const MIN_FONT_SIZE: u16 = 6;
const MAX_FONT_SIZE: u16 = 16;
const DEFAULT_FONT_SIZE: u16 = 14;

pub struct HtmlEngine {
    window: RefCell<Window>,
    game_display_el: RefCell<HtmlDivElement>,
    font_size: RefCell<u16>,
    update_fn: Option<Box<UpdateFn>>,
    checked_window_size_after_initial_draw: bool,
}

impl HtmlEngine {
    pub fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("no global window")?;
        let document = window.document().ok_or("no document")?;
        let font_size = RefCell::new(DEFAULT_FONT_SIZE);

        // Find or create the pre element for displaying the game
        let game_display_el = match document.get_element_by_id("game-display") {
            Some(element) => element
                .dyn_into::<HtmlDivElement>()
                .map_err(|_| "element is not a pre element")?,
            None => {
                let game_display_el = document
                    .create_element("pre")?
                    .dyn_into::<HtmlDivElement>()?;
                game_display_el.set_id("game-display");
                game_display_el
                    .set_attribute("style", &format!("font-size: {}px", *font_size.borrow()))?;

                let body = document.body().ok_or("no body")?;
                body.append_child(&game_display_el)?;
                game_display_el
            }
        };
        let game_display_el = RefCell::new(game_display_el);
        let window = RefCell::new(window);

        // Set up window resize listener
        let window_clone = window.clone();
        let game_display_el_clone = game_display_el.clone();
        let font_size_clone = font_size.clone();
        let resize_callback = Closure::wrap(Box::new(move || {
            if let Err(e) = update_font_size(
                &*window_clone.borrow(),
                &*game_display_el_clone.borrow(),
                &mut *font_size_clone.borrow_mut(),
            ) {
                log_error(&format!("Error updating font size: {:?}", e));
            }
        }) as Box<dyn Fn()>);

        window
            .borrow()
            .add_event_listener_with_callback("resize", resize_callback.as_ref().unchecked_ref())?;

        // Keep the closure alive
        resize_callback.forget();

        Ok(Self {
            window,
            game_display_el,
            font_size,
            update_fn: None,
            checked_window_size_after_initial_draw: false,
        })
    }

    pub fn draw_scene(&mut self, state: &GameState) -> Result<(), JsValue> {
        let (render_result, update) = html_render_scene(state)
            .map_err(|e| JsValue::from_str(&format!("Error rendering scene: {:?}", e)))?;

        let inner_html = render_result.html;

        let game_display_el = self.game_display_el.borrow();

        // Convert the rendered text to HTML
        game_display_el.set_inner_html(&inner_html);

        if !self.checked_window_size_after_initial_draw {
            self.checked_window_size_after_initial_draw = true;
            update_font_size(
                &*self.window.borrow(),
                &*game_display_el,
                &mut *self.font_size.borrow_mut(),
            )?;
        }

        self.update_fn = Some(update);
        Ok(())
    }

    pub fn handle_key_event(
        &mut self,
        event: KeyboardEvent,
        game_state: &mut GameState,
    ) -> Result<(), JsValue> {
        // don't intervene if meta or ctrl key is pressed
        if event.meta_key() || event.ctrl_key() {
            return Ok(());
        }

        // at this point, we handle the keypress, so prevent default browser behavior
        event.prevent_default();

        // Convert web keyboard event to terminal_commands KeyEvent
        let key_event = convert_web_key_event(&event);

        // If we have an update function, call it
        if let Some(update_fn) = self.update_fn.take() {
            match update_fn(key_event, game_state) {
                Ok(signal) => Ok(signal),
                Err(e) => Err(JsValue::from_str(&format!("Update error: {:?}", e))),
            }
        } else {
            Ok(())
        }
    }
}

pub fn html_render_scene(state: &GameState) -> Result<(HtmlRenderOutput, Box<UpdateFn>), String> {
    let (mut commands, mut update) = render_scene(state)?;
    if state.game_end {
        comp!(
            commands,
            ScreenCenteredText::new(&["(Enter) to play again".to_owned()], 29),
        )?;
        update = Box::new(|event: KeyEvent, state: &mut GameState| match event.code {
            terminal_commands::event::KeyCode::Enter => {
                state.restart();
                Ok(())
            }
            _ => Ok(()),
        });
    }
    Ok((render_to_html(&commands), update))
}

pub fn convert_web_key_event(event: &KeyboardEvent) -> terminal_commands::event::KeyEvent {
    let key = event.key();

    let code = match key.as_str() {
        "Enter" => terminal_commands::event::KeyCode::Enter,
        "Backspace" => terminal_commands::event::KeyCode::Backspace,
        "Tab" => terminal_commands::event::KeyCode::Tab,
        "Escape" => terminal_commands::event::KeyCode::Esc,
        "ArrowUp" => terminal_commands::event::KeyCode::Up,
        "ArrowDown" => terminal_commands::event::KeyCode::Down,
        "ArrowLeft" => terminal_commands::event::KeyCode::Left,
        "ArrowRight" => terminal_commands::event::KeyCode::Right,
        "Home" => terminal_commands::event::KeyCode::Home,
        "End" => terminal_commands::event::KeyCode::End,
        "PageUp" => terminal_commands::event::KeyCode::PageUp,
        "PageDown" => terminal_commands::event::KeyCode::PageDown,
        "Delete" => terminal_commands::event::KeyCode::Delete,
        "Insert" => terminal_commands::event::KeyCode::Insert,
        s if s.len() == 1 => {
            let ch = s.chars().next().unwrap();
            terminal_commands::event::KeyCode::Char(ch)
        }
        s if s.starts_with("F") && s.len() <= 3 => {
            if let Ok(num) = s[1..].parse::<u8>() {
                terminal_commands::event::KeyCode::F(num)
            } else {
                terminal_commands::event::KeyCode::Null
            }
        }
        _ => terminal_commands::event::KeyCode::Null,
    };

    let mut modifiers = terminal_commands::event::KeyModifiers::empty();
    if event.shift_key() {
        modifiers.insert(terminal_commands::event::KeyModifiers::SHIFT);
    }
    if event.ctrl_key() {
        modifiers.insert(terminal_commands::event::KeyModifiers::CONTROL);
    }
    if event.alt_key() {
        modifiers.insert(terminal_commands::event::KeyModifiers::ALT);
    }

    terminal_commands::event::KeyEvent::new(code, modifiers)
}

pub fn update_font_size(
    window: impl AsRef<Window>,
    game_display_el: impl AsRef<HtmlDivElement>,
    font_size: &mut u16,
) -> Result<(), JsValue> {
    let window = window.as_ref();
    let game_display_el = game_display_el.as_ref();
    let mut tried_down = false;
    loop {
        let game_display_width = game_display_el.client_width() as f64 + 20f64 /* padding */;
        let window_display_width: f64 = window
            .inner_width()?
            .as_f64()
            .ok_or(JsValue::from_str("Failed to get window width"))?;
        if game_display_width > window_display_width {
            if *font_size <= MIN_FONT_SIZE {
                break;
            }
            tried_down = true;
            *font_size -= 1;
            log(&format!("Decreasing font size to {}", font_size));
            game_display_el.set_attribute("style", &format!("font-size: {}px", font_size))?;
        } else if (game_display_width) < window_display_width {
            if tried_down || *font_size >= MAX_FONT_SIZE {
                break;
            }
            *font_size += 1;
            log(&format!("Increasing font size to {}", font_size));
            game_display_el.set_attribute("style", &format!("font-size: {}px", font_size))?;
        } else {
            break;
        }
    }
    Ok(())
}
