use rand::{rngs::StdRng, RngCore};

use super::{Inventory, Location, LocationEvent, MerchantRng};

/// For each good type, contains a "low multiple" and "high multiple".
/// These multiples are multiplied by the starting gold to produce the range of good prices at any
/// location.
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
pub struct PriceRanges {
    pub tea: (u32, u32),
    pub coffee: (u32, u32),
    pub sugar: (u32, u32),
    pub tobacco: (u32, u32),
    pub rum: (u32, u32),
    pub cotton: (u32, u32),
}

impl PriceConfig {
    pub fn price_ranges(&self) -> PriceRanges {
        let starting_gold = self.starting_gold as f32;
        PriceRanges {
            tea: (
                (starting_gold as f32 * self.tea.0).floor() as u32,
                (starting_gold as f32 * self.tea.1).floor() as u32,
            ),
            coffee: (
                (starting_gold as f32 * self.coffee.0).floor() as u32,
                (starting_gold as f32 * self.coffee.1).floor() as u32,
            ),
            sugar: (
                (starting_gold as f32 * self.sugar.0).floor() as u32,
                (starting_gold as f32 * self.sugar.1).floor() as u32,
            ),
            tobacco: (
                (starting_gold as f32 * self.tobacco.0).floor() as u32,
                (starting_gold as f32 * self.tobacco.1).floor() as u32,
            ),
            rum: (
                (starting_gold as f32 * self.rum.0).floor() as u32,
                (starting_gold as f32 * self.rum.1).floor() as u32,
            ),
            cotton: (
                (starting_gold as f32 * self.cotton.0).floor() as u32,
                (starting_gold as f32 * self.cotton.1).floor() as u32,
            ),
        }
    }

    pub fn randomized_inventory(&self, rng: &mut StdRng) -> Inventory {
        let ranges = self.price_ranges();
        let mut gen = |(low, high): (u32, u32)| -> u32 { rng.next_u32() % (high - low) + low };
        Inventory {
            tea: gen(ranges.tea),
            coffee: gen(ranges.coffee),
            sugar: gen(ranges.sugar),
            tobacco: gen(ranges.tobacco),
            rum: gen(ranges.rum),
            cotton: gen(ranges.cotton),
        }
    }

    pub fn avg_prices(&self) -> Inventory {
        let ranges = self.price_ranges();
        Inventory {
            tea: (ranges.tea.0 + ranges.tea.1).div_ceil(2),
            coffee: (ranges.coffee.0 + ranges.coffee.1).div_ceil(2),
            sugar: (ranges.sugar.0 + ranges.sugar.1).div_ceil(2),
            tobacco: (ranges.tobacco.0 + ranges.tobacco.1).div_ceil(2),
            rum: (ranges.rum.0 + ranges.rum.1).div_ceil(2),
            cotton: (ranges.cotton.0 + ranges.cotton.1).div_ceil(2),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocationInfo {
    pub prices: Inventory,
    pub event: Option<LocationEvent>,
}

impl LocationInfo {
    pub fn empty() -> Self {
        Self {
            prices: Inventory::default(),
            event: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Locations {
    pub config: PriceConfig,
    pub london: LocationInfo,
    pub savannah: LocationInfo,
    pub lisbon: LocationInfo,
    pub amsterdam: LocationInfo,
    pub capetown: LocationInfo,
    pub venice: LocationInfo,
}

impl Locations {
    pub fn new(
        rng: &mut Box<dyn MerchantRng>,
        starting_gold: u32,
        starting_debt: u32,
    ) -> Locations {
        let config = PriceConfig {
            starting_gold,
            tea: (10.0, 14.0),
            coffee: (4.25, 6.0),
            sugar: (1.0, 2.2),
            tobacco: (0.15, 0.35),
            rum: (0.04, 0.14),
            cotton: (0.005, 0.025),
        };
        let mut res = Locations {
            config,
            london: LocationInfo::empty(),
            savannah: LocationInfo::empty(),
            lisbon: LocationInfo::empty(),
            amsterdam: LocationInfo::empty(),
            capetown: LocationInfo::empty(),
            venice: LocationInfo::empty(),
        };
        for location in Location::variants() {
            // for new games, don't put an event in home base
            res.generate_location(
                rng,
                location,
                location != &Location::London,
                starting_gold as i32 - starting_debt as i32,
            );
        }
        res
    }

    pub fn generate_location(
        &mut self,
        rng: &mut Box<dyn MerchantRng>,
        location: &Location,
        allow_events: bool,
        player_net_worth: i32,
    ) -> &LocationInfo {
        let new_location_info = rng.gen_location_info(allow_events, &self.config, player_net_worth);
        let location_info = self.location_info_mut(location);
        *location_info = new_location_info;
        location_info
    }

    pub fn location_info(&self, location: &Location) -> &LocationInfo {
        match location {
            Location::London => &self.london,
            Location::Savannah => &self.savannah,
            Location::Lisbon => &self.lisbon,
            Location::Amsterdam => &self.amsterdam,
            Location::CapeTown => &self.capetown,
            Location::Venice => &self.venice,
        }
    }

    pub fn location_info_mut(&mut self, location: &Location) -> &mut LocationInfo {
        match location {
            Location::London => &mut self.london,
            Location::Savannah => &mut self.savannah,
            Location::Lisbon => &mut self.lisbon,
            Location::Amsterdam => &mut self.amsterdam,
            Location::CapeTown => &mut self.capetown,
            Location::Venice => &mut self.venice,
        }
    }
}
