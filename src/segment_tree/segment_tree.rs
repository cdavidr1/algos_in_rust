struct SegmentTree {
    nodes: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    fn construct_and_build(input: &[i32]) -> Self {
        let mut tree = Self::new(input.len());
        tree.build(&input, 1, 0, tree.nodes - 1);
        tree
    }

    fn new(nodes: usize) -> Self {
        Self {
            nodes,
            tree: vec![0; 4 * nodes],
        }
    }

    fn build(&mut self, input: &[i32], vertex: usize, left: usize, right: usize) {
        if left == right {
            self.tree[vertex] = input[left];
        } else {
            let mid = (left + right) / 2;
            self.build(input, vertex * 2, left, mid);
            self.build(input, vertex * 2 + 1, mid + 1, right);
            self.tree[vertex] = self.tree[vertex * 2] + self.tree[vertex * 2 + 1];
        }
    }

    fn update(&mut self, index: usize, value: i32) {
        self.update_helper(index, value, 1, 0, self.nodes - 1);
    }

    fn update_helper(
        &mut self,
        index: usize,
        value: i32,
        vertex: usize,
        left: usize,
        right: usize,
    ) {
        if index >= self.nodes {
            return;
        }
        if left == right {
            self.tree[vertex] = value;
        } else {
            let mid = (left + right) / 2;
            if index <= mid {
                self.update_helper(index, value, vertex * 2, left, mid);
            } else {
                self.update_helper(index, value, vertex * 2 + 1, mid + 1, right);
            }
            self.tree[vertex] = self.tree[vertex * 2] + self.tree[vertex * 2 + 1];
        }
    }

    fn print_tree_structure(&self) {
        println!("Segment Tree (Sum value)");
        for (i, val) in self.tree.iter().enumerate() {
            if *val == 0 {
                continue;
            }
            if Self::is_power_of_two(i) {
                println!("  [{}]: {}", i, val);
            } else {
                print!("  [{}]: {}", i, val);
            }
        }
        println!();
    }

    fn is_power_of_two(n: usize) -> bool {
        let n = n + 1;
        n > 0 && (n & (n - 1)) == 0
    }
}

fn main() {
    let example_input = vec![4, 2, 8];
    let mut seg_tree = SegmentTree::construct_and_build(&example_input);
    seg_tree.print_tree_structure();
    seg_tree.update(1, 20);
    seg_tree.print_tree_structure();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seg_tree_constructs() {
        let nodes = 3;
        let tree = SegmentTree::new(nodes);
        assert_eq!(tree.nodes, nodes);
        assert_eq!(tree.tree.len(), 4 * nodes);
    }
}
