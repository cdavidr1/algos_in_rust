use super::vertex::Vertex;
pub struct Edge {
    start: Vertex,
    end: Vertex,
    weight: Option<i32>,
}
impl Edge {
    pub fn new(start: Vertex, end: Vertex, weight: Option<i32>) -> Self {
        Self { start, end, weight }
    }
}
