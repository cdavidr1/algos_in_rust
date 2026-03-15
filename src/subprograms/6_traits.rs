use std::collections::HashSet;
use std::hash::Hash;
fn main() {
    let ve: Vec<usize> = Vec::from([0, 1, 2, 3, 2, 4, 5, 1, 2]);
    println!("{:?}", dedup(ve));
}

fn dedup<T: Eq + Hash + Clone>(items: Vec<T>) -> Vec<T> {
    let mut set = HashSet::new();
    for item in items {
        set.insert(item);
    }
    set.iter().cloned().collect()
}

trait Internable {
    type Key: Eq + Hash;
    fn key(&self) -> Self::Key;
}

impl Internable for String {
    type Key = String;
    fn key(&self) -> String {
        self.clone()
    }
}
impl Internable for &str {
    type Key = String;
    fn key(&self) -> String {
        (*self).to_string()
    }
}
