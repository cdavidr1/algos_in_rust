pub struct FenwickTree {
    bit: Vec<i32>,
    nodes: usize,
}
impl FenwickTree {
    pub fn construct_and_build(input: &[i32]) -> Self {
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
    pub fn add(&mut self, mut idx: usize, delta: i32) {
        while idx < self.nodes {
            self.bit[idx] += delta;
            idx = idx | (idx + 1);
        }
    }
    // [0, r] sum
    pub fn range_sum_from_zero(&self, mut r: usize) -> i32 {
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
    pub fn range_sum(&self, l: usize, r: usize) -> i32 {
        if l == 0 {
            self.range_sum_from_zero(r)
        } else {
            self.range_sum_from_zero(r) - self.range_sum_from_zero(l - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_ft() -> FenwickTree {
        let input = vec![2, 3, 4, 5];

        FenwickTree::construct_and_build(&input)
    }

    #[test]
    fn test_create() {
        let ft = create_ft();
        assert_ne!(ft.nodes, 0);
    }
}
