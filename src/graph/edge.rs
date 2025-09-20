use super::vertex::Vertex;
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
