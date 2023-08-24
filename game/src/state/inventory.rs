use super::Good;

#[derive(Clone, Debug)]
pub struct Inventory {
    pub tea: u32,
    pub coffee: u32,
    pub sugar: u32,
    pub tobacco: u32,
    pub rum: u32,
    pub cotton: u32,
}

impl Inventory {
    pub fn new() -> Inventory {
        Inventory {
            tea: 0,
            coffee: 0,
            sugar: 0,
            tobacco: 0,
            rum: 0,
            cotton: 0,
        }
    }

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
        *good = *good + amount;
        good
    }

    pub fn remove_good(&mut self, good: &Good, amount: u32) -> &u32 {
        let good = self.get_good_mut(good);
        *good = *good - amount;
        good
    }
}
