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
    // [0, r] sum
    fn range_sum_from_zero(&self, mut r: usize) -> i32 {
        let mut sum = 0;
        loop {
            sum += self.bit[r];
            let r_next = r & (r + 1);
            if r_next == 0 {
                break;
            }
            r = r_next - 1;
        }
        sum
    }
    // [l, r]
    fn range_sum(&self, l: usize, r: usize) -> i32 {
        if l == 0 {
            self.range_sum_from_zero(r)
        } else {
            self.range_sum_from_zero(r) - self.range_sum_from_zero(l - 1)
        }
    }
}
fn main() {
    let input = vec![2, 3, 4, 5];

    let tree = FenwickTree::construct_and_build(&input);

    println!("Input: {:?}", input);
    println!("Fenwick: {:?}", tree.bit);
    println!("[0, 2] sum = {}", tree.range_sum_from_zero(2));
    println!("[1, 3] sum = {}", tree.range_sum(1, 3));
}
#[cfg(test)]
mod tests {
    use super::*;
}
