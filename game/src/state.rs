pub mod good;
pub mod inventory;
pub mod location;
pub mod locations;

use std::borrow::BorrowMut;
use std::fmt::{self, Display};

use chrono::Month;
use rand::distributions::{Distribution, WeightedIndex};
use rand::rngs::StdRng;
use rand::Rng;

pub use self::good::Good;
pub use self::inventory::Inventory;
pub use self::location::Location;
pub use self::locations::Locations;

#[derive(PartialEq, Clone, Debug)]
pub struct Transaction {
    pub good: Good,
    pub amount: Option<u32>,
}

pub const CANNON_COST: u16 = 5000;
pub const SHIP_HEALTH: u8 = 5;
pub const GOLD_PER_PIRATE_VICTORY_MIN: u32 = 500;
pub const GOLD_PER_PIRATE_VICTORY_MAX: u32 = 2000;

#[derive(PartialEq, Clone, Debug, Copy)]
pub struct PirateEncounterInfo {
    pub health: u8,
    pub total_pirates: u8,
    pub cur_pirates: u8,
}

impl PirateEncounterInfo {
    pub fn new(pirates: u8) -> PirateEncounterInfo {
        PirateEncounterInfo {
            health: SHIP_HEALTH,
            total_pirates: pirates,
            cur_pirates: pirates,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Copy)]
pub enum PirateEncounterState {
    Initial,
    Prompt {
        info: PirateEncounterInfo,
    },
    AttackResult {
        info: PirateEncounterInfo,
        did_kill_a_pirate: bool,
    },
    RunSuccess,
    RunFailure {
        info: PirateEncounterInfo,
    },
    PiratesAttack {
        info: PirateEncounterInfo,
        damage_this_attack: u8,
    },
    Destroyed,
    Victory {
        gold_recovered: u32,
    },
}

impl PirateEncounterState {
    pub fn pirates_attack(
        info: PirateEncounterInfo,
        rng: &mut StdRng,
    ) -> Result<PirateEncounterState, StateError> {
        // make lower damages more likely.
        // eg. the damage possibilities when there are 3 pirates will be [0, 1, 2, 3]
        // and the weights corresponding to those damage possibilities are [4, 3, 2, 1]
        let damage_possibilities: Vec<u8> = (0..=info.cur_pirates).collect();
        let weights: Vec<usize> = damage_possibilities
            .iter()
            .enumerate()
            .map(|(i, _)| i + 1)
            .rev()
            .collect();
        let dist =
            WeightedIndex::new(&weights).map_err(|e| StateError::UnknownError(e.to_string()))?;
        let damage_this_attack = damage_possibilities[dist.sample(rng)];
        Ok(PirateEncounterState::PiratesAttack {
            info,
            damage_this_attack,
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum LocationEvent {
    CheapGood(Good),
    ExpensiveGood(Good),
    FindGoods(Good, u32),
    GoodsStolen(Option<GoodsStolenResult>),
    CanBuyCannon,
    PirateEncounter(PirateEncounterState),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Mode {
    ViewingInventory,
    Buying(Option<Transaction>),
    Selling(Option<Transaction>),
    Sailing,
    StashDeposit(Option<Transaction>),
    StashWithdraw(Option<Transaction>),
    PayDebt(Option<u32>),
    BankDeposit(Option<u32>),
    BankWithdraw(Option<u32>),
    GameEvent(LocationEvent),
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub rng: StdRng,
    pub initialized: bool,
    pub date: (u16, Month),
    pub hold_size: u32,
    pub cannons: u8,
    pub gold: u32,
    pub bank: u32,
    pub location: Location,
    pub stash: Inventory,
    pub inventory: Inventory,
    pub locations: Locations,
    pub debt: u32,
    pub mode: Mode,
    pub game_end: bool,
}

impl GameState {
    pub fn new(mut rng: StdRng) -> GameState {
        let starting_gold = 500;
        let debt = starting_gold * 3;
        let locations = Locations::new(&mut rng, starting_gold);
        GameState {
            rng,
            initialized: false,
            date: (1782, Month::March),
            hold_size: 100,
            cannons: 1,
            gold: starting_gold,
            bank: 0,
            location: Location::London, // home base
            stash: Inventory::default(),
            inventory: Inventory::default(),
            locations,
            debt,
            mode: Mode::ViewingInventory,
            game_end: false,
        }
    }

    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    fn require_viewing_inventory(&self) -> Result<(), StateError> {
        if self.mode != Mode::ViewingInventory {
            Err(StateError::InvalidMode(self.mode.clone()))
        } else {
            Ok(())
        }
    }

    fn require_location_home_base(&self) -> Result<(), StateError> {
        if self.location != Location::London {
            Err(StateError::LocationNotHomeBase(self.location.clone()))
        } else {
            Ok(())
        }
    }

    pub fn begin_buying(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.mode = Mode::Buying(None);
        Ok(self)
    }

    pub fn begin_selling(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.mode = Mode::Selling(None);
        Ok(self)
    }

    pub fn begin_sailing(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.mode = Mode::Sailing;
        Ok(self)
    }

    pub fn begin_stash_deposit(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        self.mode = Mode::StashDeposit(None);
        Ok(self)
    }

    pub fn begin_stash_withdraw(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        self.mode = Mode::StashWithdraw(None);
        Ok(self)
    }

    pub fn begin_pay_debt(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        self.mode = Mode::PayDebt(None);
        Ok(self)
    }

    pub fn begin_bank_deposit(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        self.mode = Mode::BankDeposit(None);
        Ok(self)
    }

    pub fn begin_bank_withdraw(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        self.mode = Mode::BankWithdraw(None);
        Ok(self)
    }

    pub fn choose_buy_good(&mut self, good: Good) -> Result<&mut GameState, StateError> {
        if let Mode::Buying(None) = &self.mode {
            self.mode = Mode::Buying(Some(Transaction { good, amount: None }));
            return Ok(self);
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn choose_sell_good(&mut self, good: Good) -> Result<&mut GameState, StateError> {
        if let Mode::Selling(None) = &self.mode {
            self.mode = Mode::Selling(Some(Transaction { good, amount: None }));
            return Ok(self);
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn choose_stash_deposit_good(&mut self, good: Good) -> Result<&mut GameState, StateError> {
        if let Mode::StashDeposit(None) = &self.mode {
            self.mode = Mode::StashDeposit(Some(Transaction { good, amount: None }));
            return Ok(self);
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn choose_stash_withdraw_good(&mut self, good: Good) -> Result<&mut GameState, StateError> {
        if let Mode::StashWithdraw(None) = &self.mode {
            self.mode = Mode::StashWithdraw(Some(Transaction { good, amount: None }));
            return Ok(self);
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn cancel_buy(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::Buying(None) = &self.mode {
            self.mode = Mode::ViewingInventory;
            return Ok(self);
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn cancel_sell(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::Selling(None) = &self.mode {
            self.mode = Mode::ViewingInventory;
            return Ok(self);
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn cancel_stash_deposit(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::StashDeposit(None) = &self.mode {
            self.mode = Mode::ViewingInventory;
            return Ok(self);
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn cancel_stash_withdraw(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::StashWithdraw(None) = &self.mode {
            self.mode = Mode::ViewingInventory;
            return Ok(self);
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn user_typed_digit(&mut self, digit: u32) -> Result<&mut GameState, StateError> {
        let mut binding = None;
        let info = match &mut self.mode {
            Mode::Buying(info) => info,
            Mode::Selling(info) => info,
            Mode::StashDeposit(info) => info,
            Mode::StashWithdraw(info) => info,
            _ => &mut binding,
        };
        if let Some(info) = info {
            info.amount = Some(info.amount.map_or(digit, |amount| amount * 10 + digit));
            Ok(self)
        } else {
            let binding: Option<&mut Option<u32>> = None;
            let amount = match &mut self.mode {
                Mode::PayDebt(amount) => Some(amount),
                Mode::BankDeposit(amount) => Some(amount),
                Mode::BankWithdraw(amount) => Some(amount),
                _ => binding,
            };
            if let Some(amount) = amount {
                *amount = amount.map_or(Some(digit), |amount| Some(amount * 10 + digit));
                Ok(self)
            } else {
                Err(StateError::InvalidMode(self.mode.clone()))
            }
        }
    }

    pub fn user_typed_backspace(&mut self) -> Result<&mut GameState, StateError> {
        let mut binding = None;
        let info = match &mut self.mode {
            Mode::Buying(info) => info,
            Mode::Selling(info) => info,
            Mode::StashDeposit(info) => info,
            Mode::StashWithdraw(info) => info,
            _ => &mut binding,
        };
        if let Some(info) = info {
            info.amount = info
                .amount
                .and_then(|amount| if amount <= 9 { None } else { Some(amount / 10) });
            Ok(self)
        } else {
            let binding: Option<&mut Option<u32>> = None;
            let amount = match &mut self.mode {
                Mode::PayDebt(amount) => Some(amount),
                Mode::BankDeposit(amount) => Some(amount),
                Mode::BankWithdraw(amount) => Some(amount),
                _ => binding,
            };
            if let Some(amount) = amount {
                *amount =
                    amount.and_then(|amount| if amount <= 9 { None } else { Some(amount / 10) });
                Ok(self)
            } else {
                Err(StateError::InvalidMode(self.mode.clone()))
            }
        }
    }

    pub fn commit_buy(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::Buying(Some(info)) = &self.mode {
            let amount = info.amount.unwrap_or(0);
            let good_price = self
                .locations
                .location_info(&self.location)
                .prices
                .get_good(&info.good);
            let can_afford = self.gold / good_price;
            if amount > can_afford {
                return Err(StateError::CannotAfford);
            } else {
                let hold_size = self.hold_size;
                let current_hold = self.inventory.total_amount();
                if current_hold + amount > hold_size {
                    return Err(StateError::InsufficientHold);
                } else {
                    self.inventory.add_good(&info.good, amount);
                    self.gold -= good_price * amount;
                    self.mode = Mode::ViewingInventory;
                    return Ok(self);
                }
            }
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn commit_sell(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::Selling(Some(info)) = &self.mode {
            let amount = info.amount.unwrap_or(0);
            let good_price = self
                .locations
                .location_info(&self.location)
                .prices
                .get_good(&info.good);
            let user_amount = self.inventory.get_good(&info.good);
            if &amount > user_amount {
                return Err(StateError::InsufficientInventory);
            } else {
                self.inventory.remove_good(&info.good, amount);
                self.gold += good_price * amount;
                self.mode = Mode::ViewingInventory;
                return Ok(self);
            }
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn commit_stash_deposit(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::StashDeposit(Some(info)) = &self.mode {
            let amount = info.amount.unwrap_or(0);
            let user_amount = self.inventory.get_good(&info.good);
            if &amount > user_amount {
                return Err(StateError::InsufficientInventory);
            } else {
                self.inventory.remove_good(&info.good, amount);
                self.stash.add_good(&info.good, amount);
                self.mode = Mode::ViewingInventory;
                return Ok(self);
            }
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn commit_stash_withdraw(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::StashWithdraw(Some(info)) = &self.mode {
            let amount = info.amount.unwrap_or(0);
            let stash_amount = self.stash.get_good(&info.good);
            if &amount > stash_amount {
                return Err(StateError::InsufficientStash);
            } else {
                self.stash.remove_good(&info.good, amount);
                self.inventory.add_good(&info.good, amount);
                self.mode = Mode::ViewingInventory;
                return Ok(self);
            }
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn commit_pay_debt(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::PayDebt(amount) = &self.mode {
            let amount = &amount.unwrap_or(0);
            if amount > &self.debt {
                return Err(StateError::PayDownAmountHigherThanDebt);
            }
            if amount > &self.gold {
                return Err(StateError::CannotAfford);
            }
            self.debt -= amount;
            self.gold -= amount;
            self.mode = Mode::ViewingInventory;
            return Ok(self);
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn commit_bank_deposit(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::BankDeposit(amount) = &self.mode {
            let amount = &amount.unwrap_or(0);
            if amount > &self.gold {
                return Err(StateError::CannotAfford);
            }
            self.gold -= amount;
            self.bank += amount;
            self.mode = Mode::ViewingInventory;
            return Ok(self);
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn commit_bank_withdraw(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::BankWithdraw(amount) = &self.mode {
            let amount = &amount.unwrap_or(0);
            if amount > &self.bank {
                return Err(StateError::InsufficientBank);
            }
            self.gold += amount;
            self.bank -= amount;
            self.mode = Mode::ViewingInventory;
            return Ok(self);
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn sail_to(&mut self, destination: &Location) -> Result<&mut GameState, StateError> {
        if let Mode::Sailing = self.mode {
            if destination == &self.location {
                Err(StateError::AlreadyInLocation)
            } else {
                self.mode = Mode::ViewingInventory;
                // increment the month
                self.date.1 = self.date.1.succ();
                if self.date.1 == Month::January {
                    self.date.0 += 1;
                }
                if self.date.0 == 1785 && self.date.1 == Month::March {
                    // 3 years have elapsed
                    // end the game
                    self.game_end = true
                }
                // update location info for location we just left
                let new_location_info =
                    self.locations
                        .generate_location(&mut self.rng, destination, true);
                // set current location
                self.location = destination.clone();
                // increment debt, if any
                let new_debt = f64::from(self.debt) * 1.1;
                self.debt = new_debt.floor() as u32;
                // determine if we've encountered an event
                if let Some(event) = &new_location_info.event {
                    self.mode = Mode::GameEvent(event.clone());
                }
                Ok(self)
            }
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }

    pub fn cancel_sail_to(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::Sailing = self.mode {
            self.mode = Mode::ViewingInventory;
            Ok(self)
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }

    pub fn acknowledge_event(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::GameEvent(_) = self.mode {
            self.mode = Mode::ViewingInventory;
            Ok(self)
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }

    pub fn remaining_hold(&self) -> u32 {
        self.hold_size.saturating_sub(self.inventory.total_amount())
    }

    pub(crate) fn compute_goods_stolen(&mut self) -> GoodsStolenResult {
        if let Mode::GameEvent(event) = &mut self.mode {
            if let LocationEvent::GoodsStolen(info) = event.borrow_mut() {
                if let Some(info) = info {
                    return *info;
                } else {
                    // randomly select a good that we have inventory of
                    let goods_with_inventory = self
                        .inventory
                        .iter()
                        .filter(|x| x.1 > 0)
                        .collect::<Vec<(Good, u32)>>();
                    let computed_info = if goods_with_inventory.is_empty() {
                        GoodsStolenResult::NothingStolen
                    } else {
                        let index = self.rng.gen_range(0..goods_with_inventory.len());
                        // safe unwrap, we generated the index to be in range
                        let good_to_steal = goods_with_inventory.get(index).unwrap();
                        // choose some amount of good to steal
                        let amount = self.rng.gen_range(1..good_to_steal.1);
                        GoodsStolenResult::WasStolen {
                            good: good_to_steal.0,
                            amount,
                        }
                    };
                    *info = Some(computed_info);
                    return computed_info;
                }
            }
        }
        GoodsStolenResult::NothingStolen
    }

    pub(crate) fn remove_stolen_goods(&mut self, goods_stolen_info: GoodsStolenResult) {
        if let GoodsStolenResult::WasStolen { good, amount } = goods_stolen_info {
            self.inventory.remove_good(&good, amount);
        }
    }

    pub(crate) fn confirm_buy_cannon(&mut self) -> Result<(), StateError> {
        if self.gold >= CANNON_COST.into() {
            self.gold = self.gold - (CANNON_COST as u32);
            self.cannons = self.cannons + 1;
            self.acknowledge_event()?;
        }
        Ok(())
    }

    pub(crate) fn proceed_pirate_encounter(&mut self) -> Result<(), StateError> {
        if let Mode::GameEvent(LocationEvent::PirateEncounter(PirateEncounterState::Initial)) =
            self.mode
        {
            let pirates = self.rng.gen_range(2..=4);
            self.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
                PirateEncounterState::Prompt {
                    info: PirateEncounterInfo::new(pirates),
                },
            ));
            Ok(())
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }

    pub(crate) fn pirate_run(&mut self) -> Result<(), StateError> {
        if let Mode::GameEvent(LocationEvent::PirateEncounter(PirateEncounterState::Prompt {
            info,
        })) = self.mode
        {
            let run_success_chance = logarithmic_decay(info.cur_pirates as u32, 0.5);
            let random_value: f64 = self.rng.gen();
            if random_value < run_success_chance {
                // success
                self.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
                    PirateEncounterState::RunSuccess,
                ));
            } else {
                // failure
                self.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
                    PirateEncounterState::RunFailure { info },
                ));
            }
            Ok(())
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }

    pub(crate) fn pirate_fight(&mut self) -> Result<(), StateError> {
        if let Mode::GameEvent(LocationEvent::PirateEncounter(PirateEncounterState::Prompt {
            info,
        })) = self.mode
        {
            let kill_a_pirate_possibilities = [false, true];
            let weights = [1, self.cannons];
            let dist = WeightedIndex::new(&weights)
                .map_err(|e| StateError::UnknownError(e.to_string()))?;
            let did_kill_a_pirate = kill_a_pirate_possibilities[dist.sample(&mut self.rng)];
            self.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
                PirateEncounterState::AttackResult {
                    info,
                    did_kill_a_pirate,
                },
            ));
            Ok(())
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }

    pub(crate) fn proceed_pirate_run_failure(&mut self) -> Result<(), StateError> {
        if let Mode::GameEvent(LocationEvent::PirateEncounter(PirateEncounterState::RunFailure {
            info,
        })) = self.mode
        {
            self.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
                PirateEncounterState::pirates_attack(info, &mut self.rng)?,
            ));
            Ok(())
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }

    pub(crate) fn proceed_pirates_attack(&mut self) -> Result<(), StateError> {
        if let Mode::GameEvent(LocationEvent::PirateEncounter(
            PirateEncounterState::PiratesAttack {
                info:
                    PirateEncounterInfo {
                        health,
                        cur_pirates,
                        total_pirates,
                    },
                damage_this_attack,
            },
        )) = self.mode
        {
            let health = health.checked_sub(damage_this_attack).unwrap_or(0);
            if health == 0 {
                self.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
                    PirateEncounterState::Destroyed,
                ));
            } else {
                self.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
                    PirateEncounterState::Prompt {
                        info: PirateEncounterInfo {
                            health,
                            cur_pirates,
                            total_pirates,
                        },
                    },
                ));
            }
            Ok(())
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }

    pub(crate) fn proceed_destroyed(&mut self) -> Result<(), StateError> {
        if let Mode::GameEvent(LocationEvent::PirateEncounter(PirateEncounterState::Destroyed)) =
            self.mode
        {
            // player loses their inventory and half of their gold to the pirates
            self.inventory = Inventory::default();
            self.gold = self.gold.div_ceil(2);
            self.mode = Mode::ViewingInventory;
            Ok(())
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }

    pub(crate) fn proceed_pirate_run_success(&mut self) -> Result<(), StateError> {
        if let Mode::GameEvent(LocationEvent::PirateEncounter(PirateEncounterState::RunSuccess)) =
            self.mode
        {
            self.mode = Mode::ViewingInventory;
            Ok(())
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }

    pub(crate) fn proceed_attack_result(&mut self) -> Result<(), StateError> {
        if let Mode::GameEvent(LocationEvent::PirateEncounter(
            PirateEncounterState::AttackResult {
                info,
                did_kill_a_pirate,
            },
        )) = self.mode
        {
            let cur_pirates = info
                .cur_pirates
                .checked_sub(if did_kill_a_pirate { 1 } else { 0 })
                .unwrap_or(0);
            if cur_pirates == 0 {
                // player recovers some gold from wreckage
                let gold_recovered: u32 = self.rng.gen_range(
                    (GOLD_PER_PIRATE_VICTORY_MIN * (info.total_pirates as u32))
                        ..(GOLD_PER_PIRATE_VICTORY_MAX * (info.total_pirates as u32)),
                );
                self.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
                    PirateEncounterState::Victory { gold_recovered },
                ));
            } else {
                self.mode = Mode::GameEvent(LocationEvent::PirateEncounter(
                    PirateEncounterState::pirates_attack(
                        PirateEncounterInfo {
                            health: info.health,
                            cur_pirates,
                            total_pirates: info.cur_pirates,
                        },
                        &mut self.rng,
                    )?,
                ))
            }
            Ok(())
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }

    pub(crate) fn proceed_pirate_encounter_victory(&mut self) -> Result<(), StateError> {
        if let Mode::GameEvent(LocationEvent::PirateEncounter(PirateEncounterState::Victory {
            gold_recovered,
        })) = self.mode
        {
            self.gold += gold_recovered;
            self.mode = Mode::ViewingInventory;
            Ok(())
        } else {
            Err(StateError::InvalidMode(self.mode.clone()))
        }
    }
}

fn logarithmic_decay(count: u32, decay_factor: f64) -> f64 {
    let initial_probability: f64 = 1.0; // 100%
    let decayed = initial_probability - decay_factor * (count as f64 + 1.0).ln();
    let smoothed = decayed + (initial_probability - decayed) / 2f64;
    smoothed
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GoodsStolenResult {
    NothingStolen,
    WasStolen { good: Good, amount: u32 },
}

#[derive(Debug)]
pub enum StateError {
    InvalidMode(Mode),
    CannotAfford,
    InsufficientHold,
    InsufficientInventory,
    InsufficientStash,
    AlreadyInLocation,
    LocationNotHomeBase(Location),
    PayDownAmountHigherThanDebt,
    InsufficientBank,
    UnknownError(String),
}

impl Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
