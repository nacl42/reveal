
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};
use crate::flake;

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

