use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
};

fn main() {
    let counter = SharedCounter::new();
    let c = &counter;
    println!("Init: {}", c.get());
    c.increment();
    c.increment();
    c.increment();
    println!("Cur: {}", c.get());

    let m = MutableMap::new();
    m.insert("apple".to_string(), 5);
    m.insert("orange".to_string(), 6);

    if let Some(val) = m.get("apple") {
        assert_eq!(*val, 5);
    };

    // without if let, guard is not dropped
    // runtime panic: RefCell already borrowed
    // let val = m.get("apple").unwrap(); // ← immutable borrow acquired
    // assert_eq!(*val, 5);
    m.insert("pear".to_string(), 4);
}

struct SharedCounter {
    count: RefCell<usize>,
}
impl SharedCounter {
    fn new() -> Self {
        SharedCounter {
            count: RefCell::new(0),
        }
    }
    fn increment(&self) {
        let mut g = self.count.borrow_mut();
        *g += 1;
    }
    fn get(&self) -> usize {
        *self.count.borrow()
    }
}

struct MutableMap {
    map: RefCell<HashMap<String, i32>>,
}
impl MutableMap {
    fn new() -> Self {
        MutableMap {
            map: RefCell::new(HashMap::new()),
        }
    }
    fn insert(&self, key: String, val: i32) {
        self.map.borrow_mut().insert(key, val);
    }
    fn get(&self, key: &str) -> Option<Ref<'_, i32>> {
        Ref::filter_map(self.map.borrow(), |map| map.get(key)).ok()
    }
}
