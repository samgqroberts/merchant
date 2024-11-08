use crossterm::{
    cursor::{MoveToNextLine, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::Print,
};
use std::{
    cell::RefCell,
    cmp::min,
    io::{self, Write},
    time::Duration,
};
use tracing::{debug, error, info};

use crate::{
    components::{
        BankDepositInput, BankWithdrawInput, BuyInput, BuyPrompt, CanBuyCannon, CanBuyHoldSpace,
        CheapGoodDialog, ExpensiveGoodDialog, FindGoodsDialog, GameEndScreen, GoodsStolenDialog,
        IntroductionScreen, NoEffect, PayDebtInput, PirateEncounter, RequireResize, SailPrompt,
        SellInput, SellPrompt, SplashScreen, StashDepositInput, StashDepositPrompt,
        StashWithdrawInput, StashWithdrawPrompt, ViewingInventoryActions, ViewingInventoryBase,
        FRAME_HEIGHT, FRAME_WIDTH,
    },
    state::{
        GameState, Good, Initialization, Location, LocationEvent, Mode, PirateEncounterState,
        StateError,
    },
};

#[derive(Debug)]
#[allow(unused)]
pub struct UpdateError(String);

impl From<io::Error> for UpdateError {
    fn from(value: io::Error) -> Self {
        Self(value.to_string())
    }
}

impl From<StateError> for UpdateError {
    fn from(value: StateError) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, PartialEq)]
pub enum UpdateSignal {
    Continue,
    Quit,
    Restart,
}

pub type UpdateResult<T> = Result<T, UpdateError>;

pub type UpdateFn = dyn FnOnce(KeyEvent, &mut GameState) -> UpdateResult<UpdateSignal>;

trait FromKeyCode
where
    Self: Sized,
{
    fn from_key_code(key_code: &KeyCode) -> Option<Self>;
}

