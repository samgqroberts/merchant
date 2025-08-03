use std::{collections::HashMap, fmt};

use super::Good;

/// A collection of generic information about each Good type there is.
pub struct GoodsMap<T> {
    pub tea: T,
    pub coffee: T,
    pub sugar: T,
    pub tobacco: T,
    pub rum: T,
    pub cotton: T,
}

impl<T: Clone> Clone for GoodsMap<T> {
    fn clone(&self) -> Self {
        Self {
            tea: self.tea.clone(),
            coffee: self.coffee.clone(),
            sugar: self.sugar.clone(),
            tobacco: self.tobacco.clone(),
            rum: self.rum.clone(),
            cotton: self.cotton.clone(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for GoodsMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GoodsMap")
            .field("tea", &self.tea)
            .field("coffee", &self.coffee)
            .field("sugar", &self.sugar)
            .field("tobacco", &self.tobacco)
            .field("rum", &self.rum)
            .field("cotton", &self.cotton)
            .finish()
    }
}

impl<T: Default> Default for GoodsMap<T> {
    fn default() -> Self {
        Self {
            tea: Default::default(),
            coffee: Default::default(),
            sugar: Default::default(),
            tobacco: Default::default(),
            rum: Default::default(),
            cotton: Default::default(),
        }
    }
}

impl<T: PartialEq> PartialEq for GoodsMap<T> {
    fn eq(&self, other: &Self) -> bool {
        self.tea == other.tea
            && self.coffee == other.coffee
            && self.sugar == other.sugar
            && self.tobacco == other.tobacco
            && self.rum == other.rum
            && self.cotton == other.cotton
    }
}

impl<T> GoodsMap<T> {
    pub fn get_good(&self, good_type: &Good) -> &T {
        match good_type {
            Good::Tea => &self.tea,
            Good::Coffee => &self.coffee,
            Good::Sugar => &self.sugar,
            Good::Tobacco => &self.tobacco,
            Good::Rum => &self.rum,
            Good::Cotton => &self.cotton,
        }
    }

    pub fn get_good_mut(&mut self, good_type: &Good) -> &mut T {
        match good_type {
            Good::Tea => &mut self.tea,
            Good::Coffee => &mut self.coffee,
            Good::Sugar => &mut self.sugar,
            Good::Tobacco => &mut self.tobacco,
            Good::Rum => &mut self.rum,
            Good::Cotton => &mut self.cotton,
        }
    }

    pub(crate) fn iter(&self) -> std::vec::IntoIter<(Good, &T)> {
        self.into_iter()
    }

    pub fn map<F>(&self, f: F) -> GoodsMap<T>
    where
        F: Fn(&T) -> T,
    {
        GoodsMap {
            tea: f(&self.tea),
            coffee: f(&self.coffee),
            sugar: f(&self.sugar),
            tobacco: f(&self.tobacco),
            rum: f(&self.rum),
            cotton: f(&self.cotton),
        }
    }
}

impl<'a, T> IntoIterator for &'a GoodsMap<T> {
    type Item = (Good, &'a T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let v = vec![
            (Good::Tea, &self.tea),
            (Good::Coffee, &self.coffee),
            (Good::Sugar, &self.sugar),
            (Good::Tobacco, &self.tobacco),
            (Good::Rum, &self.rum),
            (Good::Cotton, &self.cotton),
        ];
        v.into_iter()
    }
}

impl<T> IntoIterator for GoodsMap<T> {
    type Item = (Good, T);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let v = vec![
            (Good::Tea, self.tea),
            (Good::Coffee, self.coffee),
            (Good::Sugar, self.sugar),
            (Good::Tobacco, self.tobacco),
            (Good::Rum, self.rum),
            (Good::Cotton, self.cotton),
        ];
        v.into_iter()
    }
}

impl <T> From<HashMap<Good, T>> for GoodsMap<T> {
    fn from(mut value: HashMap<Good, T>) -> Self {
        GoodsMap {
            tea: value
                .remove(&Good::Tea)
                .expect("expectation failed: tea present in hashmap"),
            coffee: value
                .remove(&Good::Coffee)
                .expect("expectation failed: coffee present in hashmap"),
            sugar: value
                .remove(&Good::Sugar)
                .expect("expectation failed: sugar present in hashmap"),
            tobacco: value
                .remove(&Good::Tobacco)
                .expect("expectation failed: tobacco present in hashmap"),
            rum: value
                .remove(&Good::Rum)
                .expect("expectation failed: rum present in hashmap"),
            cotton: value
                .remove(&Good::Cotton)
                .expect("expectation failed: cotton present in hashmap"),
        }
    }
}

impl <T> FromIterator<(Good, T)> for GoodsMap<T> {
    fn from_iter<U: IntoIterator<Item = (Good, T)>>(iter: U) -> Self {
        iter.into_iter()
            .collect::<HashMap<Good, T>>()
            .into()
    }
}