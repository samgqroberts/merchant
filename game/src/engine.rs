use crossterm::{
    cursor::{MoveToNextLine, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::Print,
};
use std::{
    cell::RefCell,
    io::{self, Write},
    time::Duration,
};

use crate::{
    components::{
        BankDepositInput, BankWithdrawInput, BuyInput, BuyPrompt, CheapGoodDialog, GameEndScreen,
        PayDebtInput, SailPrompt, SellInput, SellPrompt, SplashScreen, StashDepositInput,
        StashDepositPrompt, StashWithdrawInput, StashWithdrawPrompt, ViewingInventoryActions,
        ViewingInventoryBase,
    },
    state::{GameEvent, GameState, GoodType, Location, Mode, StateError},
};

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

pub type UpdateFn = dyn FnOnce(KeyEvent, &GameState) -> UpdateResult<Option<GameState>>;
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
                '1' => Some(GoodType::Tea),
                '2' => Some(GoodType::Coffee),
                '3' => Some(GoodType::Sugar),
                '4' => Some(GoodType::Tobacco),
                '5' => Some(GoodType::Rum),
                '6' => Some(GoodType::Cotton),
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
                '1' => Some(Location::London),
                '2' => Some(Location::Savannah),
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

pub struct Engine<'a, Writer: Write> {
    pub writer: &'a RefCell<Writer>,
}

impl<'a, Writer: Write> Engine<'a, Writer> {
    pub fn new(writer: &'a RefCell<Writer>) -> Self {
        Self { writer }
    }

    pub fn draw_and_prompt(
        &mut self,
        game_state: &GameState,
    ) -> Result<(bool, Option<GameState>), UpdateError> {
        // draw the game state
        let update_fn = self.draw_scene(game_state)?;
        // Wait for any user event
        loop {
            // Wait up to 1s for some user event per loop iteration
            if poll(Duration::from_millis(1_000))? {
                // Read what even happened from the poll
                // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
                match read()? {
                    Event::Key(event) => {
                        // detect exit request
                        if event.modifiers == KeyModifiers::CONTROL
                            && event.code == KeyCode::Char('c')
                        {
                            return Ok((true, None));
                        }
                        // move forward game state
                        return update_fn(event, game_state).map(|st| (false, st));
                    }
                    _ => continue,
                }
            } else {
                // Timeout expired, no event for 1s, wait for user input again
                continue;
            }
        }
    }

