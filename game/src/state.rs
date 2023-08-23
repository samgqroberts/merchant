use std::fmt::{self, Display};

use chrono::Month;
use rand::{rngs::StdRng, RngCore};

#[derive(Clone, Debug, PartialEq)]
pub enum Location {
    London,
    Savannah,
    Lisbon,
    Amsterdam,
    CapeTown,
    Venice,
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Location::London => "London",
            Location::Savannah => "Savannah",
            Location::Lisbon => "Lisbon",
            Location::Amsterdam => "Amsterdam",
            Location::CapeTown => "Cape Town",
            Location::Venice => "Venice",
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug)]
pub struct Prices {
    pub london: Inventory,
    pub savannah: Inventory,
    pub lisbon: Inventory,
    pub amsterdam: Inventory,
    pub capetown: Inventory,
    pub venice: Inventory,
}

impl Prices {
    pub fn new(rng: &mut StdRng) -> Prices {
        Prices {
            london: Prices::randomized_inventory(rng),
            savannah: Prices::randomized_inventory(rng),
            lisbon: Prices::randomized_inventory(rng),
            amsterdam: Prices::randomized_inventory(rng),
            capetown: Prices::randomized_inventory(rng),
            venice: Prices::randomized_inventory(rng),
        }
    }

    pub fn randomized_inventory(rng: &mut StdRng) -> Inventory {
        // number between 39 and 111
        let mut gen = || rng.next_u32() % (111 - 39) + 39;
        Inventory {
            sugar: gen(),
            tobacco: gen(),
            tea: gen(),
            cotton: gen(),
            rum: gen(),
            coffee: gen(),
        }
    }

    pub fn randomize_location_inventory(&mut self, rng: &mut StdRng, location: &Location) -> () {
        match location {
            Location::London => self.london = Prices::randomized_inventory(rng),
            Location::Savannah => self.savannah = Prices::randomized_inventory(rng),
            Location::Lisbon => self.lisbon = Prices::randomized_inventory(rng),
            Location::Amsterdam => self.amsterdam = Prices::randomized_inventory(rng),
            Location::CapeTown => self.capetown = Prices::randomized_inventory(rng),
            Location::Venice => self.venice = Prices::randomized_inventory(rng),
        }
    }

