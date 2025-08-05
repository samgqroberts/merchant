use rand::{distributions::WeightedIndex, prelude::Distribution, rngs::StdRng};

use super::{location_map::LocationMap, Location, PriceRanges};

#[derive(Debug, Clone, Copy)]
pub enum EventPossibility {
    NoEvent,
    CheapGood,
    ExpensiveGood,
    FindGoods,
    StolenGoods,
    CanBuyCannon,
    PirateEncounter,
    CanBuyHoldSpace,
    NoEffect,
}

#[derive(Debug, Clone)]
pub struct EventWeights {
    pub no_event: u8,
    pub cheap_good: u8,
    pub expensive_good: u8,
    pub find_goods: u8,
    pub stolen_goods: u8,
    pub can_buy_cannon: u8,
    pub pirate_encounter: u8,
    pub can_buy_more_hold_space: u8,
    pub no_effect: u8,
}

impl EventWeights {
    pub fn generate_random_event(&self, rng: &mut StdRng) -> EventPossibility {
        const POSSIBILITIES: [EventPossibility; 9] = [
            EventPossibility::NoEvent,
            EventPossibility::CheapGood,
            EventPossibility::ExpensiveGood,
            EventPossibility::FindGoods,
            EventPossibility::StolenGoods,
            EventPossibility::CanBuyCannon,
            EventPossibility::PirateEncounter,
            EventPossibility::CanBuyHoldSpace,
            EventPossibility::NoEffect,
        ];
        let weights = self.weights();
        let dist = WeightedIndex::new(weights).expect("Unable to create WeightedIndex");
        POSSIBILITIES[dist.sample(rng)]
    }

    pub fn weights(&self) -> [u8; 9] {
        [
            self.no_event,
            self.cheap_good,
            self.expensive_good,
            self.find_goods,
            self.stolen_goods,
            self.can_buy_cannon,
            self.pirate_encounter,
            self.can_buy_more_hold_space,
            self.no_effect,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct LocationPersonality {
    pub price_ranges: PriceRanges,
    pub event_weights: EventWeights,
}

pub type LocationPersonalities = LocationMap<LocationPersonality>;

#[derive(Debug, Clone)]
pub struct LocationConfig {
    pub home_port: Location,
    pub overall_price_ranges: PriceRanges,
    pub personalities: LocationPersonalities,
}
