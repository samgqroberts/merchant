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
pub struct PriceConfig {
    pub starting_gold: u32,
    pub tea: (f32, f32),
    pub coffee: (f32, f32),
    pub sugar: (f32, f32),
    pub tobacco: (f32, f32),
    pub rum: (f32, f32),
    pub cotton: (f32, f32),
}

#[derive(Clone, Debug)]
pub struct Prices {
    pub config: PriceConfig,
    pub london: Inventory,
    pub savannah: Inventory,
    pub lisbon: Inventory,
    pub amsterdam: Inventory,
    pub capetown: Inventory,
    pub venice: Inventory,
}

impl Prices {
    pub fn new(rng: &mut StdRng, starting_gold: u32) -> Prices {
        let config = PriceConfig {
            starting_gold,
            tea: (10.0, 14.0),
            coffee: (4.25, 6.0),
            sugar: (1.0, 2.2),
            tobacco: (0.15, 0.35),
            rum: (0.04, 0.14),
            cotton: (0.005, 0.025),
        };
        let mut res = Prices {
            config,
            london: Inventory::new(),
            savannah: Inventory::new(),
            lisbon: Inventory::new(),
            amsterdam: Inventory::new(),
            capetown: Inventory::new(),
            venice: Inventory::new(),
        };
        res.randomize_location_inventory(rng, &Location::London);
        res.randomize_location_inventory(rng, &Location::Savannah);
        res.randomize_location_inventory(rng, &Location::Lisbon);
        res.randomize_location_inventory(rng, &Location::Amsterdam);
        res.randomize_location_inventory(rng, &Location::CapeTown);
        res.randomize_location_inventory(rng, &Location::Venice);
        res
    }

    pub fn randomized_inventory(&self, rng: &mut StdRng) -> Inventory {
        let config = &self.config;
        let starting_gold = config.starting_gold;
        let mut gen = |low_multiple: f32, high_multiple: f32| -> u32 {
            let low = (starting_gold as f32 * low_multiple).floor() as u32;
            let high = (starting_gold as f32 * high_multiple).floor() as u32;
            rng.next_u32() % (high - low) + low
        };
        Inventory {
            tea: gen(config.tea.0, config.tea.1),
            coffee: gen(config.coffee.0, config.coffee.1),
            sugar: gen(config.sugar.0, config.sugar.1),
            tobacco: gen(config.tobacco.0, config.tobacco.1),
            rum: gen(config.rum.0, config.rum.1),
            cotton: gen(config.cotton.0, config.cotton.1),
        }
    }

    pub fn randomize_location_inventory(&mut self, rng: &mut StdRng, location: &Location) -> () {
        match location {
            Location::London => self.london = self.randomized_inventory(rng),
            Location::Savannah => self.savannah = self.randomized_inventory(rng),
            Location::Lisbon => self.lisbon = self.randomized_inventory(rng),
            Location::Amsterdam => self.amsterdam = self.randomized_inventory(rng),
            Location::CapeTown => self.capetown = self.randomized_inventory(rng),
            Location::Venice => self.venice = self.randomized_inventory(rng),
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
    pub tea: u32,
    pub coffee: u32,
    pub sugar: u32,
    pub tobacco: u32,
    pub rum: u32,
    pub cotton: u32,
}

impl Inventory {
    fn new() -> Inventory {
        Inventory {
            tea: 0,
            coffee: 0,
            sugar: 0,
            tobacco: 0,
            rum: 0,
            cotton: 0,
        }
    }

    pub fn good_amount(&self, good_type: &GoodType) -> u32 {
        match good_type {
            GoodType::Tea => self.tea,
            GoodType::Coffee => self.coffee,
            GoodType::Sugar => self.sugar,
            GoodType::Tobacco => self.tobacco,
            GoodType::Rum => self.rum,
            GoodType::Cotton => self.cotton,
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
            GoodType::Tea => new_inventory.tea += amount,
            GoodType::Coffee => new_inventory.coffee += amount,
            GoodType::Sugar => new_inventory.sugar += amount,
            GoodType::Tobacco => new_inventory.tobacco += amount,
            GoodType::Rum => new_inventory.rum += amount,
            GoodType::Cotton => new_inventory.cotton += amount,
        }
        new_inventory
    }

    pub fn remove_good(&self, good: &GoodType, amount: u32) -> Inventory {
        let mut new_inventory = self.clone();
        match good {
            GoodType::Tea => new_inventory.tea -= amount,
            GoodType::Coffee => new_inventory.coffee -= amount,
            GoodType::Sugar => new_inventory.sugar -= amount,
            GoodType::Tobacco => new_inventory.tobacco -= amount,
            GoodType::Rum => new_inventory.rum -= amount,
            GoodType::Cotton => new_inventory.cotton -= amount,
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
    pub game_end: bool,
}

impl GameState {
    pub fn new(mut rng: StdRng) -> GameState {
        let starting_gold = 500;
        let debt = starting_gold * 3;
        let prices = Prices::new(&mut rng, starting_gold);
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
                // update prices for location we just left
                new_state
                    .prices
                    .randomize_location_inventory(&mut new_state.rng, &destination);
                new_state.location = destination.clone();
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
    Tea,
    Coffee,
    Sugar,
    Tobacco,
    Rum,
    Cotton,
}

const GOOD_TYPES: &'static [GoodType] = &[
    GoodType::Tea,
    GoodType::Coffee,
    GoodType::Sugar,
    GoodType::Tobacco,
    GoodType::Rum,
    GoodType::Cotton,
];

impl Display for GoodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            GoodType::Tea => "Tea",
            GoodType::Coffee => "Coffee",
            GoodType::Sugar => "Sugar",
            GoodType::Tobacco => "Tobacco",
            GoodType::Rum => "Rum",
            GoodType::Cotton => "Cotton",
        };
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", string)
    }
}
