use rand::{rngs::StdRng, RngCore};

use super::{Good, Inventory, Location, LocationEvent};

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
pub struct LocationInfo {
    pub prices: Inventory,
    pub event: Option<LocationEvent>,
}

impl LocationInfo {
    pub fn empty() -> Self {
        Self {
            prices: Inventory::new(),
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
    pub fn new(rng: &mut StdRng, starting_gold: u32) -> Locations {
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
            res.generate_location(rng, location, location != &Location::London);
        }
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

    pub fn generate_location(
        &mut self,
        rng: &mut StdRng,
        location: &Location,
        allow_events: bool,
    ) -> &LocationInfo {
        let mut new_location_info = LocationInfo::empty();
        new_location_info.prices = self.randomized_inventory(rng);
        if allow_events {
            // % 2 means 50% chance of hitting some event
            new_location_info.event = if rng.next_u32() % 2 == 0 {
                // we've hit an event
                let event: LocationEvent = match rng.next_u32() % 4 {
                    0 => {
                        // cheap good
                        let good = Good::random(rng);
                        // update location prices
                        let good_price = (&mut new_location_info).prices.get_good_mut(&good);
                        *good_price = ((*good_price as f64) * 0.5).floor() as u32;
                        LocationEvent::CheapGood(good)
                    }
                    1 => {
                        // more expensive good
                        let good = Good::random(rng);
                        // update location prices
                        let good_price = (&mut new_location_info).prices.get_good_mut(&good);
                        *good_price = ((*good_price as f64) * 2.0).floor() as u32;
                        LocationEvent::ExpensiveGood(good)
                    }
                    2 => {
                        // find goods
                        let good = Good::random(rng);
                        let amount = (rng.next_u32() % 10) + 1;
                        LocationEvent::FindGoods(good, amount)
                    }
                    _ => {
                        // stolen goods
                        LocationEvent::GoodsStolen(None)
                    }
                };
                Some(event)
            } else {
                None
            };
        };
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
