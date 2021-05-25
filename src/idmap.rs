use std::collections::HashMap;
use std::collections::hash_map::{Iter, IterMut, Values, ValuesMut};

use crate::id::Id;

#[derive(Debug)]
pub struct IdMap<T> {
    map: HashMap<Id<T>, T>,
}

impl<T> IdMap<T> {

    /// Set up a new, empty IdMap.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Insert the given value of type T into the IdMap.
    /// Returns the newly created key which is of type Id<T>.
    #[allow(dead_code)]
    pub fn add(&mut self, value: T) -> Id<T> {
        let key = Id::new();
        self.map.insert(key.clone(), value);
        key
    }

    /// Get a reference to the value with the given id.
    pub fn get(&self, id: &Id<T>) -> Option<&T> {
        self.map.get(id)
    }

    /// Get a mutable reference to the value with the given id.
    pub fn get_mut(&mut self, id: &Id<T>) -> Option<&mut T> {
        self.map.get_mut(id)
    }

    /// Return an iterator over all (id, value) pairs.
    pub fn iter(&self) -> Iter<'_, Id<T>, T> {
        self.map.iter()
    }

    /// Return a mutable iterator over all (id, value) pairs
    pub fn iter_mut(&mut self) -> IterMut<'_, Id<T>, T> {
        self.map.iter_mut()
    }

    /// Return an iterator over all values.
    pub fn values(&self) -> Values<'_, Id<T>, T> {
        self.map.values()
    }

    /// Return a mutable iterator over all values.
    pub fn values_mut(&mut self) -> ValuesMut<'_, Id<T>, T> {
        self.map.values_mut()
    }
}


impl <'a, T> IntoIterator for &'a IdMap<T> {
    type Item = (&'a Id<T>, &'a T);
    type IntoIter = Iter<'a, Id<T>, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}


impl <'a, T> IntoIterator for &'a mut IdMap<T> {
    type Item = (&'a Id<T>, &'a mut T);
    type IntoIter = IterMut<'a, Id<T>, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter_mut()
    }
}



// TODO: write tests for IdMap
#[cfg(tests)]
mod tests {
    #[test]
    fn test_new() {
        let map = IdMap::<String>::new();
        
    }
}
