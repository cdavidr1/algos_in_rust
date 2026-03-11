use std::collections::HashMap;

fn main() {}

struct Cache {
    map: HashMap<String, usize>,
}

impl Cache {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    fn insert(&mut self, key: String, value: usize) {
        self.map.insert(key, value);
    }
    fn get(&self, key: &str) -> Option<&usize> {
        self.map.get(key)
    }
}
