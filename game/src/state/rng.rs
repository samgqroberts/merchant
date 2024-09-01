use rand::{
    distributions::{Distribution, WeightedIndex},
    rngs::StdRng,
    Rng, RngCore,
};

use super::{
    constants::{GOLD_PER_PIRATE_VICTORY_MAX, GOLD_PER_PIRATE_VICTORY_MIN},
    locations::{LocationInfo, PriceConfig},
    Good, Inventory, LocationEvent, NoEffectEvent,
};

/// A trait that abstracts the pieces of logic that need to use some kind of random number generation.
/// Allows injecting a mocked (deterministic) implementation in testing.
pub trait MerchantRng {
    fn gen_gold_recovered_from_pirate_encounter(&mut self, total_pirates: u8) -> u32;
    fn gen_damage_from_pirates(&mut self, cur_pirates: u8) -> u8;
    fn gen_did_kill_a_pirate(&mut self, cannons: u8) -> bool;
    fn gen_run_success(&mut self, cur_pirates: u8) -> bool;
    fn gen_num_pirates_encountered(&mut self) -> u8;
    fn gen_good_stolen(&mut self, goods_with_inventory: &[(Good, u32)]) -> (Good, u32);
    fn gen_location_info(&mut self, allow_events: bool, price_config: &PriceConfig)
        -> LocationInfo;
}

impl MerchantRng for StdRng {
    fn gen_gold_recovered_from_pirate_encounter(&mut self, total_pirates: u8) -> u32 {
        self.gen_range(
            (GOLD_PER_PIRATE_VICTORY_MIN * (total_pirates as u32))
                ..(GOLD_PER_PIRATE_VICTORY_MAX * (total_pirates as u32)),
        )
    }

    fn gen_damage_from_pirates(&mut self, cur_pirates: u8) -> u8 {
        // make lower damages more likely.
        // eg. the damage possibilities when there are 3 pirates will be [0, 1, 2, 3]
        // and the weights corresponding to those damage possibilities are [4, 3, 2, 1]
        let damage_possibilities: Vec<u8> = (0..=cur_pirates).collect();
        let weights: Vec<usize> = damage_possibilities
            .iter()
            .enumerate()
            .map(|(i, _)| i + 1)
            .rev()
            .collect();
        let dist = WeightedIndex::new(weights).unwrap();
        damage_possibilities[dist.sample(self)]
    }

    fn gen_did_kill_a_pirate(&mut self, cannons: u8) -> bool {
        let kill_a_pirate_possibilities = [false, true];
        let weights = [1, cannons];
        let dist = WeightedIndex::new(weights).unwrap();
        kill_a_pirate_possibilities[dist.sample(self)]
    }

    fn gen_run_success(&mut self, cur_pirates: u8) -> bool {
        let run_success_chance = logarithmic_decay(cur_pirates as u32, 0.5);
        let random_value: f64 = self.gen();
        random_value < run_success_chance
    }

    fn gen_num_pirates_encountered(&mut self) -> u8 {
        self.gen_range(2..=4)
    }

    // TODO gen_range panics if range is empty
    fn gen_good_stolen(&mut self, goods_with_inventory: &[(Good, u32)]) -> (Good, u32) {
        let index = self.gen_range(0..goods_with_inventory.len());
        // safe unwrap, we generated the index to be in range
        let good_to_steal = goods_with_inventory.get(index).unwrap();
        // choose some amount of good to steal
        let amount = self.gen_range(1..good_to_steal.1);
        (good_to_steal.0, amount)
    }

