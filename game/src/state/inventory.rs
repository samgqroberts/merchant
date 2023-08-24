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

    pub fn good_amount(&self, good_type: &Good) -> u32 {
        match good_type {
            Good::Tea => self.tea,
            Good::Coffee => self.coffee,
            Good::Sugar => self.sugar,
            Good::Tobacco => self.tobacco,
            Good::Rum => self.rum,
            Good::Cotton => self.cotton,
        }
    }

    pub fn total_amount(&self) -> u32 {
        let mut total: u32 = 0;
        for good in Good::variants() {
            total += self.good_amount(good);
        }
        total
    }

    pub fn add_good(&self, good: &Good, amount: u32) -> Inventory {
        let mut new_inventory = self.clone();
        match good {
            Good::Tea => new_inventory.tea += amount,
            Good::Coffee => new_inventory.coffee += amount,
            Good::Sugar => new_inventory.sugar += amount,
            Good::Tobacco => new_inventory.tobacco += amount,
            Good::Rum => new_inventory.rum += amount,
            Good::Cotton => new_inventory.cotton += amount,
        }
        new_inventory
    }

    pub fn remove_good(&self, good: &Good, amount: u32) -> Inventory {
        let mut new_inventory = self.clone();
        match good {
            Good::Tea => new_inventory.tea -= amount,
            Good::Coffee => new_inventory.coffee -= amount,
            Good::Sugar => new_inventory.sugar -= amount,
            Good::Tobacco => new_inventory.tobacco -= amount,
            Good::Rum => new_inventory.rum -= amount,
            Good::Cotton => new_inventory.cotton -= amount,
        }
        new_inventory
    }

    pub fn get_good_mut(&mut self, good: &Good) -> &mut u32 {
        match good {
            Good::Tea => &mut self.tea,
            Good::Coffee => &mut self.coffee,
            Good::Sugar => &mut self.sugar,
            Good::Tobacco => &mut self.tobacco,
            Good::Rum => &mut self.rum,
            Good::Cotton => &mut self.cotton,
        }
    }
}
