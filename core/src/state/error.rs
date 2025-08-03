use super::{game_state::Mode, Location};
use std::fmt::{self, Display};

#[derive(Debug)]
pub enum StateError {
    #[allow(dead_code)]
    InvalidMode(Mode),
    CannotAfford,
    InsufficientHold,
    InsufficientInventory,
    InsufficientStash,
    AlreadyInLocation,
    #[allow(dead_code)]
    LocationNotHomeBase(Location),
    PayDownAmountHigherThanDebt,
    InsufficientBank,
}

impl Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