    fn gen_location_info(
        &mut self,
        allow_events: bool,
        price_config: &PriceConfig,
    ) -> LocationInfo {
        let mut location_info = LocationInfo::empty();
        location_info.prices = randomized_inventory(self, price_config);
        if allow_events {
            let event_possibilities: [u8; 9] = [
                0, // no event
                1, // cheap good
                2, // expensive good
                3, // find goods
                4, // stolen goods
                5, // can buy cannon
                6, // pirate encounter
                7, // can buy more hold space
                8, // no effect
            ];
            let weights: [u8; 9] = [6, 1, 1, 1, 1, 1, 1, 1, 1];
            let dist = WeightedIndex::new(weights).unwrap();
            location_info.event = match event_possibilities[dist.sample(self)] {
                // no event
                0 => None,
                // cheap good
                1 => {
                    let good = Good::random(self);
                    // update location prices
                    let good_price = location_info.prices.get_good_mut(&good);
                    *good_price = ((*good_price as f64) * 0.5).floor() as u32;
                    Some(LocationEvent::CheapGood(good))
                }
                // expensive good
                2 => {
                    let good = Good::random(self);
                    // update location prices
                    let good_price = location_info.prices.get_good_mut(&good);
                    *good_price = ((*good_price as f64) * 2.0).floor() as u32;
                    Some(LocationEvent::ExpensiveGood(good))
                }
                // find goods
                3 => {
                    let good = Good::random(self);
                    let amount = (self.next_u32() % 10) + 1;
                    Some(LocationEvent::FindGoods(good, amount))
                }
                // stolen goods
                4 => Some(LocationEvent::GoodsStolen(None)),
                // can buy cannon
                5 => Some(LocationEvent::CanBuyCannon),
                // pirate encounter
                6 => Some(LocationEvent::PirateEncounter(
                    super::PirateEncounterState::Initial,
                )),
                // can buy more hold space
                7 => {
                    let price: u32 = self.gen_range(500..1500);
                    let more_hold: u32 = self.gen_range(65..130);
                    Some(LocationEvent::CanBuyHoldSpace { price, more_hold })
                }
                // no effect
                8 => {
                    let no_effect_event_possibilities: [NoEffectEvent; 2] =
                        [NoEffectEvent::SunnyDay, NoEffectEvent::StormOnHorizon];
                    let weights: [u8; 2] = [1, 1];
                    let dist = WeightedIndex::new(weights).unwrap();
                    let no_effect_event = no_effect_event_possibilities[dist.sample(self)];
                    Some(LocationEvent::NoEffect(no_effect_event))
                }
                _ => unreachable!(),
            };
        };
        location_info
    }
}

pub fn randomized_inventory(rng: &mut StdRng, config: &PriceConfig) -> Inventory {
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

fn logarithmic_decay(count: u32, decay_factor: f64) -> f64 {
    let initial_probability: f64 = 1.0; // 100%
    let decayed = initial_probability - decay_factor * (count as f64 + 1.0).ln();
    decayed + (initial_probability - decayed) / 2f64
}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;

    use super::*;

    #[test]
    fn gen_gold_recovered_from_pirate_encounter() {
        assert_eq!(
            StdRng::seed_from_u64(42).gen_gold_recovered_from_pirate_encounter(3),
            2100
        );
    }

    #[test]
    fn gen_damange_from_pirates() {
        assert_eq!(StdRng::seed_from_u64(42).gen_damage_from_pirates(3), 1);
    }

    #[test]
    fn gen_did_kill_a_pirate() {
        assert!(!StdRng::seed_from_u64(42).gen_did_kill_a_pirate(3));
    }

    #[test]
    fn gen_run_success() {
        assert!(StdRng::seed_from_u64(42).gen_run_success(3));
    }

    #[test]
    fn gen_num_pirates_encountered() {
        assert_eq!(StdRng::seed_from_u64(42).gen_num_pirates_encountered(), 2);
    }

    #[test]
    fn gen_location_info() {
        assert_eq!(
            StdRng::seed_from_u64(42).gen_location_info(
                true,
                &PriceConfig {
                    starting_gold: 500,
                    tea: (10.0, 14.0),
                    coffee: (4.25, 6.0),
                    sugar: (1.0, 2.2),
                    tobacco: (0.15, 0.35),
                    rum: (0.04, 0.14),
                    cotton: (0.005, 0.025),
                }
            ),
            LocationInfo {
                prices: Inventory {
                    tea: 5626,
                    coffee: 2976,
                    sugar: 897,
                    tobacco: 102,
                    rum: 59,
                    cotton: 7
                },
                event: Some(LocationEvent::NoEffect(NoEffectEvent::SunnyDay))
            }
        );
    }
}
