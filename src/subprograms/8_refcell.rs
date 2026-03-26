use std::cell::RefCell;

fn main() {
    let counter = SharedCounter::new();
    let c = &counter;
    println!("Init: {}", c.get());
    c.increment();
    c.increment();
    c.increment();
    println!("Cur: {}", c.get());
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
