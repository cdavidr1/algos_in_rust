#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    nodes: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
        }
    }

    pub fn push(&mut self, elem: T) {
        self.nodes.push(elem);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.nodes.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.nodes.last()
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_stack() -> Stack<i32> {
        let mut s = Stack::default();
        s.push(2);
        s.push(4);
        s.push(1);
        s
    }

    fn create_stack_w_capacity() -> Stack<i32> {
        let mut s = Stack::with_capacity(10);
        s.push(3);
        s
    }

    #[test]
    fn test_pop() {
        let mut pop = create_stack();
        assert_eq!(1, pop.pop().unwrap());
        assert_eq!(4, pop.pop().unwrap());
    }

    #[test]
    fn test_peek() {
        let peek = create_stack();
        assert_eq!(1, *peek.peek().unwrap());
    }

    #[test]
    fn test_size() {
        let size = create_stack();
        assert_eq!(3, size.size());
    }

    #[test]
    fn test_not_empty() {
        let ne = create_stack_w_capacity();
        assert_eq!(false, ne.is_empty());
    }
}
