use std::collections::HashMap;
use crate::id::Id;

#[derive(Debug)]
pub struct IdMap<T> {
    map: HashMap<Id<T>, T>,
}

impl<T> IdMap<T> {

    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add(&mut self, value: T) -> Id<T> {
        let key = Id::new();
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
