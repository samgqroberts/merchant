use std::fmt::{self, Display};

use chrono::NaiveDate;
use rand::{rngs::StdRng, RngCore, SeedableRng};

#[derive(Clone, Debug)]
pub enum Location {
    Savannah,
    London,
    Lisbon,
    Amsterdam,
    CapeTown,
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Location::Savannah => "Savannah",
            Location::London => "London",
            Location::Lisbon => "Lisbon",
            Location::Amsterdam => "Amsterdam",
            Location::CapeTown => "Cape Town",
        };
        write!(f, "{}", string)
    }
}

#[derive(Clone, Debug)]
pub struct Prices {
    pub savannah: Inventory,
    pub london: Inventory,
    pub lisbon: Inventory,
    pub amsterdam: Inventory,
    pub capetown: Inventory,
}

impl Prices {
    pub fn new(rng: &mut StdRng) -> Prices {
        Prices {
            savannah: Prices::randomized_inventory(rng),
            london: Prices::randomized_inventory(rng),
            lisbon: Prices::randomized_inventory(rng),
            amsterdam: Prices::randomized_inventory(rng),
            capetown: Prices::randomized_inventory(rng),
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

    pub fn location_prices(&self, location: &Location) -> &Inventory {
        match location {
            Location::Savannah => &self.savannah,
            Location::London => &self.london,
            Location::Lisbon => &self.lisbon,
            Location::Amsterdam => &self.amsterdam,
            Location::CapeTown => &self.capetown,
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
pub struct BuyInfo {
    pub good: GoodType,
    pub amount: Option<u32>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Mode {
    ViewingInventory,
    Buying(Option<BuyInfo>),
    Selling(Option<BuyInfo>),
    Sailing,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub rng: StdRng,
    pub initialized: bool,
    pub date: NaiveDate,
    pub hold_size: u32,
    pub gold: u32,
    pub location: Location,
    pub inventory: Inventory,
    pub prices: Prices,
    pub mode: Mode,
}

impl GameState {
    pub fn new(mut rng: StdRng) -> GameState {
        let prices = Prices::new(&mut rng);
        GameState {
            rng,
            initialized: false,
            date: NaiveDate::from_ymd_opt(1782, 3, 1).unwrap(),
            hold_size: 100,
            gold: 1400,
            location: Location::London,
            inventory: Inventory::new(),
            prices,
            mode: Mode::ViewingInventory,
        }
    }

    pub fn from_u64_seed(seed: u64) -> Self {
        Self::new(StdRng::seed_from_u64(seed))
    }

    pub fn initialize(&self) -> GameState {
        let mut game_state = self.clone();
        game_state.initialized = true;
        game_state
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
