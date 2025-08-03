use ansi_commands::{
    comp,
    event::{KeyCode, KeyEvent},
};
use std::{
    cmp::min,
    io::{self},
};

use crate::{
    components::{
        BankDepositInput, BankWithdrawInput, BuyInput, BuyPrompt, CanBuyCannon, CanBuyHoldSpace,
        CheapGoodDialog, ExpensiveGoodDialog, FindGoodsDialog, GameEndScreen, GoodsStolenDialog,
        IntroductionScreen, NoEffect, PayDebtInput, PirateEncounter, SailPrompt, SellInput,
        SellPrompt, SplashScreen, StashDepositInput, StashDepositPrompt, StashWithdrawInput,
        StashWithdrawPrompt, ViewingInventoryActions, ViewingInventoryBase,
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

pub fn render_scene_to_existing(
    frame: &mut ansi_commands::frame::Frame,
    state: &mut GameState,
) -> Result<Box<UpdateFn>, String> {
    if state.initialization == Initialization::SplashScreen {
        // initial splash screen
        comp!(frame, SplashScreen())?;
        Ok(Box::new(|_: KeyEvent, state: &mut GameState| {
            state.splash_to_introduction();
            Ok(UpdateSignal::Continue)
        }))
    } else if state.game_end {
        comp!(frame, GameEndScreen(state))?;
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
        comp!(
            frame,
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
        comp!(
            frame,
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
        comp!(frame, ViewingInventoryBase(state))?;
        match &state.mode {
            Mode::ViewingInventory => {
                comp!(
                    frame,
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
                    comp!(frame, BuyInput { info, state })?;
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
                    comp!(frame, BuyPrompt)?;
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
                    comp!(frame, SellInput(info, current_amount))?;
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
                            return state.commit_sell().map(|_| UpdateSignal::Continue).or_else(
                                |e| match e {
                                    StateError::InsufficientInventory => Ok(UpdateSignal::Continue),
                                    x => Err(x.into()),
                                },
                            );
                        }
                        Ok(UpdateSignal::Continue)
                    }));
                } else {
                    // user is choosing which good to sell
                    comp!(frame, SellPrompt)?;
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
                comp!(frame, SailPrompt)?;
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
                    comp!(frame, StashDepositInput(info, current_amount))?;
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
                                    StateError::InsufficientInventory => Ok(UpdateSignal::Continue),
                                    x => Err(x.into()),
                                });
                        }
                        Ok(UpdateSignal::Continue)
                    }));
                } else {
                    // user is choosing which good to stash
                    comp!(frame, StashDepositPrompt)?;
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
                    comp!(frame, StashWithdrawInput(info, current_amount))?;
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
                    comp!(frame, StashWithdrawPrompt)?;
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
                comp!(frame, PayDebtInput(amount))?;
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
                comp!(frame, BankDepositInput(amount))?;
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
                comp!(frame, BankWithdrawInput(amount))?;
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
                    comp!(frame, CheapGoodDialog(good))?;
                    return Ok(Box::new(|_: KeyEvent, state: &mut GameState| {
                        state.acknowledge_event()?;
                        Ok(UpdateSignal::Continue)
                    }));
                }
                LocationEvent::ExpensiveGood(good) => {
                    comp!(frame, ExpensiveGoodDialog(good))?;
                    return Ok(Box::new(|_: KeyEvent, state: &mut GameState| {
                        state.acknowledge_event()?;
                        Ok(UpdateSignal::Continue)
                    }));
                }
                LocationEvent::FindGoods(good, amount) => {
                    comp!(frame, FindGoodsDialog(good, amount, state))?;
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
                    comp!(frame, GoodsStolenDialog(info))?;
                    return Ok(Box::new(move |_: KeyEvent, state: &mut GameState| {
                        state.remove_stolen_goods(info);
                        state.acknowledge_event()?;
                        Ok(UpdateSignal::Continue)
                    }));
                }
                LocationEvent::CanBuyCannon => {
                    comp!(frame, CanBuyCannon)?;
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
                    comp!(frame, CanBuyHoldSpace { price, more_hold })?;
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
                    comp!(frame, NoEffect { variant: *variant })?;
                    return Ok(Box::new(move |_: KeyEvent, state: &mut GameState| {
                        state.acknowledge_event()?;
                        Ok(UpdateSignal::Continue)
                    }));
                }
            },
        }
    }
}

pub fn render_scene(
    state: &mut GameState,
) -> Result<(ansi_commands::frame::Frame, Box<UpdateFn>), String> {
    let mut frame = ansi_commands::frame::Frame::new();
    let update = render_scene_to_existing(&mut frame, state)?;
    Ok((frame, update))
}
