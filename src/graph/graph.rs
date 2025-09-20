use super::{Edge, Vertex};
struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    isWeighted: bool,
    isDirected: bool,
}
impl Graph {
    fn new(isWeighted: bool, isDirected: bool) -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            isWeighted,
            isDirected,
        }
    }
}
fn main() {}
