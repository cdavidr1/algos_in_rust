pub struct Edge {
    pub start: usize,
    pub end: usize,
    pub weight: Option<i32>,
}
impl Edge {
    pub fn new(start: usize, end: usize, weight: Option<i32>) -> Self {
        Self { start, end, weight }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // <--- Add this!
    fn create_edge() {
        let e = Edge::new(1, 2, Some(3));
        // You might want to assert something here
        assert_eq!(e.start, 1);
        assert_eq!(e.end, 2);
        assert_eq!(e.weight, Some(3));
    }
}
