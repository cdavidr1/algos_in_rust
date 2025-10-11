use std::collections::VecDeque;

struct Queue<T> {
    nodes: VecDeque<T>,
}

// FIFO
impl<T> Queue<T> {
    fn default() -> Self {
        Self {
            nodes: VecDeque::new(),
        }
    }
    // add to back of queue, adding, offering
    fn enqueue(&mut self, elem: T) {
        self.nodes.push_back(elem);
    }
    // remove element from front of queue, polling, 'removing'
    fn dequeue(&mut self) -> Option<T> {
        self.nodes.pop_front()
    }
    // view front
    fn peek(&self) -> &T {
        &self.nodes[0]
    }
    fn contains(&self, elem: T) -> bool
    where
        T: PartialEq,
    {
        self.nodes.contains(&elem)
    }
    fn remove(&mut self, index: usize) -> Option<T> {
        self.nodes.remove(index)
    }
    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    fn size(&self) -> usize {
        self.nodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_q() -> Queue<i32> {
        let mut q = Queue::default();
        q
    }

    #[test]
    fn test_create_q() {
        let mut c = create_q();
        assert_eq!(c.size(), 0);
    }

    #[test]
    fn test_is_empty() {
        let mut c = create_q();
        assert_eq!(c.is_empty(), true);
    }

    #[test]
    fn test_peek() {
        let mut c = create_q();
        c.enqueue(7);
        assert_eq!(*c.peek(), 7);
    }

    #[test]
    fn test_dequeue() {
        let mut c = create_q();
        assert_eq!(c.dequeue(), None)
    }

    #[test]
    fn test_remove() {
        let mut c = create_q();
        c.enqueue(3);
        c.enqueue(6);
        c.enqueue(4);
        c.enqueue(1);
        assert_eq!(c.contains(6), true);
        assert_eq!(c.size(), 4);
        assert_eq!(c.remove(1), Some(6));
    }
}
