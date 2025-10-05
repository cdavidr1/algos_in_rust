pub struct Stack {
    nodes: Vec<Node>,
}

pub struct Node {
    value: i32,
    next: Box<Node>,
    prev: Box<Node>,
}

impl Node {
    fn new(value: i32, next: Node, prev: Node) -> Self {
        Self {
            value,
            next: Box::new(next),
            prev: Box::new(prev),
        }
    }
}

impl Stack {
    fn default() -> Self {
        Self { nodes: Vec::new() }
    }

    // Usually we append to the end of LL
    fn push(self, elem: Node) {
        self.push(elem);
    }

    fn pop(self) {
        if !self.isEmpty() {
            todo!();
        }
    }

    fn peek() {
        todo!();
    }

    fn size(self) -> i32 {
        self.size()
    }

    fn isEmpty(self) -> bool {
        self.size() == 0
    }
}
