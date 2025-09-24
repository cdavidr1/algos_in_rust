pub mod function {
    #[derive(Debug)]
    pub enum Function {
        Max,
        Min,
        Sum,
        GCD,
        LCM,
        Zeros,
    }

    pub fn is_power_of_two(n: usize) -> bool {
        let n = n + 1;
        n > 0 && (n & (n - 1)) == 0
    }

    pub fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }
}

pub mod segment_tree {

    use super::function::{gcd, is_power_of_two, Function};

    pub struct SegmentTree {
        nodes: usize,
        tree: Vec<(i32, i32)>,
        result: Function,
    }

    impl SegmentTree {
        pub fn construct_and_build(input: &[i32], result: Function) -> Self {
            let mut tree = Self::new(input.len(), result);
            tree.build(&input, 1, 0, tree.nodes - 1);
            tree
        }

        fn new(nodes: usize, result: Function) -> Self {
            Self {
                nodes,
                tree: vec![(0, 0); 4 * nodes],
                result,
            }
        }

        fn find_result(&self, left: (i32, i32), right: (i32, i32)) -> (i32, i32) {
            match self.result {
                Function::Max => {
                    if right.0 == left.0 {
                        (right.0, right.1 + left.1)
                    } else if right.0 > left.0 {
                        right
                    } else {
                        left
                    }
                }
                Function::Min => {
                    if right.0 == left.0 {
                        (right.0, right.1 + left.1)
                    } else if right.0 < left.0 {
                        right
                    } else {
                        left
                    }
                }
                Function::Sum | Function::Zeros => (left.0 + right.0, left.1 + right.1),
                Function::GCD => (gcd(left.0, right.0), left.1 + right.1),
                Function::LCM => ((left.0 * right.0) / gcd(left.0, right.0), left.1 + right.1),
            }
        }

        fn build(&mut self, input: &[i32], vertex: usize, left: usize, right: usize) {
            if left == right {
                match self.result {
                    Function::Zeros => self.tree[vertex].0 = if input[left] == 0 { 1 } else { 0 },
                    _ => self.tree[vertex].0 = input[left],
                }
                self.tree[vertex].1 = 1;
            } else {
                let mid = (left + right) / 2;
                self.build(input, vertex * 2, left, mid);
                self.build(input, vertex * 2 + 1, mid + 1, right);
                self.tree[vertex] =
                    self.find_result(self.tree[vertex * 2], self.tree[vertex * 2 + 1]);
            }
        }

        pub fn update(&mut self, index: usize, value: i32) {
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
                match self.result {
                    Function::Zeros => self.tree[vertex].0 = if value == 0 { 1 } else { 0 },
                    _ => self.tree[vertex].0 = value,
                }
                self.tree[vertex].1 = 1;
            } else {
                let mid = (left + right) / 2;
                if index <= mid {
                    self.update_helper(index, value, vertex * 2, left, mid);
                } else {
                    self.update_helper(index, value, vertex * 2 + 1, mid + 1, right);
                }
                self.tree[vertex] =
                    self.find_result(self.tree[vertex * 2], self.tree[vertex * 2 + 1]);
            }
        }

        pub fn get_segment(self, x: usize, y: usize) -> (i32, i32) {
            self.get_segment_helper(0, 0, 1, x, y)
        }

        fn get_segment_helper(
            self,
            node: usize,
            left: usize,
            right: usize,
            x: usize,
            y: usize,
        ) -> (i32, i32) {
            if x <= left && right >= y {
                self.tree[node]
            }
        }

        pub fn print_tree_structure(&self) {
            println!("Segment Tree ({:?})", self.result);
            for (i, (r, m)) in self.tree.iter().enumerate() {
                if *r == 0 {
                    continue;
                }
                if is_power_of_two(i) {
                    println!("  [{}]: {},{}", i, r, m);
                } else {
                    print!("  [{}]: {},{}", i, r, m);
                }
            }
            println!();
        }

        fn find_kth_zero(&self, v: usize, left: usize, right: usize, k: i32) -> Option<usize> {
            if k > self.tree[v].0 {
                return None;
            }
            if left == right {
                return Some(left);
            }
            let mid = (left + right) / 2;
            if self.tree[v * 2].0 >= k {
                return self.find_kth_zero(v * 2, left, mid, k);
            } else {
                return self.find_kth_zero(v * 2 + 1, mid + 1, right, k - self.tree[v * 2].0);
            }
        }
    }
}

use crate::segment_tree::segment_tree::function::Function;

use crate::segment_tree::segment_tree::segment_tree::SegmentTree;

fn main() {
    let example_input = vec![4, 2, 8];
    let mut seg_tree =
        segment_tree::SegmentTree::construct_and_build(&example_input, Function::Min);
    seg_tree.print_tree_structure();
    seg_tree.update(1, 20);
    seg_tree.print_tree_structure();
    let seg_tree = SegmentTree::construct_and_build(&example_input, Function::Sum);
    seg_tree.print_tree_structure();

    // Max / Min
    let seg_tree = SegmentTree::construct_and_build(&example_input, Function::Max);
    seg_tree.print_tree_structure();
    let max_dup_input = vec![9, 7, 4, 9, 2, 3, 9];
    let seg_tree = SegmentTree::construct_and_build(&max_dup_input, Function::Max);
    seg_tree.print_tree_structure();
    let min_dup_input = vec![1, 2, 3, 1, 4, 2, 1];
    let seg_tree = SegmentTree::construct_and_build(&min_dup_input, Function::Min);
    seg_tree.print_tree_structure();
    let seg_tree = SegmentTree::construct_and_build(&min_dup_input, Function::Sum);
    seg_tree.print_tree_structure();

    // GCD / LCM
    let seg_tree = SegmentTree::construct_and_build(&example_input, Function::GCD);
    seg_tree.print_tree_structure();

    let seg_tree = SegmentTree::construct_and_build(&example_input, Function::LCM);
    seg_tree.print_tree_structure();

    // Count Zeros
    let zeros_input = [0, 4, 2, 3, 1, 9, 0];
    let seg_tree = SegmentTree::construct_and_build(&zeros_input, Function::Zeros);
    seg_tree.print_tree_structure();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seg_tree_constructs() {
        let nodes = 3;
        let tree = SegmentTree::new(nodes, Function::Max);
        assert_eq!(tree.nodes, nodes);
        assert_eq!(tree.tree.len(), 4 * nodes);
    }
}
