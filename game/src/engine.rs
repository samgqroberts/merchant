use core::fmt;
use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{style, Attribute, Color, Print, PrintStyledContent, Stylize},
    terminal::Clear,
    Command,
};
use std::{
    cell::RefCell,
    io::{self, Write},
    time::Duration,
};

use crate::{
    comp,
    state::{GameState, GoodType, Inventory, Location, Mode, StateError},
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
            queue!(
                writer,
                Clear(crossterm::terminal::ClearType::All),
                MoveTo(0, 0),
                PrintStyledContent(
                    "Merchant"
                        .with(Color::Yellow)
                        .on(Color::Blue)
                        .attribute(Attribute::Bold)
                ),
                MoveTo(0, 2),
                PrintStyledContent(
                    "Navigate shifting markets and unreliable sources."
                        .with(Color::Yellow)
                        .on(Color::Blue)
                        .attribute(Attribute::Bold)
                ),
                MoveTo(0, 4),
                PrintStyledContent(
                    "By samgqroberts"
                        .with(Color::Yellow)
                        .on(Color::Blue)
                        .attribute(Attribute::Bold)
                ),
                // prompt user
                MoveToNextLine(2),
                PrintStyledContent(
                    style("Press any key to begin")
                        .with(Color::Blue)
                        .on(Color::Yellow)
                        .attribute(Attribute::Bold),
                ),
                Hide
            )?;
            return Ok(Box::new(|_: KeyEvent, state: &GameState| {
                Ok(Some(state.initialize()))
            }));
        } else {
            queue!(
                writer,
                // clear terminal
                Clear(crossterm::terminal::ClearType::All),
                // hide cursor
                Hide,
                // date
                MoveTo(9, 0),
                PrintStyledContent(
                    format!("{} {}", state.date.1.name(), state.date.0.to_string())
                        .with(Color::White)
                ),
                // hold size
                MoveTo(32, 0),
                PrintStyledContent(format!("Hold Size {}", state.hold_size).with(Color::White)),
                // gold
                MoveTo(9, 1),
                PrintStyledContent(format!("Gold {}", state.gold).with(Color::White)),
                // location
                MoveTo(33, 1),
                PrintStyledContent(format!("Location {}", state.location).with(Color::White)),
                // home base
                MoveTo(10, 3),
                PrintStyledContent("Home base".with(Color::White)),
                InventoryList(&state.stash, 9, 4),
                MoveTo(12, 11),
                PrintStyledContent(format!("Bank: {}", state.bank).with(Color::White)),
                MoveTo(12, 12),
                PrintStyledContent(format!("Debt: {}", state.debt).with(Color::White)),
                // inventory
                MoveTo(33, 3),
                PrintStyledContent("Inventory".with(Color::White)),
                InventoryList(&state.inventory, 32, 4),
                // current prices
                CurrentPrices(state.prices.location_prices(&state.location), 5, 14),
            )?;
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
                                    '6' => return Ok(Some(state.begin_borrow_gold()?)),
                                    '7' => return Ok(Some(state.begin_pay_debt()?)),
                                    '8' => return Ok(Some(state.begin_bank_deposit()?)),
                                    '9' => return Ok(Some(state.begin_bank_withdraw()?)),
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
                        // user has indicated which good they want to buy
                        let good = &info.good;
                        let prompt = format!(
                            "How much {} do you want? {}",
                            good,
                            info.amount
                                .map_or("".to_owned(), |amount| amount.to_string())
                        );
                        let prompt_len: u16 = prompt.len().try_into().unwrap();
                        let good_price = state
                            .prices
                            .location_prices(&state.location)
                            .good_amount(&good);
                        let can_afford = state.gold / good_price;
                        queue!(
                            writer,
                            // prompt what to buy
                            MoveTo(9, 19),
                            PrintStyledContent(prompt.with(Color::White)),
                            MoveTo(9, 20),
                            PrintStyledContent(
                                format!("You can afford ({})", can_afford).with(Color::White)
                            ),
                            MoveTo(9 + prompt_len, 19),
                            Show
                        )?;
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
                        // user is choosing which good to buy
                        queue!(
                            writer,
                            // prompt what to buy
                            MoveTo(9, 19),
                            PrintStyledContent("Which do you want to buy?".with(Color::White)),
                            MoveTo(9, 20),
                            PrintStyledContent("(1) Sugar".with(Color::White)),
                            MoveTo(9, 21),
                            PrintStyledContent("(2) Tobacco".with(Color::White)),
                            MoveTo(9, 22),
                            PrintStyledContent("(3) Tea".with(Color::White)),
                            MoveTo(9, 23),
                            PrintStyledContent("(4) Cotton".with(Color::White)),
                            MoveTo(9, 24),
                            PrintStyledContent("(5) Rum".with(Color::White)),
                            MoveTo(9, 25),
                            PrintStyledContent("(6) Coffee".with(Color::White)),
                        )?;
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
                        let good = &info.good;
                        let prompt = format!(
                            "How much {} do you want to sell? {}",
                            good,
                            info.amount
                                .map_or("".to_owned(), |amount| amount.to_string())
                        );
                        let prompt_len: u16 = prompt.len().try_into().unwrap();
                        let current_amount = state.inventory.good_amount(good);
                        queue!(
                            writer,
                            // prompt what to sell
                            MoveTo(9, 19),
                            PrintStyledContent(prompt.with(Color::White)),
                            MoveTo(9, 20),
                            PrintStyledContent(
                                format!("You have ({})", current_amount).with(Color::White)
                            ),
                            MoveTo(9 + prompt_len, 19),
                            Show
                        )?;
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
                        queue!(
                            writer,
                            // prompt what to sell
                            MoveTo(9, 19),
                            PrintStyledContent("Which do you want to sell?".with(Color::White)),
                            MoveTo(9, 20),
                            PrintStyledContent("(1) Sugar".with(Color::White)),
                            MoveTo(9, 21),
                            PrintStyledContent("(2) Tobacco".with(Color::White)),
                            MoveTo(9, 22),
                            PrintStyledContent("(3) Tea".with(Color::White)),
                            MoveTo(9, 23),
                            PrintStyledContent("(4) Cotton".with(Color::White)),
                            MoveTo(9, 24),
                            PrintStyledContent("(5) Rum".with(Color::White)),
                            MoveTo(9, 25),
                            PrintStyledContent("(6) Coffee".with(Color::White)),
                        )?;
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
                    queue!(
                        writer,
                        MoveTo(9, 19),
                        PrintStyledContent("Where do you want to sail?".with(Color::White)),
                        MoveTo(9, 20),
                        PrintStyledContent("(1) London".with(Color::White)),
                        MoveTo(9, 21),
                        PrintStyledContent("(2) Savannah".with(Color::White)),
                        MoveTo(9, 22),
                        PrintStyledContent("(3) Lisbon".with(Color::White)),
                        MoveTo(9, 23),
                        PrintStyledContent("(4) Amsterdam".with(Color::White)),
                        MoveTo(9, 24),
                        PrintStyledContent("(5) Cape Town".with(Color::White)),
                        MoveTo(9, 25),
                        PrintStyledContent("(6) Venice".with(Color::White)),
                    )?;
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
                        let prompt = format!(
                            "How much {} do you want to stash? {}",
                            good,
                            info.amount
                                .map_or("".to_owned(), |amount| amount.to_string())
                        );
                        let prompt_len: u16 = prompt.len().try_into().unwrap();
                        let current_amount = state.inventory.good_amount(good);
                        queue!(
                            writer,
                            MoveTo(9, 19),
                            PrintStyledContent(prompt.with(Color::White)),
                            MoveTo(9, 20),
                            PrintStyledContent(
                                format!("You have ({})", current_amount).with(Color::White)
                            ),
                            MoveTo(9 + prompt_len, 19),
                            Show
                        )?;
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
                        queue!(
                            writer,
                            MoveTo(9, 19),
                            PrintStyledContent("Which do you want to stash?".with(Color::White)),
                            MoveTo(9, 20),
                            PrintStyledContent("(1) Sugar".with(Color::White)),
                            MoveTo(9, 21),
                            PrintStyledContent("(2) Tobacco".with(Color::White)),
                            MoveTo(9, 22),
                            PrintStyledContent("(3) Tea".with(Color::White)),
                            MoveTo(9, 23),
                            PrintStyledContent("(4) Cotton".with(Color::White)),
                            MoveTo(9, 24),
                            PrintStyledContent("(5) Rum".with(Color::White)),
                            MoveTo(9, 25),
                            PrintStyledContent("(6) Coffee".with(Color::White)),
                        )?;
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
                        let prompt = format!(
                            "How much {} do you want to withdraw? {}",
                            good,
                            info.amount
                                .map_or("".to_owned(), |amount| amount.to_string())
                        );
                        let prompt_len: u16 = prompt.len().try_into().unwrap();
                        let current_amount = state.stash.good_amount(good);
                        queue!(
                            writer,
                            MoveTo(9, 19),
                            PrintStyledContent(prompt.with(Color::White)),
                            MoveTo(9, 20),
                            PrintStyledContent(
                                format!("There are ({})", current_amount).with(Color::White)
                            ),
                            MoveTo(9 + prompt_len, 19),
                            Show
                        )?;
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
                        queue!(
                            writer,
                            MoveTo(9, 19),
                            PrintStyledContent("Which do you want to withdraw?".with(Color::White)),
                            MoveTo(9, 20),
                            PrintStyledContent("(1) Sugar".with(Color::White)),
                            MoveTo(9, 21),
                            PrintStyledContent("(2) Tobacco".with(Color::White)),
                            MoveTo(9, 22),
                            PrintStyledContent("(3) Tea".with(Color::White)),
                            MoveTo(9, 23),
                            PrintStyledContent("(4) Cotton".with(Color::White)),
                            MoveTo(9, 24),
                            PrintStyledContent("(5) Rum".with(Color::White)),
                            MoveTo(9, 25),
                            PrintStyledContent("(6) Coffee".with(Color::White)),
                        )?;
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
                Mode::BorrowGold(amount) => {
                    let prompt = format!(
                        "How much gold do you want to borrow? {}",
                        amount.map_or("".to_owned(), |amount| amount.to_string())
                    );
                    let prompt_len: u16 = prompt.len().try_into().unwrap();
                    queue!(
                        writer,
                        MoveTo(9, 19),
                        PrintStyledContent(prompt.with(Color::White)),
                        MoveTo(9 + prompt_len, 19),
                        Show
                    )?;
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
                            return match state.commit_borrow_gold() {
                                Ok(new_state) => Ok(Some(new_state)),
                                Err(variant) => match variant {
                                    StateError::InsufficientStash => Ok(None),
                                    x => Err(x.into()),
                                },
                            };
                        }
                        Ok(None)
                    }));
                }
                Mode::PayDebt(amount) => {
                    let prompt = format!(
                        "How much debt do you want to pay down? {}",
                        amount.map_or("".to_owned(), |amount| amount.to_string())
                    );
                    let prompt_len: u16 = prompt.len().try_into().unwrap();
                    queue!(
                        writer,
                        MoveTo(9, 19),
                        PrintStyledContent(prompt.with(Color::White)),
                        MoveTo(9 + prompt_len, 19),
                        Show
                    )?;
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
                    queue!(writer, BankDepositPrompt(amount, 9, 19))?;
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
                    queue!(writer, BankWithdrawPrompt(amount, 9, 19))?;
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

pub struct InventoryList<'a>(&'a Inventory, u16, u16);

