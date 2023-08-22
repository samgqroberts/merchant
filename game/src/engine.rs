use crossterm::{
    cursor::{Hide, MoveTo, MoveToNextLine, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{style, Attribute, Color, Print, PrintStyledContent, Stylize},
    terminal::Clear,
};
use std::{
    cell::RefCell,
    io::{self, Write},
    time::Duration,
};

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
                // date
                MoveTo(9, 0),
                PrintStyledContent(format!("Date {}", state.date.to_string()).with(Color::White)),
                // hold size
                MoveTo(32, 0),
                PrintStyledContent(format!("Hold Size {}", state.hold_size).with(Color::White)),
                // gold
                MoveTo(9, 1),
                PrintStyledContent(format!("Gold {}", state.gold).with(Color::White)),
                // location
                MoveTo(33, 1),
                PrintStyledContent(format!("Location {}", state.location).with(Color::White)),
                // inventory
                MoveTo(9, 3),
                PrintStyledContent("Inventory".with(Color::White)),
                MoveTo(11, 4),
                PrintStyledContent(format!("Sugar: {}", state.inventory.sugar).with(Color::White)),
                MoveTo(9, 5),
                PrintStyledContent(
                    format!("Tobacco: {}", state.inventory.tobacco).with(Color::White)
                ),
                MoveTo(13, 6),
                PrintStyledContent(format!("Tea: {}", state.inventory.tea).with(Color::White)),
                MoveTo(10, 7),
                PrintStyledContent(
                    format!("Cotton: {}", state.inventory.cotton).with(Color::White)
                ),
                MoveTo(13, 8),
                PrintStyledContent(format!("Rum: {}", state.inventory.rum).with(Color::White)),
                MoveTo(10, 9),
                PrintStyledContent(
                    format!("Coffee: {}", state.inventory.coffee).with(Color::White)
                ),
                // current prices
                MoveTo(5, 11),
                PrintStyledContent("Captain, the prices of goods here are:".with(Color::White)),
                MoveTo(11, 12),
                PrintStyledContent(
                    format!(
                        "Sugar: {}",
                        state.prices.location_prices(&state.location).sugar
                    )
                    .with(Color::White)
                ),
                MoveTo(27, 12),
                PrintStyledContent(
                    format!(
                        "Tobacco: {}",
                        state.prices.location_prices(&state.location).tobacco
                    )
                    .with(Color::White)
                ),
                MoveTo(13, 13),
                PrintStyledContent(
                    format!("Tea: {}", state.prices.location_prices(&state.location).tea)
                        .with(Color::White)
                ),
                MoveTo(28, 13),
                PrintStyledContent(
                    format!(
                        "Cotton: {}",
                        state.prices.location_prices(&state.location).cotton
                    )
                    .with(Color::White)
                ),
                MoveTo(13, 14),
                PrintStyledContent(
                    format!("Rum: {}", state.prices.location_prices(&state.location).rum)
                        .with(Color::White)
                ),
                MoveTo(28, 14),
                PrintStyledContent(
                    format!(
                        "Coffee: {}",
                        state.prices.location_prices(&state.location).coffee
                    )
                    .with(Color::White)
                ),
            )?;
            match &state.mode {
                Mode::ViewingInventory => {
                    queue!(
                        writer,
                        // actions
                        MoveTo(9, 16),
                        PrintStyledContent("(1) Buy".with(Color::White)),
                        MoveTo(9, 17),
                        PrintStyledContent("(2) Sell".with(Color::White)),
                        MoveTo(9, 18),
                        PrintStyledContent("(3) Sail".with(Color::White)),
                    )?;
                    return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                        if let KeyCode::Char(ch) = event.code {
                            match ch {
                                '1' => Ok(Some(state.begin_buying()?)),
                                '2' => Ok(Some(state.begin_selling()?)),
                                '3' => Ok(Some(state.begin_sailing()?)),
                                _ => Ok(None),
                            }
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
                            MoveTo(9, 16),
                            PrintStyledContent(prompt.with(Color::White)),
                            MoveTo(9, 17),
                            PrintStyledContent(
                                format!("You can afford ({})", can_afford).with(Color::White)
                            ),
                            MoveTo(9 + prompt_len, 16),
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
                            MoveTo(9, 16),
                            PrintStyledContent("Which do you want to buy?".with(Color::White)),
                            MoveTo(9, 17),
                            PrintStyledContent("(1) Sugar".with(Color::White)),
                            MoveTo(9, 18),
                            PrintStyledContent("(2) Tobacco".with(Color::White)),
                            MoveTo(9, 19),
                            PrintStyledContent("(3) Tea".with(Color::White)),
                            MoveTo(9, 20),
                            PrintStyledContent("(4) Cotton".with(Color::White)),
                            MoveTo(9, 21),
                            PrintStyledContent("(5) Rum".with(Color::White)),
                            MoveTo(9, 22),
                            PrintStyledContent("(6) Coffee".with(Color::White)),
                        )?;
                        return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                            if let Some(good) = GoodType::from_key_code(&event.code) {
                                return Ok(Some(state.choose_buy_good(good)?));
                            }
                            return Ok(None);
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
                            MoveTo(9, 16),
                            PrintStyledContent(prompt.with(Color::White)),
                            MoveTo(9, 17),
                            PrintStyledContent(
                                format!("You have ({})", current_amount).with(Color::White)
                            ),
                            MoveTo(9 + prompt_len, 16),
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
                            MoveTo(9, 16),
                            PrintStyledContent("Which do you want to sell?".with(Color::White)),
                            MoveTo(9, 17),
                            PrintStyledContent("(1) Sugar".with(Color::White)),
                            MoveTo(9, 18),
                            PrintStyledContent("(2) Tobacco".with(Color::White)),
                            MoveTo(9, 19),
                            PrintStyledContent("(3) Tea".with(Color::White)),
                            MoveTo(9, 20),
                            PrintStyledContent("(4) Cotton".with(Color::White)),
                            MoveTo(9, 21),
                            PrintStyledContent("(5) Rum".with(Color::White)),
                            MoveTo(9, 22),
                            PrintStyledContent("(6) Coffee".with(Color::White)),
                        )?;
                        return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                            if let Some(good) = GoodType::from_key_code(&event.code) {
                                return Ok(Some(state.choose_sell_good(good)?));
                            }
                            return Ok(None);
                        }));
                    }
                }
                Mode::Sailing => {
                    // user is choosing where to sail
                    queue!(
                        writer,
                        MoveTo(9, 16),
                        PrintStyledContent("Where do you want to sail?".with(Color::White)),
                        MoveTo(9, 17),
                        PrintStyledContent("(1) Savannah".with(Color::White)),
                        MoveTo(9, 18),
                        PrintStyledContent("(2) London".with(Color::White)),
                        MoveTo(9, 19),
                        PrintStyledContent("(3) Lisbon".with(Color::White)),
                        MoveTo(9, 20),
                        PrintStyledContent("(4) Amsterdam".with(Color::White)),
                        MoveTo(9, 21),
                        PrintStyledContent("(5) Cape Town".with(Color::White)),
                        MoveTo(9, 22),
                        PrintStyledContent("(6) Venice".with(Color::White)),
                    )?;
                    return Ok(Box::new(|event: KeyEvent, state: &GameState| {
                        if let Some(destination) = Location::from_key_code(&event.code) {
                            return match state.relocate(&destination) {
                                Ok(new_state) => Ok(Some(new_state)),
                                Err(variant) => match variant {
                                    StateError::AlreadyInLocation => Ok(None),
                                    x => Err(x.into()),
                                },
                            };
                        }
                        return Ok(None);
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
