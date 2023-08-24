use super::{good::GOOD_TYPES, GoodType};

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

    pub fn good_amount(&self, good_type: &GoodType) -> u32 {
        match good_type {
            GoodType::Tea => self.tea,
            GoodType::Coffee => self.coffee,
            GoodType::Sugar => self.sugar,
            GoodType::Tobacco => self.tobacco,
            GoodType::Rum => self.rum,
            GoodType::Cotton => self.cotton,
        }
    }

    pub fn total_amount(&self) -> u32 {
        let mut total: u32 = 0;
        for good in GOOD_TYPES {
            total += self.good_amount(good);
        }
        total
    }

    pub fn add_good(&self, good: &GoodType, amount: u32) -> Inventory {
        let mut new_inventory = self.clone();
        match good {
            GoodType::Tea => new_inventory.tea += amount,
            GoodType::Coffee => new_inventory.coffee += amount,
            GoodType::Sugar => new_inventory.sugar += amount,
            GoodType::Tobacco => new_inventory.tobacco += amount,
            GoodType::Rum => new_inventory.rum += amount,
            GoodType::Cotton => new_inventory.cotton += amount,
        }
        new_inventory
    }

    pub fn remove_good(&self, good: &GoodType, amount: u32) -> Inventory {
        let mut new_inventory = self.clone();
        match good {
            GoodType::Tea => new_inventory.tea -= amount,
            GoodType::Coffee => new_inventory.coffee -= amount,
            GoodType::Sugar => new_inventory.sugar -= amount,
            GoodType::Tobacco => new_inventory.tobacco -= amount,
            GoodType::Rum => new_inventory.rum -= amount,
            GoodType::Cotton => new_inventory.cotton -= amount,
        }
        new_inventory
    }

    pub fn get_good_mut(&mut self, good: &GoodType) -> &mut u32 {
        match good {
            GoodType::Tea => &mut self.tea,
            GoodType::Coffee => &mut self.coffee,
            GoodType::Sugar => &mut self.sugar,
            GoodType::Tobacco => &mut self.tobacco,
            GoodType::Rum => &mut self.rum,
            GoodType::Cotton => &mut self.cotton,
        }
    }
}
