use crossterm::{
    cursor::{MoveToNextLine, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::Print,
};
use std::{
    cell::RefCell,
    cmp::min,
    io::{self, Write},
    time::Duration,
};

use crate::{
    components::{
        BankDepositInput, BankWithdrawInput, BuyInput, BuyPrompt, CanBuyCannon, CheapGoodDialog,
        ExpensiveGoodDialog, FindGoodsDialog, GameEndScreen, GoodsStolenDialog, PayDebtInput,
        PirateEncounter, SailPrompt, SellInput, SellPrompt, SplashScreen, StashDepositInput,
        StashDepositPrompt, StashWithdrawInput, StashWithdrawPrompt, ViewingInventoryActions,
        ViewingInventoryBase,
    },
    state::{GameState, Good, Location, LocationEvent, Mode, PirateEncounterState, StateError},
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

pub type UpdateResult<T> = Result<T, UpdateError>;

pub type UpdateFn = dyn FnOnce(KeyEvent, &mut GameState) -> UpdateResult<()>;

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
                '1' => Some(Good::Tea),
                '2' => Some(Good::Coffee),
                '3' => Some(Good::Sugar),
                '4' => Some(Good::Tobacco),
                '5' => Some(Good::Rum),
                '6' => Some(Good::Cotton),
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

    pub fn draw_and_prompt(&mut self, game_state: &mut GameState) -> Result<bool, UpdateError> {
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
                            return Ok(true);
                        }
                        // update game state
                        update_fn(event, game_state)?;
                        // indicate we do not want to exit
                        return Ok(false);
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
        if !state.initialized {
            // initial splash screen
            queue!(writer, SplashScreen())?;
            Ok(Box::new(|_: KeyEvent, state: &mut GameState| {
                state.initialize();
                Ok(())
            }))
        } else if state.game_end {
            queue!(writer, GameEndScreen(state))?;
            return Ok(Box::new(|_: KeyEvent, _: &mut GameState| Ok(())));
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
                Ok(())
            }));
        } else {
            queue!(writer, ViewingInventoryBase(state))?;
            match &state.mode {
                Mode::ViewingInventory => {
                    queue!(
                        writer,
                        ViewingInventoryActions {
                            location: &state.location,
                            debt: state.debt
                        }
                    )?;
                    return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                        if let KeyCode::Char(ch) = event.code {
                            if ch == '1' {
                                state.begin_buying()?;
                            } else if ch == '2' {
                                state.begin_selling()?;
                            } else if ch == '3' {
                                state.begin_sailing()?;
                            };
                            if state.location == Location::London {
                                if ch == '4' {
                                    state.begin_stash_deposit()?;
                                } else if ch == '5' {
                                    state.begin_stash_withdraw()?;
                                } else if ch == '6' {
                                    state.begin_bank_deposit()?;
                                } else if ch == '7' {
                                    state.begin_bank_withdraw()?;
                                }
                                if state.debt > 0 && ch == '8' {
                                    state.begin_pay_debt()?;
                                }
                            }
                        }
                        Ok(())
                    }));
                }
                Mode::Buying(info) => {
                    if let Some(info) = info {
                        queue!(writer, BuyInput { info, state })?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let KeyCode::Char(c) = event.code {
                                if let Some(digit) = c.to_digit(10) {
                                    state.user_typed_digit(digit)?;
                                }
                            } else if event.code == KeyCode::Backspace {
                                state.user_typed_backspace()?;
                            } else if event.code == KeyCode::Enter {
                                return state.commit_buy().map(|_| ()).or_else(|e| match e {
                                    StateError::CannotAfford | StateError::InsufficientHold => {
                                        Ok(())
                                    }
                                    x => Err(x.into()),
                                });
                            }
                            Ok(())
                        }));
                    } else {
                        queue!(writer, BuyPrompt)?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let Some(good) = Good::from_key_code(&event.code) {
                                state.choose_buy_good(good)?;
                            } else if event.code == KeyCode::Backspace {
                                state.cancel_buy()?;
                            }
                            Ok(())
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
                                if let Some(digit) = c.to_digit(10) {
                                    state.user_typed_digit(digit)?;
                                }
                            } else if event.code == KeyCode::Backspace {
                                state.user_typed_backspace()?;
                            } else if event.code == KeyCode::Enter {
                                return state.commit_sell().map(|_| ()).or_else(|e| match e {
                                    StateError::InsufficientInventory => Ok(()),
                                    x => Err(x.into()),
                                });
                            }
                            Ok(())
                        }));
                    } else {
                        // user is choosing which good to sell
                        queue!(writer, SellPrompt)?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let Some(good) = Good::from_key_code(&event.code) {
                                state.choose_sell_good(good)?;
                            } else if event.code == KeyCode::Backspace {
                                state.cancel_sell()?;
                            }
                            Ok(())
                        }));
                    }
                }
                Mode::Sailing => {
                    // user is choosing where to sail
                    queue!(writer, SailPrompt)?;
                    return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                        if let Some(destination) = Location::from_key_code(&event.code) {
                            return state
                                .sail_to(&destination)
                                .map(|_| ())
                                .or_else(|e| match e {
                                    StateError::AlreadyInLocation => Ok(()),
                                    x => Err(x.into()),
                                });
                        } else if event.code == KeyCode::Backspace {
                            state.cancel_sail_to()?;
                        }
                        Ok(())
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
                                if let Some(digit) = c.to_digit(10) {
                                    state.user_typed_digit(digit)?;
                                }
                            }
                            if event.code == KeyCode::Backspace {
                                state.user_typed_backspace()?;
                            }
                            if event.code == KeyCode::Enter {
                                return state.commit_stash_deposit().map(|_| ()).or_else(
                                    |e| match e {
                                        StateError::InsufficientInventory => Ok(()),
                                        x => Err(x.into()),
                                    },
                                );
                            }
                            Ok(())
                        }));
                    } else {
                        // user is choosing which good to stash
                        queue!(writer, StashDepositPrompt)?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let Some(good) = Good::from_key_code(&event.code) {
                                state.choose_stash_deposit_good(good)?;
                            } else if event.code == KeyCode::Backspace {
                                state.cancel_stash_deposit()?;
                            }
                            Ok(())
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
                                if let Some(digit) = c.to_digit(10) {
                                    state.user_typed_digit(digit)?;
                                }
                            } else if event.code == KeyCode::Backspace {
                                state.user_typed_backspace()?;
                            } else if event.code == KeyCode::Enter {
                                return state.commit_stash_withdraw().map(|_| ()).or_else(
                                    |e| match e {
                                        StateError::InsufficientStash => Ok(()),
                                        x => Err(x.into()),
                                    },
                                );
                            }
                            Ok(())
                        }));
                    } else {
                        // user is choosing which good to withdraw from stash
                        queue!(writer, StashWithdrawPrompt)?;
                        return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                            if let Some(good) = Good::from_key_code(&event.code) {
                                state.choose_stash_withdraw_good(good)?;
                            } else if event.code == KeyCode::Backspace {
                                state.cancel_stash_withdraw()?;
                            }
                            Ok(())
                        }));
                    }
                }
                Mode::PayDebt(amount) => {
                    queue!(writer, PayDebtInput(amount))?;
                    return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                        if let KeyCode::Char(c) = event.code {
                            if let Some(digit) = c.to_digit(10) {
                                state.user_typed_digit(digit)?;
                            }
                        } else if event.code == KeyCode::Backspace {
                            state.user_typed_backspace()?;
                        } else if event.code == KeyCode::Enter {
                            return state.commit_pay_debt().map(|_| ()).or_else(|e| match e {
                                StateError::PayDownAmountHigherThanDebt => Ok(()),
                                StateError::CannotAfford => Ok(()),
                                x => Err(x.into()),
                            });
                        }
                        Ok(())
                    }));
                }
                Mode::BankDeposit(amount) => {
                    queue!(writer, BankDepositInput(amount))?;
                    return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                        if let KeyCode::Char(c) = event.code {
                            if let Some(digit) = c.to_digit(10) {
                                state.user_typed_digit(digit)?;
                            }
                        } else if event.code == KeyCode::Backspace {
                            state.user_typed_backspace()?;
                        } else if event.code == KeyCode::Enter {
                            return state
                                .commit_bank_deposit()
                                .map(|_| ())
                                .or_else(|e| match e {
                                    StateError::CannotAfford => Ok(()),
                                    x => Err(x.into()),
                                });
                        }
                        Ok(())
                    }));
                }
                Mode::BankWithdraw(amount) => {
                    queue!(writer, BankWithdrawInput(amount))?;
                    return Ok(Box::new(|event: KeyEvent, state: &mut GameState| {
                        if let KeyCode::Char(c) = event.code {
                            if let Some(digit) = c.to_digit(10) {
                                state.user_typed_digit(digit)?;
                            }
                        } else if event.code == KeyCode::Backspace {
                            state.user_typed_backspace()?;
                        } else if event.code == KeyCode::Enter {
                            return state
                                .commit_bank_withdraw()
                                .map(|_| ())
                                .or_else(|e| match e {
                                    StateError::InsufficientBank => Ok(()),
                                    x => Err(x.into()),
                                });
                        }
                        Ok(())
                    }));
                }
                Mode::GameEvent(event) => match event {
                    LocationEvent::CheapGood(good) => {
                        queue!(writer, CheapGoodDialog(good))?;
                        return Ok(Box::new(|_: KeyEvent, state: &mut GameState| {
                            state.acknowledge_event()?;
                            Ok(())
                        }));
                    }
                    LocationEvent::ExpensiveGood(good) => {
                        queue!(writer, ExpensiveGoodDialog(good))?;
                        return Ok(Box::new(|_: KeyEvent, state: &mut GameState| {
                            state.acknowledge_event()?;
                            Ok(())
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
                            Ok(())
                        }));
                    }
                    LocationEvent::GoodsStolen(info) => {
                        let info = info.unwrap_or_else(|| state.compute_goods_stolen());
                        queue!(writer, GoodsStolenDialog(info))?;
                        return Ok(Box::new(move |_: KeyEvent, state: &mut GameState| {
                            state.remove_stolen_goods(info);
                            state.acknowledge_event()?;
                            Ok(())
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
                            Ok(())
                        }));
                    }
                    _ => {
                        // TODO<samgqroberts> 2024-04-10 return Error type.
                        panic!("Unknown location event.");
                    }
                },
            }
        }
    }

    pub fn draw_scene(&mut self, state: &mut GameState) -> io::Result<Box<UpdateFn>> {
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
