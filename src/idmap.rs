use std::collections::HashMap;
use std::collections::hash_map::{Iter, IterMut, Values, ValuesMut};
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};

use crate::flake;


#[derive(Debug)]
pub enum IdMapError {
    IdDoesNotExist
}

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

    /// For the entry with the given `id`, replace its
    /// value with the new `value`. The old value is returned.
    /// If there was no old value, the function does not
    /// insert the new value but returns Err.
    pub fn replace(&mut self, id: &Id<T>, value: T) -> Result<T, IdMapError>
    {
        if let Some(old_value)= self.map.remove(&id) {
            self.map.insert(id.clone(), value);
            Ok(old_value)
        } else {
            Err(IdMapError::IdDoesNotExist)
        }
    }
    
    /// Remove the value with the given key from the IdMap.
    pub fn remove(&mut self, id: &Id<T>) -> Option<T>{
        self.map.remove(&id)
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




#[derive(Debug)]
pub struct Id<T>(u64, PhantomData<T>);

impl<T> Id<T> {
    pub fn new() -> Self {
        Id::<T>(flake::new_flake().unwrap(), PhantomData)
    }
}
impl<T> std::cmp::PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}


impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Copy for Id<T> {}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

// TODO: write tests for Id and IdMap
#[cfg(tests)]
mod tests {
    #[test]
    fn test_new() {
        let map = IdMap::<String>::new();
        
    }
}
