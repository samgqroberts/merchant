use std::fmt::{self, Display};

use rand::{rngs::StdRng, RngCore};

#[derive(Debug, PartialEq, Clone)]
pub enum GoodType {
    Tea,
    Coffee,
    Sugar,
    Tobacco,
    Rum,
    Cotton,
}

pub const GOOD_TYPES: &'static [GoodType] = &[
    GoodType::Tea,
    GoodType::Coffee,
    GoodType::Sugar,
    GoodType::Tobacco,
    GoodType::Rum,
    GoodType::Cotton,
];

impl Display for GoodType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            GoodType::Tea => "Tea",
            GoodType::Coffee => "Coffee",
            GoodType::Sugar => "Sugar",
            GoodType::Tobacco => "Tobacco",
            GoodType::Rum => "Rum",
            GoodType::Cotton => "Cotton",
        };
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", string)
    }
}

impl GoodType {
    pub fn random(rng: &mut StdRng) -> GoodType {
        match rng.next_u32() % 6 {
            0 => GoodType::Tea,
            1 => GoodType::Coffee,
            2 => GoodType::Sugar,
            3 => GoodType::Tobacco,
            4 => GoodType::Rum,
            _ => GoodType::Cotton,
        }
    }
}
