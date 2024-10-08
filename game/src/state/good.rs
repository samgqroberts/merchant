use std::fmt::{self, Display};

use rand::{rngs::StdRng, seq::SliceRandom, RngCore};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
        write!(f, "{}", string)
    }
}

static VARIANTS: &[Good] = &[
    Good::Tea,
    Good::Coffee,
    Good::Sugar,
    Good::Tobacco,
    Good::Rum,
    Good::Cotton,
];

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

    pub fn variants() -> &'static [Good] {
        VARIANTS
    }

    pub fn variants_iter() -> impl Iterator<Item = &'static Good> {
        Self::variants().iter()
    }

    pub fn variants_random_order(rng: &mut StdRng) -> Vec<Good> {
        let mut variants = Self::variants().to_vec();
        variants.shuffle(rng);
        variants
    }
}
