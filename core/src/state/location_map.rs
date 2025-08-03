use std::{collections::HashMap, fmt};

use super::Location;

/// A collection of generic information about each Location type there is.
pub struct LocationMap<T> {
    pub london: T,
    pub savannah: T,
    pub lisbon: T,
    pub amsterdam: T,
    pub capetown: T,
    pub venice: T,
}

impl<T: Clone> Clone for LocationMap<T> {
    fn clone(&self) -> Self {
        Self {
            london: self.london.clone(),
            savannah: self.savannah.clone(),
            lisbon: self.lisbon.clone(),
            amsterdam: self.amsterdam.clone(),
            capetown: self.capetown.clone(),
            venice: self.venice.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for LocationMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocationMap")
            .field("london", &self.london)
            .field("savannah", &self.savannah)
            .field("lisbon", &self.lisbon)
            .field("amsterdam", &self.amsterdam)
            .field("capetown", &self.capetown)
            .field("venice", &self.venice)
            .finish()
    }
}

impl<T> From<HashMap<Location, T>> for LocationMap<T> {
    fn from(mut value: HashMap<Location, T>) -> Self {
        LocationMap {
            london: value
                .remove(&Location::London)
                .expect("expectation failed: london present in hashmap"),
            savannah: value
                .remove(&Location::Savannah)
                .expect("expectation failed: savannah present in hashmap"),
            lisbon: value
                .remove(&Location::Lisbon)
                .expect("expectation failed: lisbon present in hashmap"),
            amsterdam: value
                .remove(&Location::Amsterdam)
                .expect("expectation failed: amsterdam present in hashmap"),
            capetown: value
                .remove(&Location::CapeTown)
                .expect("expectation failed: capetown present in hashmap"),
            venice: value
                .remove(&Location::Venice)
                .expect("expectation failed: venice present in hashmap"),
        }
    }
}

impl<T> FromIterator<(Location, T)> for LocationMap<T> {
    fn from_iter<U: IntoIterator<Item = (Location, T)>>(iter: U) -> Self {
        iter.into_iter().collect::<HashMap<Location, T>>().into()
    }
}

impl<T> LocationMap<T> {
    pub fn get(&self, location: &Location) -> &T {
        match location {
            Location::London => &self.london,
            Location::Savannah => &self.savannah,
            Location::Lisbon => &self.lisbon,
            Location::Amsterdam => &self.amsterdam,
            Location::CapeTown => &self.capetown,
            Location::Venice => &self.venice,
        }
    }
}
