
use std::collections::HashMap;
use std::marker::PhantomData;

use cornflake::{Config, CornFlake};

#[derive(Debug)]
pub struct Id<T>(u64, PhantomData<T>);

impl<T> Id<T> {
    pub fn new(data: u64) -> Self {
        Id::<T>(data, PhantomData)
    }
}
impl<T> std::cmp::PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}

use std::hash::{Hash, Hasher};

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

// TODO: CornFlake currently does not implement Clone, but IdMap<T> should!
#[derive(Debug)]
pub struct IdMap<T> {
    map: HashMap<Id<T>, T>,
    generator: CornFlake,
}

impl<T> IdMap<T> {
    pub fn new() -> Self {
        let config: Config = Default::default();
        let generator = CornFlake::new(&config).unwrap();
        Self {
            map: HashMap::new(),
            generator
        }
    }

    pub fn add(&mut self, value: T) -> Id<T> {
        let key = Id::new(self.generator.next_id().unwrap());
        self.map.insert(key.clone(), value);
        key
    }
}
