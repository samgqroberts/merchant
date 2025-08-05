use rand::{rngs::StdRng, RngCore};

use super::{goods_map::GoodsMap, Good, Inventory};

/// For each good type, define the lowest and highest value the price for that good can be.
pub type PriceRanges = GoodsMap<(u32, u32)>;

impl std::fmt::Display for PriceRanges {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut prev_avg: Option<f32> = None;
        for (good, (low, high)) in self.into_iter().rev() {
            let low = *low;
            let high = *high;
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

    pub fn from_start_price_and_spreads(
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

    pub fn generate_subsection(&self, cheap: Option<Good>, expensive: Option<Good>) -> Self {
        self.iter()
            .map(|(good, (overall_low, overall_high))| {
                if cheap.is_none() && expensive.is_none() {
                    // this is a "boring" location, read: home port
                    let low = ((((overall_high - overall_low) as f64) * 0.4) + *overall_low as f64)
                        .ceil() as u32;
                    let high = ((((overall_high - overall_low) as f64) * 0.6) + *overall_low as f64)
                        .ceil() as u32;
                    (good, (low, high))
                } else if cheap.map(|x| x == good).unwrap_or(false) {
                    let low = *overall_low;
                    let high = ((((overall_high - overall_low) as f64) * 0.6) + *overall_low as f64)
                        .ceil() as u32;
                    (good, (low, high))
                } else if expensive.map(|x| x == good).unwrap_or(false) {
                    let low = ((((overall_high - overall_low) as f64) * 0.4) + *overall_low as f64)
                        .ceil() as u32;
                    let high = *overall_high;
                    (good, (low, high))
                } else {
                    // this isn't a boring port, but neither is this good cheap or expensive
                    let low = ((((overall_high - overall_low) as f64) * 0.2) + *overall_low as f64)
                        .ceil() as u32;
                    let high = ((((overall_high - overall_low) as f64) * 0.8) + *overall_low as f64)
                        .ceil() as u32;
                    (good, (low, high))
                }
            })
            .collect()
    }
}
