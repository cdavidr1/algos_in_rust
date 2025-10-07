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