impl<'a> Command for InventoryList<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let inventory = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        MoveTo(offset_x + 2, offset_y).write_ansi(f)?;
        PrintStyledContent(format!("Sugar: {}", inventory.sugar).with(Color::White))
            .write_ansi(f)?;
        MoveTo(offset_x, offset_y + 1).write_ansi(f)?;
        PrintStyledContent(format!("Tobacco: {}", inventory.tobacco).with(Color::White))
            .write_ansi(f)?;
        MoveTo(offset_x + 4, offset_y + 2).write_ansi(f)?;
        PrintStyledContent(format!("Tea: {}", inventory.tea).with(Color::White)).write_ansi(f)?;
        MoveTo(offset_x + 1, offset_y + 3).write_ansi(f)?;
        PrintStyledContent(format!("Cotton: {}", inventory.cotton).with(Color::White))
            .write_ansi(f)?;
        MoveTo(offset_x + 4, offset_y + 4).write_ansi(f)?;
        PrintStyledContent(format!("Rum: {}", inventory.rum).with(Color::White)).write_ansi(f)?;
        MoveTo(offset_x + 1, offset_y + 5).write_ansi(f)?;
        PrintStyledContent(format!("Coffee: {}", inventory.coffee).with(Color::White))
            .write_ansi(f)?;
        Ok(())
    }
}

