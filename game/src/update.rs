use std::io;

use crossterm::event::{KeyCode, KeyEvent};

use crate::state::{GameState, GoodType, Location, Mode, StateError};

#[derive(Debug)]
pub struct UpdateError(String);

impl From<io::Error> for UpdateError {
    fn from(value: io::Error) -> Self {
        Self(value.to_string())
    }
}

impl<'a> From<StateError<'a>> for UpdateError {
    fn from(value: StateError) -> Self {
        Self(value.to_string())
    }
}

pub type UpdateResult<T> = Result<T, UpdateError>;

trait FromKeyCode
where
    Self: Sized,
{
    fn from_key_code(key_code: &KeyCode) -> Option<Self>;
}

impl FromKeyCode for GoodType {
    fn from_key_code(key_code: &KeyCode) -> Option<Self> {
        if let KeyCode::Char(c) = key_code {
            match c {
                '1' => Some(GoodType::Sugar),
                '2' => Some(GoodType::Tobacco),
                '3' => Some(GoodType::Tea),
                '4' => Some(GoodType::Cotton),
                '5' => Some(GoodType::Rum),
                '6' => Some(GoodType::Coffee),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl FromKeyCode for Location {
    fn from_key_code(key_code: &KeyCode) -> Option<Self> {
        if let KeyCode::Char(c) = key_code {
            match c {
                '1' => Some(Location::Savannah),
                '2' => Some(Location::London),
                '3' => Some(Location::Lisbon),
                '4' => Some(Location::Amsterdam),
                '5' => Some(Location::CapeTown),
                '6' => Some(Location::Venice),
                _ => None,
            }
        } else {
            None
        }
    }
}

pub fn update(event: KeyEvent, game_state: &GameState) -> UpdateResult<Option<GameState>> {
    // any key event initializes the game if game is not already initialized
    if !game_state.initialized {
        return Ok(Some(game_state.initialize()));
    }
    // other updates depend on what the viewer is seeing currently
    match &game_state.mode {
        Mode::ViewingInventory => {
            if let KeyCode::Char(ch) = event.code {
                match ch {
                    '1' => {
                        return Ok(Some(game_state.begin_buying()?));
                    }
                    '2' => {
                        return Ok(Some(game_state.begin_selling()?));
                    }
                    '3' => {
                        return Ok(Some(game_state.begin_sailing()?));
                    }
                    _ => {
                        // any other character has no effect
                    }
                }
            }
        }
        Mode::Buying(info) => {
            if info.is_some() {
                // user has chosen a good to buy
                if let KeyCode::Char(c) = event.code {
                    if let Some(digit) = c.to_digit(10) {
                        return Ok(Some(game_state.user_typed_digit(digit)?));
                    }
                }
                if event.code == KeyCode::Backspace {
                    return Ok(Some(game_state.user_typed_backspace()?));
                }
                if event.code == KeyCode::Enter {
                    return match game_state.commit_buy() {
                        Ok(new_state) => Ok(Some(new_state)),
                        Err(variant) => match variant {
                            StateError::CannotAfford | StateError::InsufficientHold => Ok(None),
                            x => Err(x.into()),
                        },
                    };
                }
            } else {
                if let Some(good) = GoodType::from_key_code(&event.code) {
                    return Ok(Some(game_state.choose_buy_good(good)?));
                }
            }
        }
        Mode::Selling(info) => {
            if info.is_some() {
                // user has chosen a good to sell
                if let KeyCode::Char(c) = event.code {
                    if let Some(digit) = c.to_digit(10) {
                        return Ok(Some(game_state.user_typed_digit(digit)?));
                    }
                }
                if event.code == KeyCode::Backspace {
                    return Ok(Some(game_state.user_typed_backspace()?));
                }
                if event.code == KeyCode::Enter {
                    return match game_state.commit_sell() {
                        Ok(new_state) => Ok(Some(new_state)),
                        Err(variant) => match variant {
                            StateError::InsufficientInventory => Ok(None),
                            x => Err(x.into()),
                        },
                    };
                }
            } else {
                if let Some(good) = GoodType::from_key_code(&event.code) {
                    return Ok(Some(game_state.choose_sell_good(good)?));
                }
            }
        }
        Mode::Sailing => {
            if let Some(destination) = Location::from_key_code(&event.code) {
                return match game_state.relocate(&destination) {
                    Ok(new_state) => Ok(Some(new_state)),
                    Err(variant) => match variant {
                        StateError::AlreadyInLocation => Ok(None),
                        x => Err(x.into()),
                    },
                };
            }
        }
    }
    Ok(None)
}
