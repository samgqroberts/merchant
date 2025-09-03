use merchant_core::state::GameState;
use rand::{rngs::StdRng, SeedableRng};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, InputEvent, KeyboardEvent, KeyboardEventInit, MouseEvent};

#[cfg(test)]
mod test;

mod html_engine;
mod html_renderer;

use html_engine::HtmlEngine;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = "error")]
    fn log_error(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

thread_local! {
    static GAME_STATE: RefCell<Option<GameState>> = RefCell::new(None);
    static ENGINE: RefCell<Option<HtmlEngine>> = RefCell::new(None);
}

/// The entry point for the web application.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Set panic hook for better error messages
    console_error_panic_hook::set_once();

    console_log!("Merchant Web starting...");

    // Initialize game state with RNG
    let rng = StdRng::from_entropy();
    let game_state = GameState::new_std_rng(rng);

    // Initialize HTML engine
    let mut engine = HtmlEngine::new()?;

    // Initial draw
    let mut state = game_state;
    draw_game(&mut engine, &mut state)?;

    // Store game state and engine
    GAME_STATE.with(|gs| {
        *gs.borrow_mut() = Some(state);
    });
    ENGINE.with(|e| {
        *e.borrow_mut() = Some(engine);
    });

    // Set up keyboard event listener
    setup_keyboard_listener()?;

    console_log!("Merchant Web initialized successfully");
    Ok(())
}

fn draw_game(engine: &mut HtmlEngine, state: &mut GameState) -> Result<(), JsValue> {
    engine.draw_scene(state)
}

fn setup_keyboard_listener() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no global window")?;
    let document = window.document().ok_or("no document")?;

    // Get the mobile keyboard trigger input
    let mobile_input = document
        .get_element_by_id("mobile-keyboard-trigger")
        .and_then(|el| el.dyn_into::<HtmlInputElement>().ok());

    // Check if we're on a mobile device (rough detection)
    let is_mobile = window
        .navigator()
        .user_agent()
        .map(|ua| {
            ua.contains("Mobile")
                || ua.contains("Android")
                || ua.contains("iPhone")
                || ua.contains("iPad")
                || ua.contains("iPod")
        })
        .unwrap_or(false);

    // If mobile and input exists, set up mobile-specific handling
    if is_mobile {
        if let Some(input) = mobile_input.as_ref() {
            console_log!("Mobile device detected, setting up touch handler");

            // Focus the input initially to show keyboard
            let _ = input.focus();

            // Add click listener to game display to refocus input
            if let Some(game_display) = document.get_element_by_id("game-display") {
                let input_clone = input.clone();
                let click_closure = Closure::wrap(Box::new(move |_: MouseEvent| {
                    let _ = input_clone.focus();
                }) as Box<dyn FnMut(MouseEvent)>);

                game_display.add_event_listener_with_callback(
                    "click",
                    click_closure.as_ref().unchecked_ref(),
                )?;
                click_closure.forget();
            }
        }
    }

    // Create a closure to handle keyboard events
    let mobile_input_clone = mobile_input.clone();
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        // Clear the mobile input field after each keystroke
        if let Some(input) = mobile_input_clone.as_ref() {
            input.set_value("");
        }

        // Process the key event
        let result = ENGINE.with(|engine_cell| {
            GAME_STATE.with(|state_cell| {
                let mut engine_opt = engine_cell.borrow_mut();
                let mut state_opt = state_cell.borrow_mut();

                if let (Some(engine), Some(state)) = (engine_opt.as_mut(), state_opt.as_mut()) {
                    match engine.handle_key_event(event, state) {
                        Ok(()) => {
                            // Redraw the scene
                            if let Err(e) = draw_game(engine, state) {
                                log_error(&format!("Error drawing game: {:?}", e));
                            }
                        }
                        Err(e) => {
                            log_error(&format!("Error handling key event: {:?}", e));
                        }
                    }
                } else {
                    log_error("Game state or engine not initialized");
                }
            })
        });

        result
    }) as Box<dyn FnMut(KeyboardEvent)>);

    // Add the event listener to the document
    if !is_mobile {
        document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    }

    // Also add listener to the mobile input if it exists
    if let Some(input) = mobile_input.as_ref() {
        // Add input event listener for mobile keyboards that don't fire keydown
        let input_clone = input.clone();
        let input_event_closure = Closure::wrap(Box::new(move |_: InputEvent| {
            // Get the last character typed
            let value = input_clone.value();
            if !value.is_empty() {
                // Create a synthetic keyboard event for the last character
                if let Some(last_char) = value.chars().last() {
                    // Create synthetic event
                    let event_init = KeyboardEventInit::new();
                    event_init.set_key(&last_char.to_string());

                    let synthetic_event = web_sys::KeyboardEvent::new_with_keyboard_event_init_dict(
                        "keydown",
                        &event_init,
                    );

                    if let Ok(event) = synthetic_event {
                        ENGINE.with(|engine_cell| {
                            GAME_STATE.with(|state_cell| {
                                let mut engine_opt = engine_cell.borrow_mut();
                                let mut state_opt = state_cell.borrow_mut();

                                if let (Some(engine), Some(state)) =
                                    (engine_opt.as_mut(), state_opt.as_mut())
                                {
                                    match engine.handle_key_event(event, state) {
                                        Ok(()) => {
                                            if let Err(e) = draw_game(engine, state) {
                                                log_error(&format!("Error drawing game: {:?}", e));
                                            }
                                        }
                                        Err(e) => {
                                            log_error(&format!(
                                                "Error handling input event: {:?}",
                                                e
                                            ));
                                        }
                                    }
                                }
                            })
                        });
                    }

                    // Clear the input
                    input_clone.set_value("");
                }
            }
        }) as Box<dyn FnMut(web_sys::InputEvent)>);

        input.add_event_listener_with_callback(
            "input",
            input_event_closure.as_ref().unchecked_ref(),
        )?;
        input_event_closure.forget();
    }

    // Keep the closure alive
    closure.forget();

    Ok(())
}

/// Reset the game to a new state
#[wasm_bindgen]
pub fn reset_game() -> Result<(), JsValue> {
    GAME_STATE.with(|state_cell| {
        ENGINE.with(|engine_cell| {
            let mut state_opt = state_cell.borrow_mut();
            let mut engine_opt = engine_cell.borrow_mut();

            if let (Some(state), Some(engine)) = (state_opt.as_mut(), engine_opt.as_mut()) {
                // Create new game state
                let rng = StdRng::from_entropy();
                *state = GameState::new_std_rng(rng);

                // Redraw
                draw_game(engine, state)?;
            }
            Ok(())
        })
    })
}
