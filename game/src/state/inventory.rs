use super::{goods_map::GoodsMap, PriceRanges, Good};

pub type Inventory = GoodsMap<u32>;

impl std::fmt::Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{tea:{},coffee:{},sugar:{},tobacco:{},rum:{},cotton:{}}}",
            self.tea, self.coffee, self.sugar, self.tobacco, self.rum, self.cotton
        )
    }
}

impl Inventory {
    pub fn total_amount(&self) -> u32 {
        let mut total: u32 = 0;
        for good in Good::variants() {
            total += self.get_good(good);
        }
        total
    }

    pub fn add_good(&mut self, good: &Good, amount: u32) -> &u32 {
        let good = self.get_good_mut(good);
        *good += amount;
        good
    }

    pub fn remove_good(&mut self, good: &Good, amount: u32) -> &u32 {
        let good = self.get_good_mut(good);
        *good -= amount;
        good
    }

    /// computes the net worth of the amount of goods in this inventory according to the provided [PriceRanges].
    pub(crate) fn net_worth(&self, price_config: &PriceRanges) -> i32 {
        let avg_prices = price_config.avg_prices();
        ((self.tea * avg_prices.tea)
            + (self.coffee * avg_prices.coffee)
            + (self.sugar * avg_prices.sugar)
            + (self.tobacco * avg_prices.tobacco)
            + (self.rum * avg_prices.rum)
            + (self.cotton * avg_prices.cotton)) as i32
    }

    #[allow(dead_code)]
    pub(crate) fn max_good(&self) -> Good {
        self.iter()
            .max_by_key(|x| x.1)
            .expect("inventory iterators are always nonempty")
            .0
    }

    pub(crate) fn min_good(&self) -> Good {
        self.iter()
            .min_by_key(|x| x.1)
            .expect("inventory iterators are always nonempty")
            .0
    }
}
