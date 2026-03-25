use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

fn main() {
    let mut pool = StringPool::new();
    let a = pool.intern("hello");
    let b = pool.intern("world");
    let c = pool.intern("hello");
    println!("a points to c: {}", std::ptr::eq(&*a, &*c));
    println!("count hello {}", Rc::strong_count(&a));
    println!("count world {}", Rc::strong_count(&b));

    let d = pool.internrc("blah");
    let e = pool.internrc("blah");
    println!(
        "Unique strings in poolrc: {}",
        pool.poolRefCell.borrow().len()
    );
}

struct StringPool {
    pool: HashMap<String, Rc<String>>,
    poolRefCell: RefCell<HashSet<String>>,
}

impl StringPool {
    fn new() -> Self {
        StringPool {
            pool: HashMap::new(),
            poolRefCell: RefCell::new(HashSet::new()),
        }
    }

    fn intern(&mut self, s: &str) -> Rc<String> {
        if let Some(existing) = self.pool.get(s) {
            return Rc::clone(existing);
        }

        let rc = Rc::new(s.to_string());
        self.pool.insert(s.to_string(), Rc::clone(&rc));
        rc
    }

    fn internrc(&self, s: &str) -> String {
        // immutable borrow for existence
        {
            let pool = self.poolRefCell.borrow();
            if let Some(existing) = pool.get(s) {
                return existing.clone();
            }
        }

        let mut pool = self.poolRefCell.borrow_mut();
        let owned = s.to_string();
        pool.insert(owned.clone());
        owned
    }
}