pub struct CurrentPrices<'a>(&'a Inventory, u16, u16);

impl<'a> Command for CurrentPrices<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let prices = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent("Captain, the prices of goods here are:".with(Color::White)),
            MoveTo(offset_x + 6, offset_y + 1),
            PrintStyledContent(format!("Sugar: {}", prices.sugar).with(Color::White)),
            MoveTo(offset_x + 27, offset_y + 1),
            PrintStyledContent(format!("Tobacco: {}", prices.tobacco).with(Color::White)),
            MoveTo(offset_x + 8, offset_y + 2),
            PrintStyledContent(format!("Tea: {}", prices.tea).with(Color::White)),
            MoveTo(offset_x + 28, offset_y + 2),
            PrintStyledContent(format!("Cotton: {}", prices.cotton).with(Color::White)),
            MoveTo(offset_x + 8, offset_y + 3),
            PrintStyledContent(format!("Rum: {}", prices.rum).with(Color::White)),
            MoveTo(offset_x + 28, offset_y + 3),
            PrintStyledContent(format!("Coffee: {}", prices.coffee).with(Color::White)),
        );
        Ok(())
    }
}

pub struct ViewingInventoryActions<'a>(&'a Location, u16, u16);

impl<'a> Command for ViewingInventoryActions<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let location = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        comp!(
            f,
            // actions
            MoveTo(offset_x, offset_y),
            PrintStyledContent("(1) Buy".with(Color::White)),
            MoveTo(offset_x, offset_y + 1),
            PrintStyledContent("(2) Sell".with(Color::White)),
            MoveTo(offset_x, offset_y + 2),
            PrintStyledContent("(3) Sail".with(Color::White)),
        );
        if location == &Location::London {
            comp!(
                f,
                MoveTo(offset_x, offset_y + 3),
                PrintStyledContent("(4) Stash deposit".with(Color::White)),
                MoveTo(offset_x, offset_y + 4),
                PrintStyledContent("(5) Stash withdraw".with(Color::White)),
                MoveTo(offset_x, offset_y + 5),
                PrintStyledContent("(6) Borrow gold".with(Color::White)),
                MoveTo(offset_x, offset_y + 6),
                PrintStyledContent("(7) Pay down debt".with(Color::White)),
                MoveTo(offset_x, offset_y + 7),
                PrintStyledContent("(8) Bank deposit".with(Color::White)),
                MoveTo(offset_x, offset_y + 8),
                PrintStyledContent("(9) Bank withdraw".with(Color::White)),
            );
        }
        Ok(())
    }
}

pub struct BankDepositPrompt<'a>(&'a Option<u32>, u16, u16);

impl<'a> Command for BankDepositPrompt<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let amount = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        let prompt = format!(
            "How much gold do you want to deposit in the bank? {}",
            amount.map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(offset_x + prompt_len, offset_y),
            Show
        );
        Ok(())
    }
}

pub struct BankWithdrawPrompt<'a>(&'a Option<u32>, u16, u16);

impl<'a> Command for BankWithdrawPrompt<'a> {
    fn write_ansi(&self, f: &mut impl fmt::Write) -> fmt::Result {
        let amount = self.0;
        let offset_x = self.1;
        let offset_y = self.2;
        let prompt = format!(
            "How much gold do you want to withdraw? {}",
            amount.map_or("".to_owned(), |amount| amount.to_string())
        );
        let prompt_len: u16 = prompt.len().try_into().unwrap();
        comp!(
            f,
            MoveTo(offset_x, offset_y),
            PrintStyledContent(prompt.with(Color::White)),
            MoveTo(offset_x + prompt_len, offset_y),
            Show
        );
        Ok(())
    }
}
