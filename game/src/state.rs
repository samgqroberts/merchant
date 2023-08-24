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

    pub fn initialize(&self) -> GameState {
        let mut game_state = self.clone();
        game_state.initialized = true;
        game_state
    }

    fn require_viewing_inventory(&self) -> Result<(), StateError> {
        if self.mode != Mode::ViewingInventory {
            Err(StateError::InvalidMode(&self.mode))
        } else {
            Ok(())
        }
    }

    fn require_location_home_base(&self) -> Result<(), StateError> {
        if self.location != Location::London {
            Err(StateError::LocationNotHomeBase(&self.location))
        } else {
            Ok(())
        }
    }

    pub fn begin_buying(&self) -> Result<GameState, StateError> {
        self.require_viewing_inventory()?;
        let mut new_state = self.clone();
        new_state.mode = Mode::Buying(None);
        return Ok(new_state);
    }

    pub fn begin_selling(&self) -> Result<GameState, StateError> {
        self.require_viewing_inventory()?;
        let mut new_state = self.clone();
        new_state.mode = Mode::Selling(None);
        return Ok(new_state);
    }

    pub fn begin_sailing(&self) -> Result<GameState, StateError> {
        self.require_viewing_inventory()?;
        let mut new_state = self.clone();
        new_state.mode = Mode::Sailing;
        return Ok(new_state);
    }

    pub fn begin_stash_deposit(&self) -> Result<GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        let mut new_state = self.clone();
        new_state.mode = Mode::StashDeposit(None);
        return Ok(new_state);
    }

    pub fn begin_stash_withdraw(&self) -> Result<GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        let mut new_state = self.clone();
        new_state.mode = Mode::StashWithdraw(None);
        return Ok(new_state);
    }

    pub fn begin_pay_debt(&self) -> Result<GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        let mut new_state = self.clone();
        new_state.mode = Mode::PayDebt(None);
        return Ok(new_state);
    }

    pub fn begin_bank_deposit(&self) -> Result<GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        let mut new_state = self.clone();
        new_state.mode = Mode::BankDeposit(None);
        return Ok(new_state);
    }

    pub fn begin_bank_withdraw(&self) -> Result<GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        let mut new_state = self.clone();
        new_state.mode = Mode::BankWithdraw(None);
        return Ok(new_state);
    }

    pub fn choose_buy_good(&self, good: Good) -> Result<GameState, StateError> {
        if let Mode::Buying(None) = &self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::Buying(Some(Transaction { good, amount: None }));
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn choose_sell_good(&self, good: Good) -> Result<GameState, StateError> {
        if let Mode::Selling(None) = &self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::Selling(Some(Transaction { good, amount: None }));
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn choose_stash_deposit_good(&self, good: Good) -> Result<GameState, StateError> {
        if let Mode::StashDeposit(None) = &self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::StashDeposit(Some(Transaction { good, amount: None }));
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn choose_stash_withdraw_good(&self, good: Good) -> Result<GameState, StateError> {
        if let Mode::StashWithdraw(None) = &self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::StashWithdraw(Some(Transaction { good, amount: None }));
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn cancel_buy(&self) -> Result<GameState, StateError> {
        if let Mode::Buying(None) = &self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::ViewingInventory;
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn cancel_sell(&self) -> Result<GameState, StateError> {
        if let Mode::Selling(None) = &self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::ViewingInventory;
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn cancel_stash_deposit(&self) -> Result<GameState, StateError> {
        if let Mode::StashDeposit(None) = &self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::ViewingInventory;
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn cancel_stash_withdraw(&self) -> Result<GameState, StateError> {
        if let Mode::StashWithdraw(None) = &self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::ViewingInventory;
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn user_typed_digit(&self, digit: u32) -> Result<GameState, StateError> {
        let mut new_state = self.clone();
        let mut binding = None;
        let info = match &mut new_state.mode {
            Mode::Buying(info) => info,
            Mode::Selling(info) => info,
            Mode::StashDeposit(info) => info,
            Mode::StashWithdraw(info) => info,
            _ => &mut binding,
        };
        if let Some(info) = info {
            info.amount = Some(info.amount.map_or(digit, |amount| amount * 10 + digit));
            Ok(new_state)
        } else {
            let binding: Option<&mut Option<u32>> = None;
            let amount = match &mut new_state.mode {
                Mode::PayDebt(amount) => Some(amount),
                Mode::BankDeposit(amount) => Some(amount),
                Mode::BankWithdraw(amount) => Some(amount),
                _ => binding,
            };
            if let Some(amount) = amount {
                *amount = amount.map_or(Some(digit), |amount| Some(amount * 10 + digit));
                Ok(new_state)
            } else {
                Err(StateError::InvalidMode(&self.mode))
            }
        }
    }

    pub fn user_typed_backspace(&self) -> Result<GameState, StateError> {
        let mut new_state = self.clone();
        let mut binding = None;
        let info = match &mut new_state.mode {
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
            Ok(new_state)
        } else {
            let binding: Option<&mut Option<u32>> = None;
            let amount = match &mut new_state.mode {
                Mode::PayDebt(amount) => Some(amount),
                Mode::BankDeposit(amount) => Some(amount),
                Mode::BankWithdraw(amount) => Some(amount),
                _ => binding,
            };
            if let Some(amount) = amount {
                *amount =
                    amount.and_then(|amount| if amount <= 9 { None } else { Some(amount / 10) });
                Ok(new_state)
            } else {
                Err(StateError::InvalidMode(&self.mode))
            }
        }
    }

    pub fn commit_buy(&self) -> Result<GameState, StateError> {
        if let Mode::Buying(Some(info)) = &self.mode {
            match info.amount.unwrap_or(0) {
                0 => {
                    let mut new_state = self.clone();
                    new_state.mode = Mode::ViewingInventory;
                    return Ok(new_state);
                }
                amount => {
                    let good_price = self
                        .locations
                        .location_info(&self.location)
                        .prices
                        .good_amount(&info.good);
                    let can_afford = self.gold / good_price;
                    if amount > can_afford {
                        return Err(StateError::CannotAfford);
                    } else {
                        let hold_size = self.hold_size;
                        let current_hold = self.inventory.total_amount();
                        if current_hold + amount > hold_size {
                            return Err(StateError::InsufficientHold);
                        } else {
                            let mut new_state = self.clone();
                            new_state.inventory = new_state.inventory.add_good(&info.good, amount);
                            new_state.gold -= good_price * amount;
                            new_state.mode = Mode::ViewingInventory;
                            return Ok(new_state);
                        }
                    }
                }
            }
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn commit_sell(&self) -> Result<GameState, StateError> {
        if let Mode::Selling(Some(info)) = &self.mode {
            match info.amount.unwrap_or(0) {
                0 => {
                    let mut new_state = self.clone();
                    new_state.mode = Mode::ViewingInventory;
                    return Ok(new_state);
                }
                amount => {
                    let good_price = self
                        .locations
                        .location_info(&self.location)
                        .prices
                        .good_amount(&info.good);
                    let user_amount = self.inventory.good_amount(&info.good);
                    if amount > user_amount {
                        return Err(StateError::InsufficientInventory);
                    } else {
                        let mut new_state = self.clone();
                        new_state.inventory = new_state.inventory.remove_good(&info.good, amount);
                        new_state.gold += good_price * amount;
                        new_state.mode = Mode::ViewingInventory;
                        return Ok(new_state);
                    }
                }
            }
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn commit_stash_deposit(&self) -> Result<GameState, StateError> {
        if let Mode::StashDeposit(Some(info)) = &self.mode {
            match info.amount.unwrap_or(0) {
                0 => {
                    let mut new_state = self.clone();
                    new_state.mode = Mode::ViewingInventory;
                    return Ok(new_state);
                }
                amount => {
                    let user_amount = self.inventory.good_amount(&info.good);
                    if amount > user_amount {
                        return Err(StateError::InsufficientInventory);
                    } else {
                        let mut new_state = self.clone();
                        new_state.inventory = new_state.inventory.remove_good(&info.good, amount);
                        new_state.stash = new_state.stash.add_good(&info.good, amount);
                        new_state.mode = Mode::ViewingInventory;
                        return Ok(new_state);
                    }
                }
            }
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn commit_stash_withdraw(&self) -> Result<GameState, StateError> {
        if let Mode::StashWithdraw(Some(info)) = &self.mode {
            match info.amount.unwrap_or(0) {
                0 => {
                    let mut new_state = self.clone();
                    new_state.mode = Mode::ViewingInventory;
                    return Ok(new_state);
                }
                amount => {
                    let stash_amount = self.stash.good_amount(&info.good);
                    if amount > stash_amount {
                        return Err(StateError::InsufficientStash);
                    } else {
                        let mut new_state = self.clone();
                        new_state.stash = new_state.stash.remove_good(&info.good, amount);
                        new_state.inventory = new_state.inventory.add_good(&info.good, amount);
                        new_state.mode = Mode::ViewingInventory;
                        return Ok(new_state);
                    }
                }
            }
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn commit_pay_debt(&self) -> Result<GameState, StateError> {
        if let Mode::PayDebt(amount) = &self.mode {
            let amount = &amount.unwrap_or(0);
            if amount > &self.debt {
                return Err(StateError::PayDownAmountHigherThanDebt);
            }
            if amount > &self.gold {
                return Err(StateError::CannotAfford);
            }
            let mut new_state = self.clone();
            new_state.debt -= amount;
            new_state.gold -= amount;
            new_state.mode = Mode::ViewingInventory;
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn commit_bank_deposit(&self) -> Result<GameState, StateError> {
        if let Mode::BankDeposit(amount) = &self.mode {
            let amount = &amount.unwrap_or(0);
            if amount > &self.gold {
                return Err(StateError::CannotAfford);
            }
            let mut new_state = self.clone();
            new_state.gold -= amount;
            new_state.bank += amount;
            new_state.mode = Mode::ViewingInventory;
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn commit_bank_withdraw(&self) -> Result<GameState, StateError> {
        if let Mode::BankWithdraw(amount) = &self.mode {
            let amount = &amount.unwrap_or(0);
            if amount > &self.bank {
                return Err(StateError::InsufficientBank);
            }
            let mut new_state = self.clone();
            new_state.gold += amount;
            new_state.bank -= amount;
            new_state.mode = Mode::ViewingInventory;
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn sail_to(&self, destination: &Location) -> Result<GameState, StateError> {
        if let Mode::Sailing = self.mode {
            if destination == &self.location {
                Err(StateError::AlreadyInLocation)
            } else {
                let mut new_state = self.clone();
                new_state.mode = Mode::ViewingInventory;
                // increment the month
                new_state.date.1 = new_state.date.1.succ();
                if new_state.date.1 == Month::January {
                    new_state.date.0 += 1;
                }
                if new_state.date.0 == 1785 && new_state.date.1 == Month::March {
                    // 3 years have elapsed
                    // end the game
                    new_state.game_end = true
                }
                // update location info for location we just left
                let new_location_info =
                    new_state
                        .locations
                        .generate_location(&mut new_state.rng, &destination, true);
                // set current location
                new_state.location = destination.clone();
                // increment debt, if any
                let new_debt = f64::from(new_state.debt) * 1.1;
                new_state.debt = new_debt.floor() as u32;
                // determine if we've encountered an event
                if let Some(event) = &new_location_info.event {
                    new_state.mode = Mode::GameEvent(event.clone());
                }
                Ok(new_state)
            }
        } else {
            Err(StateError::InvalidMode(&self.mode))
        }
    }

    pub fn cancel_sail_to(&self) -> Result<GameState, StateError> {
        if let Mode::Sailing = self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::ViewingInventory;
            Ok(new_state)
        } else {
            Err(StateError::InvalidMode(&self.mode))
        }
    }

    pub fn acknowledge_event(&self) -> Result<GameState, StateError> {
        if let Mode::GameEvent(_) = self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::ViewingInventory;
            Ok(new_state)
        } else {
            Err(StateError::InvalidMode(&self.mode))
        }
    }
}

#[derive(Debug)]
pub enum StateError<'a> {
    InvalidMode(&'a Mode),
    CannotAfford,
    InsufficientHold,
    InsufficientInventory,
    InsufficientStash,
    AlreadyInLocation,
    LocationNotHomeBase(&'a Location),
    PayDownAmountHigherThanDebt,
    InsufficientBank,
}

impl<'a> Display for StateError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
