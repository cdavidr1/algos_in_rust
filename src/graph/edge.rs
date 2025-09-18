use super::vertex::Vertex;
pub struct Edge {
    start: Vertex,
    end: Vertex,
    weight: i32,
}
impl Edge {
    fn new(start: Vertex, end: Vertex, weight: i32) -> Self {
        Self { start, end, weight }
    }
}
