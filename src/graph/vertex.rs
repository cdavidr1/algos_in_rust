use super::edge::Edge;
pub struct Vertex {
    data: String,
    edges: Vec<Edge>,
}
impl Vertex {
    fn new(input: String) -> Self {
        Self {
            data: input,
            edges: vec![0; 4],
        }
    }
    pub fn addEdge(end: Vertex, weight: i32) {}
}
