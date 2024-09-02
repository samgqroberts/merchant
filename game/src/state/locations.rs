use std::{collections::HashMap, rc::Rc};

use rand::{rngs::StdRng, RngCore};

use super::{Good, Inventory, Location, LocationEvent, MerchantRng};

/// For each good type, define the lowest and highest value the price for that good can be.
#[derive(Clone, Debug)]
pub struct PriceRanges {
    pub tea: (u32, u32),
    pub coffee: (u32, u32),
    pub sugar: (u32, u32),
    pub tobacco: (u32, u32),
    pub rum: (u32, u32),
    pub cotton: (u32, u32),
}

impl std::fmt::Display for PriceRanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prev_avg: Option<f32> = None;
        for (good, (low, high)) in self.into_iter().rev() {
            let spread = (high as f32 - low as f32) / low as f32;
            let spread = format!("{:.0}", 100.0 * spread);
            let avg = (high as f32 + low as f32) / 2.0;
            let over_last = if let Some(prev_avg) = prev_avg {
                format!(", {:.0}% of prev avg", 100.0 * (avg / prev_avg))
            } else {
                "".to_owned()
            };
            prev_avg = Some(avg);
            write!(
                f,
                "[{}: {}-{} ({}% spread{})]",
                good, low, high, spread, over_last
            )?;
        }
        Ok(())
    }
}

impl<'a> IntoIterator for &'a PriceRanges {
    type Item = (Good, (u32, u32));
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let v = vec![
            (Good::Tea, self.tea),
            (Good::Coffee, self.coffee),
            (Good::Sugar, self.sugar),
            (Good::Tobacco, self.tobacco),
            (Good::Rum, self.rum),
            (Good::Cotton, self.cotton),
        ];
        v.into_iter()
    }
}

impl From<HashMap<Good, (u32, u32)>> for PriceRanges {
    fn from(mut value: HashMap<Good, (u32, u32)>) -> Self {
        PriceRanges {
            tea: value
                .remove(&Good::Tea)
                .expect("expectation failed: tea present in hashmap"),
            coffee: value
                .remove(&Good::Coffee)
                .expect("expectation failed: coffee present in hashmap"),
            sugar: value
                .remove(&Good::Sugar)
                .expect("expectation failed: sugar present in hashmap"),
            tobacco: value
                .remove(&Good::Tobacco)
                .expect("expectation failed: tobacco present in hashmap"),
            rum: value
                .remove(&Good::Rum)
                .expect("expectation failed: rum present in hashmap"),
            cotton: value
                .remove(&Good::Cotton)
                .expect("expectation failed: cotton present in hashmap"),
        }
    }
}

impl FromIterator<(Good, (u32, u32))> for PriceRanges {
    fn from_iter<T: IntoIterator<Item = (Good, (u32, u32))>>(iter: T) -> Self {
        iter.into_iter()
            .collect::<HashMap<Good, (u32, u32)>>()
            .into()
    }
}

impl PriceRanges {
    pub fn randomized_inventory(&self, rng: &mut StdRng) -> Inventory {
        let mut gen = |(low, high): (u32, u32)| -> u32 { rng.next_u32() % (high - low) + low };
        Inventory {
            tea: gen(self.tea),
            coffee: gen(self.coffee),
            sugar: gen(self.sugar),
            tobacco: gen(self.tobacco),
            rum: gen(self.rum),
            cotton: gen(self.cotton),
        }
    }

    pub fn avg_prices(&self) -> Inventory {
        Inventory {
            tea: (self.tea.0 + self.tea.1).div_ceil(2),
            coffee: (self.coffee.0 + self.coffee.1).div_ceil(2),
            sugar: (self.sugar.0 + self.sugar.1).div_ceil(2),
            tobacco: (self.tobacco.0 + self.tobacco.1).div_ceil(2),
            rum: (self.rum.0 + self.rum.1).div_ceil(2),
            cotton: (self.cotton.0 + self.cotton.1).div_ceil(2),
        }
    }

    pub(crate) fn from_start_price_and_spreads(
        cotton_low: u32,
        spreads: [f32; 6],
        avg_proportions: [f32; 5],
    ) -> Self {
        let cotton_high = (cotton_low as f32 * (1.0 + spreads[0])).ceil() as u32;
        let cotton_avg = (cotton_low + cotton_high) as f32 / 2.0;
        let mut last = cotton_avg;
        let mut averages = vec![cotton_avg];
        for avg_proportion in avg_proportions {
            let new_avg = last * (avg_proportion);
            averages.push(new_avg);
            last = new_avg;
        }
        const GOODS: [Good; 6] = [
            Good::Cotton,
            Good::Rum,
            Good::Tobacco,
            Good::Sugar,
            Good::Coffee,
            Good::Tea,
        ];
        averages
            .into_iter()
            .zip(spreads)
            .zip(GOODS)
            .map(|((avg, spread), good)| {
                let high = (2.0 * avg * (spread + 1.0)) / (spread + 2.0);
                let low = high / (spread + 1.0);
                (good, (low.round() as u32, high.round() as u32))
            })
            .collect::<PriceRanges>()
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
    pub config: Rc<PriceRanges>,
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
        price_config: Rc<PriceRanges>,
        player_net_worth: i32,
    ) -> Locations {
        let mut res = Locations {
            config: price_config,
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
                player_net_worth,
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
