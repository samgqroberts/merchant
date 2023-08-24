use std::fmt::{self, Display};

#[derive(Clone, Debug, PartialEq)]
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

impl Location {
    pub fn variants() -> impl Iterator<Item = &'static Location> {
        static VARIANTS: &'static [Location] = &[
            Location::London,
            Location::Savannah,
            Location::Lisbon,
            Location::Amsterdam,
            Location::CapeTown,
            Location::Venice,
        ];
        VARIANTS.iter()
    }
}
