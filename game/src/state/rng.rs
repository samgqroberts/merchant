use rand::{
    distributions::{Distribution, WeightedIndex},
    rngs::StdRng,
    Rng, RngCore,
};
use tracing::{debug, instrument};

use crate::state::goods_map::GoodsMap;

use super::Inventory;
use super::{
    constants::{GOLD_PER_PIRATE_VICTORY_MAX, GOLD_PER_PIRATE_VICTORY_MIN},
    game_state::LocationEvent,
    game_state::NoEffectEvent,
    game_state::PirateEncounterState,
    LocationInfo,
};
use super::{Good, PriceRanges};

/// A trait that abstracts the pieces of logic that need to use some kind of random number generation.
/// Allows injecting a mocked (deterministic) implementation in testing.
pub trait MerchantRng {
    fn gen_gold_recovered_from_pirate_encounter(&mut self, total_pirates: u8) -> u32;
    fn gen_damage_from_pirates(&mut self, cur_pirates: u8) -> u8;
    fn gen_did_kill_a_pirate(&mut self, cannons: u8) -> bool;
    fn gen_run_success(&mut self, cur_pirates: u8) -> bool;
    fn gen_num_pirates_encountered(&mut self) -> u8;
    fn gen_good_stolen(&mut self, goods_with_inventory: &[(Good, u32)]) -> (Good, u32);
    fn gen_location_info(
        &mut self,
        allow_events: bool,
        price_config: &PriceRanges,
        player_net_worth: i32,
    ) -> LocationInfo;
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
        price_config: &PriceRanges,
        player_net_worth: i32,
    ) -> LocationInfo {
        let mut location_info = LocationInfo::empty();
        location_info.prices = price_config.randomized_inventory(self);
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
                1 => Some(gen_cheap_good(self, price_config, &mut location_info)),
                // expensive good
                2 => Some(gen_expensive_good(self, price_config, &mut location_info)),
                // find goods
                3 => Some(gen_find_goods(self, price_config, player_net_worth)),
                // stolen goods
                4 => Some(LocationEvent::GoodsStolen(None)),
                // can buy cannon
                5 => Some(LocationEvent::CanBuyCannon),
                // pirate encounter
                6 => Some(LocationEvent::PirateEncounter(
                    PirateEncounterState::Initial,
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

#[instrument(level = "debug", skip_all)]
fn gen_cheap_good(
    rng: &mut StdRng,
    price_config: &PriceRanges,
    location_info: &mut LocationInfo,
) -> LocationEvent {
    // generate a low price for a random good
    // that's some amount lower than the lowest price the good can be
    let good = Good::random(rng);
    const MULTIPLES: GoodsMap<f32> = GoodsMap {
        tea: 0.85,
        coffee: 0.8,
        sugar: 0.75,
        tobacco: 0.7,
        rum: 0.65,
        cotton: 0.6,
    };
    let normal_low = price_config.get_good(&good).0;
    let mut lowest_low = (MULTIPLES.get_good(&good) * (normal_low as f32)).round() as u32;
    if lowest_low == 0 {
        lowest_low += 1;
    };
    let cheap_price = rng.next_u32() % (normal_low - lowest_low) + lowest_low;
    debug!("generated cheap price {cheap_price} for good {good} from range of {lowest_low} - {normal_low} (normal range {normal_low} - {})", price_config.get_good(&good).1);
    let good_price = location_info.prices.get_good_mut(&good);
    *good_price = cheap_price;
    LocationEvent::CheapGood(good)
}

#[instrument(level = "debug", skip_all)]
fn gen_expensive_good(
    rng: &mut StdRng,
    price_config: &PriceRanges,
    location_info: &mut LocationInfo,
) -> LocationEvent {
    // generate a high price for a random good
    // that's some amount high than the highest price the good can be
    let good = Good::random(rng);
    const MULTIPLES: GoodsMap<f32> = GoodsMap {
        tea: 1.15,
        coffee: 1.2,
        sugar: 1.25,
        tobacco: 1.3,
        rum: 1.35,
        cotton: 1.4,
    };
    let normal_high = price_config.get_good(&good).1;
    let mut highest_high = (MULTIPLES.get_good(&good) * (normal_high as f32)).round() as u32;
    if highest_high > 9999 {
        highest_high = 9999;
    };
    let expensive_price = rng.next_u32() % (highest_high - normal_high) + normal_high;
    debug!("generated expensive price {expensive_price} for good {good} from range of {normal_high} - {highest_high} (normal range {} - {normal_high})", price_config.get_good(&good).0);
    let good_price = location_info.prices.get_good_mut(&good);
    *good_price = expensive_price;
    LocationEvent::ExpensiveGood(good)
}

#[instrument(level = "debug", skip_all)]
fn gen_find_goods(
    rng: &mut StdRng,
    price_config: &PriceRanges,
    player_net_worth: i32,
) -> LocationEvent {
    debug!("player_net_worth: {}", player_net_worth);
    // for all goods, get a "comparison price"
    // where the good whose comparison price that the player's net worth is closest to
    // represents the current phase of the player's progression.
    // eg. if the multiple is 30, and the player's net worth is closest to 30 * the avg price of rum,
    //     then the player is in the "rum" phase of the game.
    const COMPARISON_PRICE_MULTIPLE: u32 = 30;
    let comparison_prices = price_config
        .avg_prices()
        .map(|x| x * COMPARISON_PRICE_MULTIPLE);
    debug!("comparison_prices: {}", comparison_prices);
    // compare how close the player's current net worth is to the "comparison price"
    // of each good to produce a probability weight for the user to find that good.
    // eg. it should be impossible to find tea if the player has a very low net worth.
    let net_worth: u32 = if player_net_worth < 0 {
        0
    } else {
        player_net_worth as u32
    };
    let comparison_distances = Inventory {
        tea: comparison_prices.tea.abs_diff(net_worth),
        coffee: comparison_prices.coffee.abs_diff(net_worth),
        sugar: comparison_prices.sugar.abs_diff(net_worth),
        tobacco: comparison_prices.tobacco.abs_diff(net_worth),
        rum: comparison_prices.rum.abs_diff(net_worth),
        cotton: comparison_prices.cotton.abs_diff(net_worth),
    };
    debug!("comparison_distances: {}", comparison_distances);
    let min_good = comparison_distances.min_good();
    debug!("closest comparison good (phase of game): {}", min_good);
    let weights: [u8; 6] = match min_good {
        Good::Tea => [3, 2, 1, 1, 1, 1], // 33% chance tea, 22% coffee, 11% sugar tobacco rum cotton
        Good::Coffee => [2, 3, 2, 1, 1, 1],
        Good::Sugar => [1, 2, 3, 2, 1, 1],
        Good::Tobacco => [0, 1, 2, 3, 2, 1],
        Good::Rum => [0, 0, 1, 2, 3, 2],
        Good::Cotton => [0, 0, 0, 1, 2, 3], // 0% chance tea coffee sugar, 17% tobacco, 33% rum, 50% cotton
    };
    debug!("weights: {:?}", weights);
    let dist = WeightedIndex::new(weights).unwrap();
    const GOODS_SLICE: [Good; 6] = [
        Good::Tea,
        Good::Coffee,
        Good::Sugar,
        Good::Tobacco,
        Good::Rum,
        Good::Cotton,
    ];
    let good = GOODS_SLICE[dist.sample(rng)];
    let amount = (rng.next_u32() % 10) + 1;
    LocationEvent::FindGoods(good, amount)
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
                &PriceRanges {
                    tea: (4253, 7442),
                    coffee: (2166, 4332),
                    sugar: (714, 1785),
                    tobacco: (184, 551),
                    rum: (35, 140),
                    cotton: (5, 30)
                },
                10000
            ),
            LocationInfo {
                prices: Inventory {
                    tea: 4926,
                    coffee: 2425,
                    sugar: 1411,
                    tobacco: 290,
                    rum: 59,
                    cotton: 15
                },
                event: Some(LocationEvent::NoEffect(NoEffectEvent::SunnyDay))
            }
        );
    }
}
