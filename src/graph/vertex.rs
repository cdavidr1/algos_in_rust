use super::edge::Edge;
pub struct Vertex {
    pub id: usize,
    data: String,
    edges: Vec<Edge>,
}
impl Vertex {
    pub fn new(id: usize, data: String) -> Self {
        Self {
            id,
            data,
            edges: Vec::new(),
        }
    }
    pub fn add_edge(&mut self, end: usize, weight: Option<i32>) {
        self.edges.push(Edge::new(self.id, end, weight));
    }
    pub fn remove_edge(&mut self, end: usize) {
        self.edges.retain(|edge| edge.end != end);
    }
}
