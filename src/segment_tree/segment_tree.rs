#[derive(Debug)]
enum Result {
    Max,
    Min,
    Sum,
}

struct SegmentTree {
    nodes: usize,
    tree: Vec<(i32, i32)>,
    result: Result,
}

impl SegmentTree {
    fn construct_and_build(input: &[i32], result: Result) -> Self {
        let mut tree = Self::new(input.len(), result);
        tree.build(&input, 1, 0, tree.nodes - 1);
        tree
    }

    fn new(nodes: usize, result: Result) -> Self {
        Self {
            nodes,
            tree: vec![(0, 0); 4 * nodes],
            result,
        }
    }

    fn find_result(&self, left: (i32, i32), right: (i32, i32)) -> (i32, i32) {
        match self.result {
            Result::Max => {
                if right.0 == left.0 {
                    (right.0, right.1 + left.1)
                } else if right.0 > left.0 {
                    right
                } else {
                    left
                }
            }
            Result::Min => {
                if right.0 == left.0 {
                    (right.0, right.1 + left.1)
                } else if right.0 < left.0 {
                    right
                } else {
                    left
                }
            }
            Result::Sum => (left.0 + right.0, left.1 + right.1),
        }
    }

    fn build(&mut self, input: &[i32], vertex: usize, left: usize, right: usize) {
        if left == right {
            self.tree[vertex].0 = input[left];
            self.tree[vertex].1 = 1;
        } else {
            let mid = (left + right) / 2;
            self.build(input, vertex * 2, left, mid);
            self.build(input, vertex * 2 + 1, mid + 1, right);
            self.tree[vertex] = self.find_result(self.tree[vertex * 2], self.tree[vertex * 2 + 1]);
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
            self.tree[vertex].0 = value;
            self.tree[vertex].1 = 1;
        } else {
            let mid = (left + right) / 2;
            if index <= mid {
                self.update_helper(index, value, vertex * 2, left, mid);
            } else {
                self.update_helper(index, value, vertex * 2 + 1, mid + 1, right);
            }
            self.tree[vertex] = self.find_result(self.tree[vertex * 2], self.tree[vertex * 2 + 1]);
        }
    }

    fn print_tree_structure(&self) {
        println!("Segment Tree ({:?})", self.result);
        for (i, (r, m)) in self.tree.iter().enumerate() {
            if *r == 0 {
                continue;
            }
            if Self::is_power_of_two(i) {
                println!("  [{}]: {},{}", i, r, m);
            } else {
                print!("  [{}]: {},{}", i, r, m);
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
    let mut seg_tree = SegmentTree::construct_and_build(&example_input, Result::Min);
    seg_tree.print_tree_structure();
    seg_tree.update(1, 20);
    seg_tree.print_tree_structure();
    let seg_tree = SegmentTree::construct_and_build(&example_input, Result::Sum);
    seg_tree.print_tree_structure();

    // Max
    let seg_tree = SegmentTree::construct_and_build(&example_input, Result::Max);
    seg_tree.print_tree_structure();
    let max_dup_input = vec![9, 7, 4, 9, 2, 3, 9];
    let seg_tree = SegmentTree::construct_and_build(&max_dup_input, Result::Max);
    seg_tree.print_tree_structure();
    let min_dup_input = vec![1, 2, 3, 1, 4, 2, 1];
    let seg_tree = SegmentTree::construct_and_build(&min_dup_input, Result::Min);
    seg_tree.print_tree_structure();
    let seg_tree = SegmentTree::construct_and_build(&min_dup_input, Result::Sum);
    seg_tree.print_tree_structure();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seg_tree_constructs() {
        let nodes = 3;
        let tree = SegmentTree::new(nodes, Result::Max);
        assert_eq!(tree.nodes, nodes);
        assert_eq!(tree.tree.len(), 4 * nodes);
    }
}
