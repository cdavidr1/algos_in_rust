use std::collections::HashMap;
use std::rc::Rc;

fn main() {
    let mut pool = StringPool::new();
    let a = pool.intern("hello");
    let b = pool.intern("world");
    let c = pool.intern("hello");
    println!("a points to c: {}", std::ptr::eq(&*a, &*c));
    println!("count hello {}", Rc::strong_count(&a));
    println!("count world {}", Rc::strong_count(&b));
}

struct StringPool {
    pool: HashMap<String, Rc<String>>,
}

impl StringPool {
    fn new() -> Self {
        StringPool {
            pool: HashMap::new(),
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
}
