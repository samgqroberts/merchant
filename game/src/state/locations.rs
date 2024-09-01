use super::{Inventory, Location, LocationEvent, MerchantRng};

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
    pub fn new(rng: &mut Box<dyn MerchantRng>, starting_gold: u32) -> Locations {
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

    pub fn generate_location(
        &mut self,
        rng: &mut Box<dyn MerchantRng>,
        location: &Location,
        allow_events: bool,
    ) -> &LocationInfo {
        let new_location_info = rng.gen_location_info(allow_events, &self.config);
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
