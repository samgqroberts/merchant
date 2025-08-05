use std::fmt::{self, Display};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Location {
    London,
    Savannah,
    Lisbon,
    Amsterdam,
    CapeTown,
    Venice,
}

impl Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Location::London => "London",
            Location::Savannah => "Savannah",
            Location::Lisbon => "Lisbon",
            Location::Amsterdam => "Amsterdam",
            Location::CapeTown => "Cape Town",
            Location::Venice => "Venice",
        };
        write!(f, "{}", string)
    }
}

static VARIANTS: &[Location] = &[
    Location::London,
    Location::Savannah,
    Location::Lisbon,
    Location::Amsterdam,
    Location::CapeTown,
    Location::Venice,
];

impl Location {
    pub fn variants() -> &'static [Location] {
        VARIANTS
    }
}
