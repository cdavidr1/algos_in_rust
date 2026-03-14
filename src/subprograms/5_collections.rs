use std::collections::{HashMap, HashSet};

fn main() {
    let mut l = UniqueList::new();
    l.add("s".to_string());
    l.add("s".to_string());
    println!("{:?}", l.contains("s"));
    println!("{:?}", l.len());

    // let s = build_set();
    // println!("{:?}", s.len());
}

// fn build_set<'a>() -> HashSet<&'a str> {
//     let local = String::from("ABC");
//     let slice = &local[..];

//     let mut s: HashSet<&str> = HashSet::new();
//     s.insert(slice);
//     s // ← trying to return it
// }

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

struct UniqueList {
    list: HashSet<String>,
}

impl UniqueList {
    fn new() -> Self {
        Self {
            list: HashSet::new(),
        }
    }
    fn add(&mut self, s: String) {
        self.list.insert(s);
    }
    fn contains(&self, s: &str) -> bool {
        self.list.contains(s)
    }
    fn len(&self) -> usize {
        self.list.len()
    }
}
