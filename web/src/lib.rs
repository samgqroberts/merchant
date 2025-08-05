use wasm_bindgen::prelude::*;
use web_sys::KeyboardEvent;
use merchant_core::state::GameState;
use merchant_core::engine::UpdateSignal;
use rand::{rngs::StdRng, SeedableRng};
use std::cell::RefCell;

mod html_renderer;
mod html_engine;

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
    // Check if terminal needs resizing
    let (needs_resize, width, height) = engine.check_terminal_size();
    
    if needs_resize {
        engine.draw_need_resize(width, height)?;
    } else {
        engine.draw_scene(state)?;
    }
    
    Ok(())
}

fn setup_keyboard_listener() -> Result<(), JsValue> {
    let window = web_sys::window().ok_or("no global window")?;
    let document = window.document().ok_or("no document")?;
    
    // Create a closure to handle keyboard events
    let closure = Closure::wrap(Box::new(move |event: KeyboardEvent| {
        // Process the key event
        let result = ENGINE.with(|engine_cell| {
            GAME_STATE.with(|state_cell| {
                let mut engine_opt = engine_cell.borrow_mut();
                let mut state_opt = state_cell.borrow_mut();
                
                if let (Some(engine), Some(state)) = (engine_opt.as_mut(), state_opt.as_mut()) {
                    match engine.handle_key_event(event, state) {
                        Ok(signal) => {
                            match signal {
                                UpdateSignal::Continue => {
                                    // Redraw the scene
                                    if let Err(e) = draw_game(engine, state) {
                                        log_error(&format!("Error drawing game: {:?}", e));
                                    }
                                }
                                UpdateSignal::Quit => {
                                    console_log!("Game quit requested");
                                    // Could show exit message
                                    if let Some(window) = web_sys::window() {
                                        if let Some(document) = window.document() {
                                            if let Some(pre) = document.get_element_by_id("game-display") {
                                                if let Ok(pre_element) = pre.dyn_into::<web_sys::HtmlPreElement>() {
                                                    pre_element.set_inner_html("Thank you for playing!");
                                                }
                                            }
                                        }
                                    }
                                }
                                UpdateSignal::Restart => {
                                    console_log!("Game restart requested");
                                    // Create new game state
                                    let rng = StdRng::from_entropy();
                                    *state = GameState::new_std_rng(rng);
                                    // Redraw
                                    if let Err(e) = draw_game(engine, state) {
                                        log_error(&format!("Error drawing game: {:?}", e));
                                    }
                                }
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
    document.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
    
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