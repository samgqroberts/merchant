use std::rc::Rc;

use super::{
    location_personalities::LocationConfig, Inventory, Location, LocationEvent, MerchantRng,
};

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
    pub config: Rc<LocationConfig>,
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
        config: Rc<LocationConfig>,
        player_net_worth: i32,
    ) -> Locations {
        let home_port = config.home_port;
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
            res.generate_location(rng, location, location != &home_port, player_net_worth);
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
        let location_personality = self.config.personalities.get(location);
        let new_location_info =
            rng.gen_location_info(allow_events, location_personality, player_net_worth);
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
