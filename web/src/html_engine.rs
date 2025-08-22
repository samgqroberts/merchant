use merchant_core::{
    components::{RequireResize, FRAME_HEIGHT, FRAME_WIDTH},
    engine::{render_scene, UpdateFn, UpdateSignal},
    state::GameState,
};
use terminal_commands::frame::Renderer;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlDivElement, KeyboardEvent, Window};

use crate::html_renderer::HtmlRenderer;

pub struct HtmlEngine {
    window: Window,
    container_element: HtmlDivElement,
    update_fn: Option<Box<UpdateFn>>,
}

impl HtmlEngine {
    pub fn new() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("no global window")?;
        let document = window.document().ok_or("no document")?;

        // Find or create the pre element for displaying the game
        let pre_element = match document.get_element_by_id("game-display") {
            Some(element) => element
                .dyn_into::<HtmlDivElement>()
                .map_err(|_| "element is not a pre element")?,
            None => {
                let pre = document
                    .create_element("pre")?
                    .dyn_into::<HtmlDivElement>()?;
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
            container_element: pre_element,
            update_fn: None,
        })
    }

    pub fn draw_scene(&mut self, state: &mut GameState) -> Result<(), JsValue> {
        let (frame, update) = render_scene(state)
            .map_err(|e| JsValue::from_str(&format!("Error rendering scene: {:?}", e)))?;

        let render_result = HtmlRenderer.render(&frame).unwrap() /* todo */;

        let mut inner_html = render_result.result;

        if render_result.show_cursor {
            let height = self.container_element.client_height();
            let width = self.container_element.client_width();
            let char_height = height / FRAME_HEIGHT as i32;
            let char_width = width / FRAME_WIDTH as i32;
            let (cursor_x, cursor_y) = render_result.cursor;
            inner_html.push_str(&format!(
                "<div id=\"cursor\" style=\"top: {}px; left: {}px;\"></div>",
                cursor_y as i32 * char_height - 4,
                (cursor_x as i32 + 2) * char_width
            ));
        }

        // Convert the rendered text to HTML
        self.container_element.set_inner_html(&inner_html);

        self.update_fn = Some(update);
        Ok(())
    }

    pub fn draw_need_resize(
        &mut self,
        current_width: u16,
        current_height: u16,
    ) -> Result<(), JsValue> {
        let mut frame = terminal_commands::frame::Frame::new();
        frame
            .render(&RequireResize {
                current_x_cols: current_width,
                current_y_cols: current_height,
            })
            .map_err(|e| JsValue::from_str(&format!("Error rendering resize: {:?}", e)))?;

        let render_result = HtmlRenderer.render(&frame);
        self.container_element.set_inner_html(
            &render_result
                .unwrap() /* todo */
                .result,
        );

        Ok(())
    }

    pub fn handle_key_event(
        &mut self,
        event: KeyboardEvent,
        game_state: &mut GameState,
    ) -> Result<UpdateSignal, JsValue> {
        // first, see if use wants to quit (restart)
        if event.ctrl_key() && event.key() == "c" {
            return Ok(UpdateSignal::Quit);
        }

        // otherwise, don't intervene if meta or ctrl key is pressed
        if event.meta_key() || event.ctrl_key() {
            return Ok(UpdateSignal::Continue);
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

fn convert_web_key_event(event: &KeyboardEvent) -> terminal_commands::event::KeyEvent {
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
