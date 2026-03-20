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

struct Interner {
    pool: Vec<String>,
}
impl Interner {
    fn new() -> Self {
        Interner { pool: Vec::new() }
    }
    fn fall(&mut self, input: &str) -> Result<&str, &'static str> {
        if self.pool.len() > 100 {
            return Err("full");
        }

        if let Some(existing) = self.pool.iter().position(|s| s == &input) {
            return Ok(&self.pool[existing]);
        }

        self.pool.push(input.to_string());
        Ok(self.pool.last().unwrap())
    }
}
