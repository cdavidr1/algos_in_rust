pub struct Stack<T> {
    nodes: Vec<T>,
}

impl<T> Stack<T> {
    fn default() -> Self {
        Self { nodes: Vec::new() }
    }

    fn push(&mut self, elem: T) {
        self.nodes.push(elem);
    }

    fn pop(&mut self) -> Option<T> {
        self.nodes.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.nodes.last()
    }

    fn size(&self) -> usize {
        self.nodes.len()
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
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
        let ne = create_stack();
        assert_eq!(false, ne.is_empty());
    }
}