    pub fn location_prices(&self, location: &Location) -> &Inventory {
        match location {
            Location::London => &self.london,
            Location::Savannah => &self.savannah,
            Location::Lisbon => &self.lisbon,
            Location::Amsterdam => &self.amsterdam,
            Location::CapeTown => &self.capetown,
            Location::Venice => &self.venice,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Inventory {
    pub sugar: u32,
    pub tobacco: u32,
    pub tea: u32,
    pub cotton: u32,
    pub rum: u32,
    pub coffee: u32,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            sugar: 0,
            tobacco: 0,
            tea: 0,
            cotton: 0,
            rum: 0,
            coffee: 0,
        }
    }

    pub fn good_amount(&self, good_type: &GoodType) -> u32 {
        match good_type {
            GoodType::Sugar => self.sugar,
            GoodType::Tobacco => self.tobacco,
            GoodType::Tea => self.tea,
            GoodType::Cotton => self.cotton,
            GoodType::Rum => self.rum,
            GoodType::Coffee => self.coffee,
        }
    }

    pub fn total_amount(&self) -> u32 {
        let mut total: u32 = 0;
        for good in GOOD_TYPES {
            total += self.good_amount(good);
        }
        total
    }

    pub fn add_good(&self, good: &GoodType, amount: u32) -> Inventory {
        let mut new_inventory = self.clone();
        match good {
            GoodType::Sugar => new_inventory.sugar += amount,
            GoodType::Tobacco => new_inventory.tobacco += amount,
            GoodType::Tea => new_inventory.tea += amount,
            GoodType::Cotton => new_inventory.cotton += amount,
            GoodType::Rum => new_inventory.rum += amount,
            GoodType::Coffee => new_inventory.coffee += amount,
        }
        new_inventory
    }

    pub fn remove_good(&self, good: &GoodType, amount: u32) -> Inventory {
        let mut new_inventory = self.clone();
        match good {
            GoodType::Sugar => new_inventory.sugar -= amount,
            GoodType::Tobacco => new_inventory.tobacco -= amount,
            GoodType::Tea => new_inventory.tea -= amount,
            GoodType::Cotton => new_inventory.cotton -= amount,
            GoodType::Rum => new_inventory.rum -= amount,
            GoodType::Coffee => new_inventory.coffee -= amount,
        }
        new_inventory
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Transaction {
    pub good: GoodType,
    pub amount: Option<u32>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Mode {
    ViewingInventory,
    Buying(Option<Transaction>),
    Selling(Option<Transaction>),
    Sailing,
    StashDeposit(Option<Transaction>),
    StashWithdraw(Option<Transaction>),
    BorrowGold(Option<u32>),
    PayDebt(Option<u32>),
    BankDeposit(Option<u32>),
    BankWithdraw(Option<u32>),
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
    pub prices: Prices,
    pub debt: u32,
    pub mode: Mode,
}

impl GameState {
    pub fn new(mut rng: StdRng) -> GameState {
        let starting_gold = 1400;
        let prices = Prices::new(&mut rng);
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
            prices,
            debt: starting_gold,
            mode: Mode::ViewingInventory,
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

    pub fn begin_borrow_gold(&self) -> Result<GameState, StateError> {
        self.require_viewing_inventory()?;
        self.require_location_home_base()?;
        let mut new_state = self.clone();
        new_state.mode = Mode::BorrowGold(None);
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

    pub fn choose_buy_good(&self, good: GoodType) -> Result<GameState, StateError> {
        if let Mode::Buying(None) = &self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::Buying(Some(Transaction { good, amount: None }));
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn choose_sell_good(&self, good: GoodType) -> Result<GameState, StateError> {
        if let Mode::Selling(None) = &self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::Selling(Some(Transaction { good, amount: None }));
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn choose_stash_deposit_good(&self, good: GoodType) -> Result<GameState, StateError> {
        if let Mode::StashDeposit(None) = &self.mode {
            let mut new_state = self.clone();
            new_state.mode = Mode::StashDeposit(Some(Transaction { good, amount: None }));
            return Ok(new_state);
        }
        Err(StateError::InvalidMode(&self.mode))
    }

    pub fn choose_stash_withdraw_good(&self, good: GoodType) -> Result<GameState, StateError> {
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
                Mode::BorrowGold(amount) => Some(amount),
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
                Mode::BorrowGold(amount) => Some(amount),
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
                        .prices
                        .location_prices(&self.location)
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
                        .prices
                        .location_prices(&self.location)
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

    pub fn commit_borrow_gold(&self) -> Result<GameState, StateError> {
        if let Mode::BorrowGold(amount) = &self.mode {
            let amount = &amount.unwrap_or(0);
            let mut new_state = self.clone();
            new_state.debt += amount;
            new_state.gold += amount;
            new_state.mode = Mode::ViewingInventory;
            return Ok(new_state);
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
                // update prices for location we just left
                new_state
                    .prices
                    .randomize_location_inventory(&mut new_state.rng, &destination);
                new_state.location = destination.clone();
                // increment the month
                new_state.date.1 = new_state.date.1.succ();
                if new_state.date.1 == Month::January {
                    new_state.date.0 += 1;
                }
                // increment debt, if any
                let new_debt = f64::from(new_state.debt) * 1.1;
                new_state.debt = new_debt.floor() as u32;
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

#[derive(Debug, PartialEq, Clone)]
pub enum GoodType {
    Sugar,
    Tobacco,
    Tea,
    Cotton,
    Rum,
    Coffee,
}

const GOOD_TYPES: &'static [GoodType] = &[
    GoodType::Sugar,
    GoodType::Tobacco,
    GoodType::Tea,
    GoodType::Cotton,
    GoodType::Rum,
    GoodType::Coffee,
];

impl Display for GoodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            GoodType::Sugar => "Sugar",
            GoodType::Tobacco => "Tobacco",
            GoodType::Tea => "Tea",
            GoodType::Cotton => "Cotton",
            GoodType::Rum => "Rum",
            GoodType::Coffee => "Coffee",
        };
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", string)
    }
}
