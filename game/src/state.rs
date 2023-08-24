pub mod good;
pub mod inventory;
pub mod location;
pub mod locations;

use std::fmt::{self, Display};

use chrono::Month;
use rand::rngs::StdRng;

pub use self::good::Good;
pub use self::inventory::Inventory;
pub use self::location::Location;
pub use self::locations::{LocationInfo, Locations, PriceConfig};

#[derive(PartialEq, Clone, Debug)]
pub struct Transaction {
    pub good: Good,
    pub amount: Option<u32>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum LocationEvent {
    CheapGood(Good),
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
            gold: starting_gold,
            bank: 0,
            location: Location::London, // home base
            stash: Inventory::new(),
            inventory: Inventory::new(),
            locations,
            debt,
            mode: Mode::ViewingInventory,
            game_end: false,
        }
    }

    pub fn initialize(&mut self) -> () {
        self.initialized = true;
    }

    fn require_viewing_inventory(&self) -> Result<(), StateError> {
        if &self.mode != &Mode::ViewingInventory {
            Err(StateError::InvalidMode(self.mode.clone()))
        } else {
            Ok(())
        }
    }

    fn require_location_home_base(&self) -> Result<(), StateError> {
        if &self.location != &Location::London {
            Err(StateError::LocationNotHomeBase(self.location.clone()))
        } else {
            Ok(())
        }
    }

    pub fn begin_buying(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.mode = Mode::Buying(None);
        return Ok(self);
    }

    pub fn begin_selling(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.mode = Mode::Selling(None);
        return Ok(self);
    }

    pub fn begin_sailing(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.mode = Mode::Sailing;
        return Ok(self);
    }

    pub fn begin_stash_deposit(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        self.mode = Mode::StashDeposit(None);
        return Ok(self);
    }

    pub fn begin_stash_withdraw(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        self.mode = Mode::StashWithdraw(None);
        return Ok(self);
    }

    pub fn begin_pay_debt(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        self.mode = Mode::PayDebt(None);
        return Ok(self);
    }

    pub fn begin_bank_deposit(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        self.mode = Mode::BankDeposit(None);
        return Ok(self);
    }

    pub fn begin_bank_withdraw(&mut self) -> Result<&mut GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        self.mode = Mode::BankWithdraw(None);
        return Ok(self);
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
            match info.amount.unwrap_or(0) {
                amount => {
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
            }
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn commit_sell(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::Selling(Some(info)) = &self.mode {
            match info.amount.unwrap_or(0) {
                amount => {
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
            }
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn commit_stash_deposit(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::StashDeposit(Some(info)) = &self.mode {
            match info.amount.unwrap_or(0) {
                amount => {
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
            }
        }
        Err(StateError::InvalidMode(self.mode.clone()))
    }

    pub fn commit_stash_withdraw(&mut self) -> Result<&mut GameState, StateError> {
        if let Mode::StashWithdraw(Some(info)) = &self.mode {
            match info.amount.unwrap_or(0) {
                amount => {
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
                        .generate_location(&mut self.rng, &destination, true);
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
}

impl<'a> Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
