
use std::collections::HashMap;
use crate::id::Id;

use cornflake::{CornFlake, Config};

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



#[cfg(tests)]
mod tests {
    #[test]
    fn test_new() {
        let map = IdMap::<String>::new();
        
    }
}
