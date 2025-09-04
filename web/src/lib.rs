use merchant_core::state::GameState;
use rand::{rngs::StdRng, SeedableRng};
use std::cell::RefCell;
use std::rc::Rc;
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

    // Track the last processed keydown to avoid double-processing
    // Keydown events and input events can both trigger on mobile devices
    // We store the key and a timestamp to ensure we can distinguish between repeated keys
    let last_keydown: Rc<RefCell<Option<(String, f64)>>> = Rc::new(RefCell::new(None));

    // Create a closure to handle keyboard events
    let mobile_input_clone = mobile_input.clone();
    let last_keydown_clone = last_keydown.clone();
    // Remove window_for_timestamp - we'll use js_sys::Date instead
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        event.stop_immediate_propagation();

        // Check if this is a valid key (not "Unidentified")
        let is_valid_key = event.key() != "Unidentified";

        // Mobile keyboards sometimes pass Unidentified as the event key for ASCII characters.
        if !is_valid_key {
            log("Skipping Unidentified key in keydown handler");
            return;
        }

        // Record this keydown
        let timestamp = js_sys::Date::now();
        *last_keydown_clone.borrow_mut() = Some((event.key(), timestamp));

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
        input.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;

        // Add input event listener for mobile keyboards that don't fire keydown
        let input_clone = input.clone();
        let last_keydown_clone2 = last_keydown.clone();
        // Remove window_for_input_timestamp - we'll use js_sys::Date instead
        let input_event_closure = Closure::wrap(Box::new(move |event: InputEvent| {
            event.stop_immediate_propagation();

            // Check if we recently processed a keydown (within 50ms)
            let current_time = js_sys::Date::now();

            let should_skip = if let Some((_, keydown_time)) = *last_keydown_clone2.borrow() {
                let time_diff = current_time - keydown_time;
                // If keydown was within 50ms, assume this input is for the same key
                time_diff < 50.0
            } else {
                false
            };

            if should_skip {
                log("Input already processed via recent keydown, skipping");
                // Clear the stored keydown since we've handled the corresponding input
                *last_keydown_clone2.borrow_mut() = None;
                input_clone.set_value(""); // Clear input
                return;
            }

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
