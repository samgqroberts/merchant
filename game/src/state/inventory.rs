use super::{Good, PriceConfig};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Inventory {
    pub tea: u32,
    pub coffee: u32,
    pub sugar: u32,
    pub tobacco: u32,
    pub rum: u32,
    pub cotton: u32,
}

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
    pub fn get_good(&self, good_type: &Good) -> &u32 {
        match good_type {
            Good::Tea => &self.tea,
            Good::Coffee => &self.coffee,
            Good::Sugar => &self.sugar,
            Good::Tobacco => &self.tobacco,
            Good::Rum => &self.rum,
            Good::Cotton => &self.cotton,
        }
    }

    pub fn get_good_mut(&mut self, good_type: &Good) -> &mut u32 {
        match good_type {
            Good::Tea => &mut self.tea,
            Good::Coffee => &mut self.coffee,
            Good::Sugar => &mut self.sugar,
            Good::Tobacco => &mut self.tobacco,
            Good::Rum => &mut self.rum,
            Good::Cotton => &mut self.cotton,
        }
    }

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

    pub(crate) fn iter(&self) -> std::vec::IntoIter<(Good, u32)> {
        self.into_iter()
    }

    pub fn map<F>(&self, f: F) -> Inventory
    where
        F: Fn(u32) -> u32,
    {
        Inventory {
            tea: f(self.tea),
            coffee: f(self.coffee),
            sugar: f(self.sugar),
            tobacco: f(self.tobacco),
            rum: f(self.rum),
            cotton: f(self.cotton),
        }
    }

    /// computes the net worth of the amount of goods in this inventory according to the provided [PriceConfig].
    pub(crate) fn net_worth(&self, price_config: &PriceConfig) -> i32 {
        let avg_prices = price_config.avg_prices();
        ((self.tea * avg_prices.tea)
            + (self.coffee * avg_prices.coffee)
            + (self.sugar * avg_prices.sugar)
            + (self.tobacco * avg_prices.tobacco)
            + (self.rum * avg_prices.rum)
            + (self.cotton * avg_prices.cotton)) as i32
    }

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

impl<'a> IntoIterator for &'a Inventory {
    type Item = (Good, u32);
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
