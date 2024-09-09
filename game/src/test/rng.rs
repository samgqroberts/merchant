use std::collections::VecDeque;

use crate::state::{
    EventWeights, Good, Inventory, Location, LocationConfig, LocationInfo, LocationMap,
    LocationPersonality, MerchantRng, PriceRanges,
};

pub struct MockRng {
    gold_recovered_from_pirate_encounter: VecDeque<u32>,
    damage_from_pirates: VecDeque<u8>,
    did_kill_a_pirate: VecDeque<bool>,
    run_success: VecDeque<bool>,
    num_pirates_encountered: VecDeque<u8>,
    good_stolen: VecDeque<(Good, u32)>,
    location_info: VecDeque<LocationInfo>,
    location_config: VecDeque<LocationConfig>,
}

impl MerchantRng for MockRng {
    fn gen_gold_recovered_from_pirate_encounter(&mut self, _: u8) -> u32 {
        self.gold_recovered_from_pirate_encounter
            .pop_front()
            .expect("MockRng not seeded with enough gold_recovered_from_pirate_encounter")
    }

    fn gen_damage_from_pirates(&mut self, _: u8) -> u8 {
        self.damage_from_pirates
            .pop_front()
            .expect("MockRng not seeded with enough damage_from_pirates")
    }

    fn gen_did_kill_a_pirate(&mut self, _: u8) -> bool {
        self.did_kill_a_pirate
            .pop_front()
            .expect("MockRng not seeded with enough did_kill_a_pirate")
    }

    fn gen_run_success(&mut self, _: u8) -> bool {
        self.run_success
            .pop_front()
            .expect("MockRng not seeded with enough run_success")
    }

    fn gen_num_pirates_encountered(&mut self) -> u8 {
        self.num_pirates_encountered
            .pop_front()
            .expect("MockRng not seeded with enough num_pirates_encountered")
    }

    fn gen_good_stolen(&mut self, _: &[(Good, u32)]) -> (Good, u32) {
        self.good_stolen
            .pop_front()
            .expect("MockRng not seeded with enough good_stolen")
    }

    fn gen_location_info(
        &mut self,
        _: bool,
        _: &crate::state::LocationPersonality,
        _: i32,
    ) -> LocationInfo {
        self.location_info
            .pop_front()
            .expect("MockRng not seeded with enough location_info")
    }

    fn gen_location_config(&mut self, _: u32) -> LocationConfig {
        self.location_config
            .pop_front()
            .expect("MockRng not seeded with enough location_config")
    }
}

impl Default for MockRng {
    fn default() -> Self {
        Self::new()
    }
}

impl MockRng {
    pub fn new() -> Self {
        MockRng {
            gold_recovered_from_pirate_encounter: VecDeque::new(),
            damage_from_pirates: VecDeque::new(),
            did_kill_a_pirate: VecDeque::new(),
            run_success: VecDeque::new(),
            num_pirates_encountered: VecDeque::new(),
            good_stolen: VecDeque::new(),
            location_info: VecDeque::new(),
            location_config: VecDeque::new(),
        }
    }

    pub fn new_with_default_locations() -> Self {
        let overall_price_ranges = PriceRanges::from_start_price_and_spreads(
            500,
            [5.0, 3.0, 2.0, 1.5, 1.0, 0.75],
            [5.0, 4.2, 3.4, 2.6, 1.8],
        );
        let basic_event_weights = EventWeights {
            no_event: 6,
            cheap_good: 1,
            expensive_good: 1,
            find_goods: 1,
            stolen_goods: 1,
            can_buy_cannon: 1,
            pirate_encounter: 1,
            can_buy_more_hold_space: 1,
            no_effect: 1,
        };
        let personalities = LocationMap {
            london: LocationPersonality {
                price_ranges: overall_price_ranges.clone(),
                event_weights: basic_event_weights.clone(),
            },
            savannah: LocationPersonality {
                price_ranges: overall_price_ranges.clone(),
                event_weights: basic_event_weights.clone(),
            },
            lisbon: LocationPersonality {
                price_ranges: overall_price_ranges.clone(),
                event_weights: basic_event_weights.clone(),
            },
            amsterdam: LocationPersonality {
                price_ranges: overall_price_ranges.clone(),
                event_weights: basic_event_weights.clone(),
            },
            capetown: LocationPersonality {
                price_ranges: overall_price_ranges.clone(),
                event_weights: basic_event_weights.clone(),
            },
            venice: LocationPersonality {
                price_ranges: overall_price_ranges.clone(),
                event_weights: basic_event_weights.clone(),
            },
        };
        Self::new()
            .push_location_config(LocationConfig {
                home_port: Location::London,
                overall_price_ranges,
                personalities,
            })
            .push_location_infos(&default_location_infos())
    }

    pub fn push_gold_recovered_from_pirate_encounter(
        mut self,
        gold_recovered_from_pirate_encounter: u32,
    ) -> Self {
        self.gold_recovered_from_pirate_encounter
            .push_back(gold_recovered_from_pirate_encounter);
        self
    }

    pub fn push_damage_from_pirates(mut self, damage_from_pirates: u8) -> Self {
        self.damage_from_pirates.push_back(damage_from_pirates);
        self
    }

    pub fn push_did_kill_a_pirate(mut self, did_kill_a_pirate: bool) -> Self {
        self.did_kill_a_pirate.push_back(did_kill_a_pirate);
        self
    }

    pub fn push_run_success(mut self, run_success: bool) -> Self {
        self.run_success.push_back(run_success);
        self
    }

    pub fn push_num_pirates_encountered(mut self, num_pirates_encountered: u8) -> Self {
        self.num_pirates_encountered
            .push_back(num_pirates_encountered);
        self
    }

    pub fn push_good_stolen(mut self, good_stolen: (Good, u32)) -> Self {
        self.good_stolen.push_back(good_stolen);
        self
    }

    pub fn push_location_info(mut self, location_info: LocationInfo) -> Self {
        self.location_info.push_back(location_info);
        self
    }

    pub fn push_location_infos(self, location_infos: &[LocationInfo]) -> Self {
        let mut x = self;
        for location_info in location_infos {
            x = x.push_location_info(location_info.clone());
        }
        x
    }

    pub fn push_location_config(mut self, location_config: LocationConfig) -> Self {
        self.location_config.push_back(location_config);
        self
    }
}

impl From<MockRng> for Box<dyn MerchantRng> {
    fn from(value: MockRng) -> Self {
        Box::new(value)
    }
}

pub fn default_location_info() -> LocationInfo {
    LocationInfo {
        prices: Inventory {
            tea: 6,
            coffee: 5,
            sugar: 4,
            tobacco: 3,
            rum: 2,
            cotton: 1,
        },
        event: None,
    }
}

pub fn default_location_infos() -> Vec<LocationInfo> {
    (0..6).map(|_| default_location_info()).collect()
}