impl FromKeyCode for Good {
    fn from_key_code(key_code: &KeyCode) -> Option<Self> {
        if let KeyCode::Char(c) = key_code {
            match c {
                '1' | 't' => Some(Good::Tea),
                '2' | 'c' => Some(Good::Coffee),
                '3' | 's' => Some(Good::Sugar),
                '4' | 'a' => Some(Good::Tobacco),
                '5' | 'r' => Some(Good::Rum),
                '6' | 'o' => Some(Good::Cotton),
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
                '1' | 'l' => Some(Location::London),
                '2' | 's' => Some(Location::Savannah),
                '3' | 'i' => Some(Location::Lisbon),
                '4' | 'a' => Some(Location::Amsterdam),
                '5' | 'c' => Some(Location::CapeTown),
                '6' | 'v' => Some(Location::Venice),
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
        game_state: &mut GameState,
    ) -> Result<UpdateSignal, UpdateError> {
        // check the terminal size, and if it needs to be taller or wider to fit the game, render those
        // instructions INSTEAD of the screen based on the game state
        let mut require_resize = false;
        if let Ok((current_x_cols, current_y_cols)) = crossterm::terminal::size() {
            debug!("Terminal size: {{x: {current_x_cols}, y: {current_y_cols}}}");
            if current_x_cols < FRAME_WIDTH || current_y_cols < FRAME_HEIGHT {
                require_resize = true;
                self.draw_need_resize(current_x_cols, current_y_cols)?;
            }
        } else {
            error!("Could not determine terminal size.");
        }
        // if terminal does not need to be resized draw the game state
        let mut update_fn: Option<_> = None;
        if !require_resize {
            update_fn = Some(self.draw_scene(game_state)?);
        }
        // Wait for any user event
        loop {
            // Wait up to 1s for some user event per loop iteration
            if poll(Duration::from_millis(1_000))? {
                // Read what even happened from the poll
                // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
                match read()? {
                    Event::Key(event) => {
                        // only react to Press KeyEvents as opposed to Release
                        // on Windows, both Press and Release get triggered
                        // if we don't filter to just Press events we will double-update
                        if event.kind == KeyEventKind::Press {
                            info!("User Key Press: {:?} {:?}", event.code, event.modifiers);
                            // detect exit request
                            if event.modifiers == KeyModifiers::CONTROL
                                && event.code == KeyCode::Char('c')
                            {
                                return Ok(UpdateSignal::Quit);
                            }
                            // update game state (if we have an update_fn, we may not if
                            // terminal needs to be resized)
                            if let Some(update_fn) = update_fn {
                                return update_fn(event, game_state);
                            } else {
                                return Ok(UpdateSignal::Continue);
                            }
                        }
                    }
                    Event::Resize(columns, rows) => {
                        info!("Terminal resized: {columns} columns, {rows} rows.");
                        return Ok(UpdateSignal::Continue); // trigger a rerender with no state updates
                    }
                    _ => continue,
                }
            } else {
                // Timeout expired, no event for 1s, wait for user input again
                continue;
            }
        }
    }

    fn queue_scene(writer: &mut Writer, state: &mut GameState) -> io::Result<Box<UpdateFn>> {
        if state.initialization == Initialization::SplashScreen {
            // initial splash screen
            queue!(writer, SplashScreen())?;
            Ok(Box::new(|_: KeyEvent, state: &mut GameState| {
                state.splash_to_introduction();
                Ok(UpdateSignal::Continue)
            }))
        } else if state.game_end {
            queue!(writer, GameEndScreen(state))?;
            Ok(Box::new(|event: KeyEvent, _: &mut GameState| {
                match event.code {
                    KeyCode::Char('q') => Ok(UpdateSignal::Quit),
                    KeyCode::Enter => Ok(UpdateSignal::Restart),
                    _ => Ok(UpdateSignal::Continue),
                }
            }))
        } else if let Mode::GameEvent(LocationEvent::PirateEncounter(pirate_encounter_state)) =
            &state.mode
        {
            let pirate_encounter_state = *pirate_encounter_state;
            queue!(
                writer,
                PirateEncounter::from((pirate_encounter_state, state))
            )?;
            return Ok(Box::new(move |event: KeyEvent, state: &mut GameState| {
                match &pirate_encounter_state {
                    PirateEncounterState::Initial => {
                        state.proceed_pirate_encounter()?;
                    }
                    PirateEncounterState::Prompt { info: _ } => {
                        if let KeyCode::Char('r') = event.code {
                            state.pirate_run()?;
                        } else if let KeyCode::Char('f') = event.code {
                            state.pirate_fight()?;
                        }
                    }
                    PirateEncounterState::RunSuccess => {
                        state.proceed_pirate_run_success()?;
                    }
                    PirateEncounterState::RunFailure { info: _ } => {
                        state.proceed_pirate_run_failure()?;
                    }
                    PirateEncounterState::PiratesAttack {
                        info: _,
                        damage_this_attack: _,
                    } => {
                        state.proceed_pirates_attack()?;
                    }
                    PirateEncounterState::Destroyed => {
                        state.proceed_destroyed()?;
                    }
                    PirateEncounterState::AttackResult {
                        info: _,
                        did_kill_a_pirate: _,
                    } => {
                        state.proceed_attack_result()?;
                    }
                    PirateEncounterState::Victory { gold_recovered: _ } => {
                        state.proceed_pirate_encounter_victory()?;
                    }
                }
                Ok(UpdateSignal::Continue)
            }));
        } else if state.initialization == Initialization::Introduction {
            // introduction screen
            queue!(
                writer,
                IntroductionScreen {
                    home: state.location_config.home_port,
                    starting_year: state.starting_date.0
                }
            )?;
            Ok(Box::new(|_: KeyEvent, state: &mut GameState| {
                state.introduction_to_game();
                Ok(UpdateSignal::Continue)
            }))
        } else {
            queue!(writer, ViewingInventoryBase(state))?;
            match &state.mode {
                Mode::ViewingInventory => {
                    queue!(
                        writer,
                        ViewingInventoryActions {
                            location: &state.location,
                            home_port: &state.location_config.home_port,
                            debt: state.debt.0
                        }
                    )?;
                    return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                        if let KeyCode::Char(ch) = event.code {
                            if ch == '1' || ch == 'b' {
                                state.begin_buying()?;
                            } else if ch == '2' || ch == 's' {
                                state.begin_selling()?;
                            } else if ch == '3' || ch == 'a' {
                                state.begin_sailing()?;
                            };
                            if state.location == state.location_config.home_port {
                                if ch == '4' || ch == 'd' {
                                    state.begin_stash_deposit()?;
                                } else if ch == '5' || ch == 'w' {
                                    state.begin_stash_withdraw()?;
                                } else if ch == '6' || ch == 'e' {
                                    state.begin_bank_deposit()?;
                                } else if ch == '7' || ch == 'i' {
                                    state.begin_bank_withdraw()?;
                                }
                                if state.debt.0 > 0 && (ch == '8' || ch == 'p') {
                                    state.begin_pay_debt()?;
                                }
                            }
                        }
                        Ok(UpdateSignal::Continue)
                    }));
                }
                Mode::Buying(info) => {
                    if let Some(info) = info {
                        queue!(writer, BuyInput { info, state })?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let KeyCode::Char(c) = event.code {
                                if c == 'b' {
                                    state.back()?;
                                } else if let Some(digit) = c.to_digit(10) {
                                    state.user_typed_digit(digit)?;
                                }
                            } else if event.code == KeyCode::Backspace {
                                state.user_typed_backspace()?;
                            } else if event.code == KeyCode::Enter {
                                return state.commit_buy().map(|_| UpdateSignal::Continue).or_else(
                                    |e| match e {
                                        StateError::CannotAfford | StateError::InsufficientHold => {
                                            Ok(UpdateSignal::Continue)
                                        }
                                        x => Err(x.into()),
                                    },
                                );
                            }
                            Ok(UpdateSignal::Continue)
                        }));
                    } else {
                        queue!(writer, BuyPrompt)?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let KeyCode::Char('b') = event.code {
                                state.back()?;
                            } else if let Some(good) = Good::from_key_code(&event.code) {
                                state.choose_buy_good(good)?;
                            } else if event.code == KeyCode::Backspace {
                                state.cancel_buy()?;
                            }
                            Ok(UpdateSignal::Continue)
                        }));
                    }
                }
                Mode::Selling(info) => {
                    if let Some(info) = info {
                        // user has indicated which good they want to sell
                        let current_amount = state.inventory.get_good(&info.good);
                        queue!(writer, SellInput(info, current_amount))?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let KeyCode::Char(c) = event.code {
                                if c == 'b' {
                                    state.back()?;
                                } else if let Some(digit) = c.to_digit(10) {
                                    state.user_typed_digit(digit)?;
                                }
                            } else if event.code == KeyCode::Backspace {
                                state.user_typed_backspace()?;
                            } else if event.code == KeyCode::Enter {
                                return state
                                    .commit_sell()
                                    .map(|_| UpdateSignal::Continue)
                                    .or_else(|e| match e {
                                        StateError::InsufficientInventory => {
                                            Ok(UpdateSignal::Continue)
                                        }
                                        x => Err(x.into()),
                                    });
                            }
                            Ok(UpdateSignal::Continue)
                        }));
                    } else {
                        // user is choosing which good to sell
                        queue!(writer, SellPrompt)?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let KeyCode::Char('b') = event.code {
                                state.back()?;
                            } else if let Some(good) = Good::from_key_code(&event.code) {
                                state.choose_sell_good(good)?;
                            } else if event.code == KeyCode::Backspace {
                                state.cancel_sell()?;
                            }
                            Ok(UpdateSignal::Continue)
                        }));
                    }
                }
                Mode::Sailing => {
                    // user is choosing where to sail
                    queue!(writer, SailPrompt)?;
                    return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                        if let KeyCode::Char('b') = event.code {
                            state.back()?;
                        } else if let Some(destination) = Location::from_key_code(&event.code) {
                            return state
                                .sail_to(&destination)
                                .map(|_| UpdateSignal::Continue)
                                .or_else(|e| match e {
                                    StateError::AlreadyInLocation => Ok(UpdateSignal::Continue),
                                    x => Err(x.into()),
                                });
                        } else if event.code == KeyCode::Backspace {
                            state.cancel_sail_to()?;
                        }
                        Ok(UpdateSignal::Continue)
                    }));
                }
                Mode::StashDeposit(info) => {
                    if let Some(info) = info {
                        // user has indicated which good they want to stash
                        let good = &info.good;
                        let current_amount = state.inventory.get_good(good);
                        queue!(writer, StashDepositInput(info, current_amount))?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let KeyCode::Char(c) = event.code {
                                if c == 'b' {
                                    state.back()?;
                                } else if let Some(digit) = c.to_digit(10) {
                                    state.user_typed_digit(digit)?;
                                }
                            }
                            if event.code == KeyCode::Backspace {
                                state.user_typed_backspace()?;
                            }
                            if event.code == KeyCode::Enter {
                                return state
                                    .commit_stash_deposit()
                                    .map(|_| UpdateSignal::Continue)
                                    .or_else(|e| match e {
                                        StateError::InsufficientInventory => {
                                            Ok(UpdateSignal::Continue)
                                        }
                                        x => Err(x.into()),
                                    });
                            }
                            Ok(UpdateSignal::Continue)
                        }));
                    } else {
                        // user is choosing which good to stash
                        queue!(writer, StashDepositPrompt)?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let KeyCode::Char('b') = event.code {
                                state.back()?;
                            } else if let Some(good) = Good::from_key_code(&event.code) {
                                state.choose_stash_deposit_good(good)?;
                            } else if event.code == KeyCode::Backspace {
                                state.cancel_stash_deposit()?;
                            }
                            Ok(UpdateSignal::Continue)
                        }));
                    }
                }
                Mode::StashWithdraw(info) => {
                    if let Some(info) = info {
                        // user has indicated which good they want to withdraw from stash
                        let good = &info.good;
                        let current_amount = state.stash.get_good(good);
                        queue!(writer, StashWithdrawInput(info, current_amount))?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let KeyCode::Char(c) = event.code {
                                if c == 'b' {
                                    state.back()?;
                                } else if let Some(digit) = c.to_digit(10) {
                                    state.user_typed_digit(digit)?;
                                }
                            } else if event.code == KeyCode::Backspace {
                                state.user_typed_backspace()?;
                            } else if event.code == KeyCode::Enter {
                                return state
                                    .commit_stash_withdraw()
                                    .map(|_| UpdateSignal::Continue)
                                    .or_else(|e| match e {
                                        StateError::InsufficientStash => Ok(UpdateSignal::Continue),
                                        x => Err(x.into()),
                                    });
                            }
                            Ok(UpdateSignal::Continue)
                        }));
                    } else {
                        // user is choosing which good to withdraw from stash
                        queue!(writer, StashWithdrawPrompt)?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let KeyCode::Char('b') = event.code {
                                state.back()?;
                            } else if let Some(good) = Good::from_key_code(&event.code) {
                                state.choose_stash_withdraw_good(good)?;
                            } else if event.code == KeyCode::Backspace {
                                state.cancel_stash_withdraw()?;
                            }
                            Ok(UpdateSignal::Continue)
                        }));
                    }
                }
                Mode::PayDebt(amount) => {
                    queue!(writer, PayDebtInput(amount))?;
                    return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                        if let KeyCode::Char(c) = event.code {
                            if c == 'b' {
                                state.back()?;
                            } else if let Some(digit) = c.to_digit(10) {
                                state.user_typed_digit(digit)?;
                            }
                        } else if event.code == KeyCode::Backspace {
                            state.user_typed_backspace()?;
                        } else if event.code == KeyCode::Enter {
                            return state
                                .commit_pay_debt()
                                .map(|_| UpdateSignal::Continue)
                                .or_else(|e| match e {
                                    StateError::PayDownAmountHigherThanDebt => {
                                        Ok(UpdateSignal::Continue)
                                    }
                                    StateError::CannotAfford => Ok(UpdateSignal::Continue),
                                    x => Err(x.into()),
                                });
                        }
                        Ok(UpdateSignal::Continue)
                    }));
                }
                Mode::BankDeposit(amount) => {
                    queue!(writer, BankDepositInput(amount))?;
                    return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                        if let KeyCode::Char(c) = event.code {
                            if c == 'b' {
                                state.back()?;
                            } else if let Some(digit) = c.to_digit(10) {
                                state.user_typed_digit(digit)?;
                            }
                        } else if event.code == KeyCode::Backspace {
                            state.user_typed_backspace()?;
                        } else if event.code == KeyCode::Enter {
                            return state
                                .commit_bank_deposit()
                                .map(|_| UpdateSignal::Continue)
                                .or_else(|e| match e {
                                    StateError::CannotAfford => Ok(UpdateSignal::Continue),
                                    x => Err(x.into()),
                                });
                        }
                        Ok(UpdateSignal::Continue)
                    }));
                }
                Mode::BankWithdraw(amount) => {
                    queue!(writer, BankWithdrawInput(amount))?;
                    return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                        if let KeyCode::Char(c) = event.code {
                            if c == 'b' {
                                state.back()?;
                            } else if let Some(digit) = c.to_digit(10) {
                                state.user_typed_digit(digit)?;
                            }
                        } else if event.code == KeyCode::Backspace {
                            state.user_typed_backspace()?;
                        } else if event.code == KeyCode::Enter {
                            return state
                                .commit_bank_withdraw()
                                .map(|_| UpdateSignal::Continue)
                                .or_else(|e| match e {
                                    StateError::InsufficientBank => Ok(UpdateSignal::Continue),
                                    x => Err(x.into()),
                                });
                        }
                        Ok(UpdateSignal::Continue)
                    }));
                }
                Mode::GameEvent(event) => match event {
                    LocationEvent::CheapGood(good) => {
                        queue!(writer, CheapGoodDialog(good))?;
                        return Ok(Box::new(|_: KeyEvent, state: &mut GameState| {
                            state.acknowledge_event()?;
                            Ok(UpdateSignal::Continue)
                        }));
                    }
                    LocationEvent::ExpensiveGood(good) => {
                        queue!(writer, ExpensiveGoodDialog(good))?;
                        return Ok(Box::new(|_: KeyEvent, state: &mut GameState| {
                            state.acknowledge_event()?;
                            Ok(UpdateSignal::Continue)
                        }));
                    }
                    LocationEvent::FindGoods(good, amount) => {
                        queue!(writer, FindGoodsDialog(good, amount, state))?;
                        let good = *good;
                        let amount = *amount;
                        return Ok(Box::new(move |_: KeyEvent, state: &mut GameState| {
                            {
                                let remaining_hold = state.remaining_hold();
                                let amount_to_add = min(amount, remaining_hold);
                                state.inventory.add_good(&good, amount_to_add);
                            }
                            state.acknowledge_event()?;
                            Ok(UpdateSignal::Continue)
                        }));
                    }
                    LocationEvent::GoodsStolen(info) => {
                        let info = info.unwrap_or_else(|| state.compute_goods_stolen());
                        queue!(writer, GoodsStolenDialog(info))?;
                        return Ok(Box::new(move |_: KeyEvent, state: &mut GameState| {
                            state.remove_stolen_goods(info);
                            state.acknowledge_event()?;
                            Ok(UpdateSignal::Continue)
                        }));
                    }
                    LocationEvent::CanBuyCannon => {
                        queue!(writer, CanBuyCannon)?;
                        return Ok(Box::new(move |event: KeyEvent, state: &mut GameState| {
                            if let KeyCode::Char(c) = event.code {
                                if c == 'y' {
                                    state.confirm_buy_cannon()?;
                                } else if c == 'n' {
                                    state.acknowledge_event()?;
                                }
                            }
                            Ok(UpdateSignal::Continue)
                        }));
                    }
                    // TODO<samgqroberts> 2024-04-10 use actual error type
                    LocationEvent::PirateEncounter(_) => panic!("Cannot encounter pirates here"),
                    LocationEvent::CanBuyHoldSpace { price, more_hold } => {
                        let price = *price;
                        let more_hold = *more_hold;
                        queue!(writer, CanBuyHoldSpace { price, more_hold })?;
                        return Ok(Box::new(move |event: KeyEvent, state: &mut GameState| {
                            if let KeyCode::Char(c) = event.code {
                                if c == 'y' {
                                    state.confirm_buy_hold_space(price, more_hold)?;
                                } else if c == 'n' {
                                    state.acknowledge_event()?;
                                }
                            }
                            Ok(UpdateSignal::Continue)
                        }));
                    }
                    LocationEvent::NoEffect(variant) => {
                        queue!(writer, NoEffect { variant: *variant })?;
                        return Ok(Box::new(move |_: KeyEvent, state: &mut GameState| {
                            state.acknowledge_event()?;
                            Ok(UpdateSignal::Continue)
                        }));
                    }
                },
            }
        }
    }

    pub fn draw_scene(&mut self, state: &mut GameState) -> io::Result<Box<UpdateFn>> {
        info!("Drawing scene: {:?}", state.mode);
        let writer = &mut *self.writer.borrow_mut();
        let update = Engine::queue_scene(writer, state)?;
        writer.flush()?;
        Ok(update)
    }

    pub fn draw_need_resize(&mut self, current_x_cols: u16, current_y_cols: u16) -> io::Result<()> {
        info!("Drawing screen requiring resize");
        let writer = &mut *self.writer.borrow_mut();
        queue!(
            writer,
            RequireResize {
                current_x_cols,
                current_y_cols
            }
        )?;
        writer.flush()?;
        Ok(())
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
