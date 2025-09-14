struct FenwickTree {
    bit: Vec<i32>,
    nodes: usize,
}

impl FenwickTree {
    fn construct_and_build(input: &[i32]) -> Self {
        let mut tree = Self::new(input);
        for i in 0..tree.bit.len() {
            tree.add(i, input[i]);
        }
        tree
    }
    fn new(input: &[i32]) -> Self {
        Self {
            bit: vec![0; input.len()],
            nodes: input.len(),
        }
    }

    fn add(&mut self, mut idx: usize, delta: i32) {
        while idx < self.nodes {
            self.bit[idx] += delta;
            idx = idx | (idx + 1);
        }
    }
}

fn main() {
    let input = vec![2, 3, 4, 5];

    let tree = FenwickTree::construct_and_build(&input);

    println!("{:?}", tree.bit);
}

#[cfg(test)]
mod tests {
    use super::*;
}
