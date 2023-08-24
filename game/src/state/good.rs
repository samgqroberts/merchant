use std::fmt::{self, Display};

use rand::{rngs::StdRng, RngCore};

#[derive(Debug, PartialEq, Clone)]
pub enum Good {
    Tea,
    Coffee,
    Sugar,
    Tobacco,
    Rum,
    Cotton,
}

impl Display for Good {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Good::Tea => "Tea",
            Good::Coffee => "Coffee",
            Good::Sugar => "Sugar",
            Good::Tobacco => "Tobacco",
            Good::Rum => "Rum",
            Good::Cotton => "Cotton",
        };
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", string)
    }
}

impl Good {
    pub fn random(rng: &mut StdRng) -> Good {
        match rng.next_u32() % 6 {
            0 => Good::Tea,
            1 => Good::Coffee,
            2 => Good::Sugar,
            3 => Good::Tobacco,
            4 => Good::Rum,
            _ => Good::Cotton,
        }
    }

    pub fn variants() -> impl Iterator<Item = &'static Good> {
        static VARIANTS: &'static [Good] = &[
            Good::Tea,
            Good::Coffee,
            Good::Sugar,
            Good::Tobacco,
            Good::Rum,
            Good::Cotton,
        ];
        VARIANTS.iter()
    }
}
