use ansi_commands::frame::{RawRenderer, Renderer};
use merchant_core::{
    components::{RequireResize, FRAME_HEIGHT, FRAME_WIDTH},
    engine::{render_scene, UpdateFn, UpdateSignal},
    state::GameState,
};
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlPreElement, KeyboardEvent, Window};

use crate::{html_renderer::HtmlRenderer, log};

pub struct HtmlEngine {
    window: Window,
    document: Document,
    pre_element: HtmlPreElement,
    update_fn: Option<Box<UpdateFn>>,
}

impl HtmlEngine {
    pub fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("no global window")?;
        let document = window.document().ok_or("no document")?;

        // Find or create the pre element for displaying the game
        let pre_element = match document.get_element_by_id("game-display") {
            Some(element) => element
                .dyn_into::<HtmlPreElement>()
                .map_err(|_| "element is not a pre element")?,
            None => {
                let pre = document
                    .create_element("pre")?
                    .dyn_into::<HtmlPreElement>()?;
                pre.set_id("game-display");

                // Set monospace font and styling
                let style = pre.style();
                style.set_property("font-family", "monospace")?;
                style.set_property("font-size", "14px")?;
                style.set_property("line-height", "1.2")?;
                style.set_property("background-color", "black")?;
                style.set_property("color", "white")?;
                style.set_property("padding", "10px")?;
                style.set_property("margin", "0")?;
                style.set_property("white-space", "pre")?;

                let body = document.body().ok_or("no body")?;
                body.append_child(&pre)?;
                pre
            }
        };

        Ok(Self {
            window,
            document,
            pre_element,
            update_fn: None,
        })
    }

    pub fn draw_scene(&mut self, state: &mut GameState) -> Result<(), JsValue> {
        let (frame, update) = render_scene(state)
            .map_err(|e| JsValue::from_str(&format!("Error rendering scene: {:?}", e)))?;

        let render_result = RawRenderer.render(&frame);
        // let render_result = HtmlRenderer.render(&frame); TODO: use this

        // Convert the rendered text to HTML
        self.pre_element.set_inner_html(&render_result.result);

        self.update_fn = Some(update);
        Ok(())
    }

    pub fn draw_need_resize(
        &mut self,
        current_width: u16,
        current_height: u16,
    ) -> Result<(), JsValue> {
        let mut frame = ansi_commands::frame::Frame::new();
        frame
            .render(&RequireResize {
                current_x_cols: current_width,
                current_y_cols: current_height,
            })
            .map_err(|e| JsValue::from_str(&format!("Error rendering resize: {:?}", e)))?;

        let render_result = HtmlRenderer.render(&frame);
        self.pre_element.set_inner_html(&render_result.result);

        Ok(())
    }

    pub fn handle_key_event(
        &mut self,
        event: KeyboardEvent,
        game_state: &mut GameState,
    ) -> Result<UpdateSignal, JsValue> {
        // Prevent default browser behavior
        event.prevent_default();

        // Convert web keyboard event to ansi_commands KeyEvent
        let key_event = convert_web_key_event(&event);

        // Check for quit command (Ctrl+C)
        if event.ctrl_key() && event.key() == "c" {
            return Ok(UpdateSignal::Quit);
        }

        // If we have an update function, call it
        if let Some(update_fn) = self.update_fn.take() {
            match update_fn(key_event, game_state) {
                Ok(signal) => Ok(signal),
                Err(e) => Err(JsValue::from_str(&format!("Update error: {:?}", e))),
            }
        } else {
            Ok(UpdateSignal::Continue)
        }
    }

    pub fn check_terminal_size(&self) -> (bool, u16, u16) {
        // For web, we can estimate based on viewport size
        // Assuming each character is about 8px wide and 16px tall
        let width = (self
            .window
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(800.0)
            / 8.0) as u16;
        let height = (self
            .window
            .inner_height()
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(600.0)
            / 16.0) as u16;

        let needs_resize = width < FRAME_WIDTH || height < FRAME_HEIGHT;
        (needs_resize, width, height)
    }
}

fn convert_web_key_event(event: &KeyboardEvent) -> ansi_commands::event::KeyEvent {
    let key = event.key();

    let code = match key.as_str() {
        "Enter" => ansi_commands::event::KeyCode::Enter,
        "Backspace" => ansi_commands::event::KeyCode::Backspace,
        "Tab" => ansi_commands::event::KeyCode::Tab,
        "Escape" => ansi_commands::event::KeyCode::Esc,
        "ArrowUp" => ansi_commands::event::KeyCode::Up,
        "ArrowDown" => ansi_commands::event::KeyCode::Down,
        "ArrowLeft" => ansi_commands::event::KeyCode::Left,
        "ArrowRight" => ansi_commands::event::KeyCode::Right,
        "Home" => ansi_commands::event::KeyCode::Home,
        "End" => ansi_commands::event::KeyCode::End,
        "PageUp" => ansi_commands::event::KeyCode::PageUp,
        "PageDown" => ansi_commands::event::KeyCode::PageDown,
        "Delete" => ansi_commands::event::KeyCode::Delete,
        "Insert" => ansi_commands::event::KeyCode::Insert,
        s if s.len() == 1 => {
            let ch = s.chars().next().unwrap();
            ansi_commands::event::KeyCode::Char(ch)
        }
        s if s.starts_with("F") && s.len() <= 3 => {
            if let Ok(num) = s[1..].parse::<u8>() {
                ansi_commands::event::KeyCode::F(num)
            } else {
                ansi_commands::event::KeyCode::Null
            }
        }
        _ => ansi_commands::event::KeyCode::Null,
    };

    let mut modifiers = ansi_commands::event::KeyModifiers::empty();
    if event.shift_key() {
        modifiers.insert(ansi_commands::event::KeyModifiers::SHIFT);
    }
    if event.ctrl_key() {
        modifiers.insert(ansi_commands::event::KeyModifiers::CONTROL);
    }
    if event.alt_key() {
        modifiers.insert(ansi_commands::event::KeyModifiers::ALT);
    }

    ansi_commands::event::KeyEvent::new(code, modifiers)
}