    fn queue_scene(writer: &mut Writer, state: &GameState) -> io::Result<Box<UpdateFn>> {
        if !state.initialized {
            // initial splash screen
            queue!(writer, SplashScreen())?;
            return Ok(Box::new(|_: KeyEvent, state: &GameState| {
                Ok(Some(state.initialize()))
            }));
        } else if state.game_end {
            queue!(writer, GameEndScreen(state))?;
            return Ok(Box::new(|_: KeyEvent, _: &GameState| Ok(None)));
        } else {
            queue!(writer, ViewingInventoryBase(state))?;
            match &state.mode {
                Mode::ViewingInventory => {
                    queue!(writer, ViewingInventoryActions(&state.location, 9, 19))?;
                    return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                        if let KeyCode::Char(ch) = event.code {
                            match ch {
                                '1' => return Ok(Some(state.begin_buying()?)),
                                '2' => return Ok(Some(state.begin_selling()?)),
                                '3' => return Ok(Some(state.begin_sailing()?)),
                                _ => {}
                            };
                            if state.location == Location::London {
                                match ch {
                                    '4' => return Ok(Some(state.begin_stash_deposit()?)),
                                    '5' => return Ok(Some(state.begin_stash_withdraw()?)),
                                    '6' => return Ok(Some(state.begin_pay_debt()?)),
                                    '7' => return Ok(Some(state.begin_bank_deposit()?)),
                                    '8' => return Ok(Some(state.begin_bank_withdraw()?)),
                                    _ => {}
                                };
                            }
                            Ok(None)
                        } else {
                            Ok(None)
                        }
                    }));
                }
                Mode::Buying(info) => {
                    if let Some(info) = info {
                        queue!(writer, BuyInput(info, state, 9, 19))?;
                        return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                            if let KeyCode::Char(c) = event.code {
                                if let Some(digit) = c.to_digit(10) {
                                    return Ok(Some(state.user_typed_digit(digit)?));
                                }
                            }
                            if event.code == KeyCode::Backspace {
                                return Ok(Some(state.user_typed_backspace()?));
                            }
                            if event.code == KeyCode::Enter {
                                return match state.commit_buy() {
                                    Ok(new_state) => Ok(Some(new_state)),
                                    Err(variant) => match variant {
                                        StateError::CannotAfford | StateError::InsufficientHold => {
                                            Ok(None)
                                        }
                                        x => Err(x.into()),
                                    },
                                };
                            }
                            return Ok(None);
                        }));
                    } else {
                        queue!(writer, BuyPrompt(9, 19))?;
                        return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                            if let Some(good) = GoodType::from_key_code(&event.code) {
                                Ok(Some(state.choose_buy_good(good)?))
                            } else if event.code == KeyCode::Backspace {
                                Ok(Some(state.cancel_buy()?))
                            } else {
                                Ok(None)
                            }
                        }));
                    }
                }
                Mode::Selling(info) => {
                    if let Some(info) = info {
                        // user has indicated which good they want to sell
                        let current_amount = state.inventory.good_amount(&info.good);
                        queue!(writer, SellInput(info, current_amount, 9, 19))?;
                        return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                            if let KeyCode::Char(c) = event.code {
                                if let Some(digit) = c.to_digit(10) {
                                    return Ok(Some(state.user_typed_digit(digit)?));
                                }
                            }
                            if event.code == KeyCode::Backspace {
                                return Ok(Some(state.user_typed_backspace()?));
                            }
                            if event.code == KeyCode::Enter {
                                return match state.commit_sell() {
                                    Ok(new_state) => Ok(Some(new_state)),
                                    Err(variant) => match variant {
                                        StateError::InsufficientInventory => Ok(None),
                                        x => Err(x.into()),
                                    },
                                };
                            }
                            Ok(None)
                        }));
                    } else {
                        // user is choosing which good to sell
                        queue!(writer, SellPrompt(9, 19))?;
                        return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                            if let Some(good) = GoodType::from_key_code(&event.code) {
                                Ok(Some(state.choose_sell_good(good)?))
                            } else if event.code == KeyCode::Backspace {
                                Ok(Some(state.cancel_sell()?))
                            } else {
                                Ok(None)
                            }
                        }));
                    }
                }
                Mode::Sailing => {
                    // user is choosing where to sail
                    queue!(writer, SailPrompt(9, 19))?;
                    return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                        if let Some(destination) = Location::from_key_code(&event.code) {
                            match state.sail_to(&destination) {
                                Ok(new_state) => Ok(Some(new_state)),
                                Err(variant) => match variant {
                                    StateError::AlreadyInLocation => Ok(None),
                                    x => Err(x.into()),
                                },
                            }
                        } else if event.code == KeyCode::Backspace {
                            Ok(Some(state.cancel_sail_to()?))
                        } else {
                            Ok(None)
                        }
                    }));
                }
                Mode::StashDeposit(info) => {
                    if let Some(info) = info {
                        // user has indicated which good they want to stash
                        let good = &info.good;
                        let current_amount = state.inventory.good_amount(good);
                        queue!(writer, StashDepositInput(info, current_amount, 9, 19))?;
                        return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                            if let KeyCode::Char(c) = event.code {
                                if let Some(digit) = c.to_digit(10) {
                                    return Ok(Some(state.user_typed_digit(digit)?));
                                }
                            }
                            if event.code == KeyCode::Backspace {
                                return Ok(Some(state.user_typed_backspace()?));
                            }
                            if event.code == KeyCode::Enter {
                                return match state.commit_stash_deposit() {
                                    Ok(new_state) => Ok(Some(new_state)),
                                    Err(variant) => match variant {
                                        StateError::InsufficientInventory => Ok(None),
                                        x => Err(x.into()),
                                    },
                                };
                            }
                            Ok(None)
                        }));
                    } else {
                        // user is choosing which good to stash
                        queue!(writer, StashDepositPrompt(9, 19))?;
                        return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                            if let Some(good) = GoodType::from_key_code(&event.code) {
                                Ok(Some(state.choose_stash_deposit_good(good)?))
                            } else if event.code == KeyCode::Backspace {
                                Ok(Some(state.cancel_stash_deposit()?))
                            } else {
                                Ok(None)
                            }
                        }));
                    }
                }
                Mode::StashWithdraw(info) => {
                    if let Some(info) = info {
                        // user has indicated which good they want to withdraw from stash
                        let good = &info.good;
                        let current_amount = state.stash.good_amount(good);
                        queue!(writer, StashWithdrawInput(info, current_amount, 9, 19))?;
                        return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                            if let KeyCode::Char(c) = event.code {
                                if let Some(digit) = c.to_digit(10) {
                                    return Ok(Some(state.user_typed_digit(digit)?));
                                }
                            }
                            if event.code == KeyCode::Backspace {
                                return Ok(Some(state.user_typed_backspace()?));
                            }
                            if event.code == KeyCode::Enter {
                                return match state.commit_stash_withdraw() {
                                    Ok(new_state) => Ok(Some(new_state)),
                                    Err(variant) => match variant {
                                        StateError::InsufficientStash => Ok(None),
                                        x => Err(x.into()),
                                    },
                                };
                            }
                            Ok(None)
                        }));
                    } else {
                        // user is choosing which good to withdraw from stash
                        queue!(writer, StashWithdrawPrompt(9, 19))?;
                        return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                            if let Some(good) = GoodType::from_key_code(&event.code) {
                                Ok(Some(state.choose_stash_withdraw_good(good)?))
                            } else if event.code == KeyCode::Backspace {
                                Ok(Some(state.cancel_stash_withdraw()?))
                            } else {
                                Ok(None)
                            }
                        }));
                    }
                }
                Mode::PayDebt(amount) => {
                    queue!(writer, PayDebtInput(amount, 9, 19))?;
                    return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                        if let KeyCode::Char(c) = event.code {
                            if let Some(digit) = c.to_digit(10) {
                                return Ok(Some(state.user_typed_digit(digit)?));
                            }
                        }
                        if event.code == KeyCode::Backspace {
                            return Ok(Some(state.user_typed_backspace()?));
                        }
                        if event.code == KeyCode::Enter {
                            return match state.commit_pay_debt() {
                                Ok(new_state) => Ok(Some(new_state)),
                                Err(variant) => match variant {
                                    StateError::PayDownAmountHigherThanDebt => Ok(None),
                                    StateError::CannotAfford => Ok(None),
                                    x => Err(x.into()),
                                },
                            };
                        }
                        Ok(None)
                    }));
                }
                Mode::BankDeposit(amount) => {
                    queue!(writer, BankDepositInput(amount, 9, 19))?;
                    return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                        if let KeyCode::Char(c) = event.code {
                            if let Some(digit) = c.to_digit(10) {
                                return Ok(Some(state.user_typed_digit(digit)?));
                            }
                        }
                        if event.code == KeyCode::Backspace {
                            return Ok(Some(state.user_typed_backspace()?));
                        }
                        if event.code == KeyCode::Enter {
                            return match state.commit_bank_deposit() {
                                Ok(new_state) => Ok(Some(new_state)),
                                Err(variant) => match variant {
                                    StateError::CannotAfford => Ok(None),
                                    x => Err(x.into()),
                                },
                            };
                        }
                        Ok(None)
                    }));
                }
                Mode::BankWithdraw(amount) => {
                    queue!(writer, BankWithdrawInput(amount, 9, 19))?;
                    return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                        if let KeyCode::Char(c) = event.code {
                            if let Some(digit) = c.to_digit(10) {
                                return Ok(Some(state.user_typed_digit(digit)?));
                            }
                        }
                        if event.code == KeyCode::Backspace {
                            return Ok(Some(state.user_typed_backspace()?));
                        }
                        if event.code == KeyCode::Enter {
                            return match state.commit_bank_withdraw() {
                                Ok(new_state) => Ok(Some(new_state)),
                                Err(variant) => match variant {
                                    StateError::InsufficientBank => Ok(None),
                                    x => Err(x.into()),
                                },
                            };
                        }
                        Ok(None)
                    }));
                }
                Mode::GameEvent(event) => match event {
                    GameEvent::CheapGood(good) => {
                        queue!(writer, CheapGoodDialog(good, 9, 19))?;
                        return Ok(Box::new(|_: KeyEvent, state: &GameState| {
                            Ok(Some(state.acknowledge_event()?))
                        }));
                    }
                },
            }
        }
    }

    pub fn draw_scene(&mut self, state: &GameState) -> io::Result<Box<UpdateFn>> {
        let writer = &mut *self.writer.borrow_mut();
        let update = Engine::queue_scene(writer, state)?;
        writer.flush()?;
        Ok(update)
    }

    pub fn exit_message(&mut self, msg: &[&str]) -> io::Result<()> {
        let writer = &mut *self.writer.borrow_mut();
        execute!(writer, Show, MoveToNextLine(1),)?;
        for line in msg {
            execute!(writer, Show, MoveToNextLine(1), Print(line),)?;
        }
        execute!(writer, MoveToNextLine(1))?;
        Ok(())
    }
}
