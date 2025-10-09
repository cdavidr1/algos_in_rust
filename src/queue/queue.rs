struct Queue<T> {
    nodes: Vec<T>,
}

// FIFO
impl<T> Queue<T> {
    fn default() -> Self {
        Self { nodes: Vec::new() }
    }
    // add to back of queue, adding, offering
    fn enqueue(&mut self, elem: T) {
        self.nodes.push(elem);
    }
    // remove element from front of queue, polling, 'removing'
    fn dequeue(&mut self) -> T {
        self.nodes.remove(0)
    }
    // view front
    fn peek(&self) -> &T {
        &self.nodes[0]
    }
    fn contains(&self, elem: &T) -> bool {
        self.nodes.contains(elem)
    }
    fn remove(&mut self, index: usize) -> T {
        self.nodes.remove(index)
    }
    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_q() -> Queue<i32> {
        let mut q = Queue::default();
        q.enqueue();
        q
    }

    #[test]
    fn test_create_q() {
        let mut c = create_q();
        assert_eq!(c.nodes.len(), 0);
    }
}
