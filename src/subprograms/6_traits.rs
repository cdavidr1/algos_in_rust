use std::collections::HashSet;
use std::hash::Hash;
fn main() {
    let v: Vec<usize> = Vec::from([0, 1, 2, 3, 2, 4, 5, 1, 2]);
    println!("{:?}", dedup(v));
}

fn dedup<T: Eq + Hash + Clone>(items: Vec<T>) -> Vec<T> {
    let mut set = HashSet::new();
    for item in items {
        set.insert(item);
    }
    set.iter().cloned().collect()
}
