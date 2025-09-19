use super::edge::Edge;
pub struct Vertex {
    data: String,
    edges: Vec<Edge>,
}
impl Vertex {
    fn new(input: String) -> Self {
        Self {
            data: input,
            edges: Vec::new(),
        }
    }
    pub fn add_edge(&mut self, end: Vertex, weight: Option<i32>) {
        self.edges.push(Edge::new(*self, end, weight));
    }
}
