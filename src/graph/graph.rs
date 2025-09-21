use crate::graph::edge::Edge;
use crate::graph::vertex::Vertex;

struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
    is_weighted: bool,
    is_directed: bool,
}
impl Graph {
    fn new(is_weighted: bool, is_directed: bool) -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            is_weighted,
            is_directed,
        }
    }
    fn add_vertex(&mut self, data: String) {
        let next_id = if !self.vertices.is_empty() {
            self.vertices.last().unwrap().id + 1
        } else {
            1
        };
        let new_vertex: Vertex = Vertex::new(next_id, data);
        self.vertices.push(new_vertex);
    }
    fn add_edge(&mut self, from_id: usize, to_id: usize, _weight: Option<i32>) {
        let weight = if self.is_weighted { _weight } else { None };

        if let Some(from_vertex) = self.vertices.iter_mut().find(|v| v.id == from_id) {
            from_vertex.add_edge(to_id, weight);
            self.edges.push(Edge::new(from_id, to_id, weight));

            if !self.is_directed {
                if let Some(to_vertex) = self.vertices.iter_mut().find(|v| v.id == to_id) {
                    to_vertex.add_edge(from_id, weight);
                    self.edges.push(Edge::new(to_id, from_id, weight));
                }
            }
        }
    }
    fn remove_edge(&mut self, from_id: usize, to_id: usize) {
        if let Some(from_vertex) = self.vertices.iter_mut().find(|v| v.id == from_id) {
            from_vertex.remove_edge(to_id);
        }

        self.edges
            .retain(|e| !(e.start == from_id && e.end == to_id));

        if !self.is_directed {
            if let Some(to_vertex) = self.vertices.iter_mut().find(|v| v.id == to_id) {
                to_vertex.remove_edge(from_id);
            }

            self.edges
                .retain(|e| !(e.start == to_id && e.end == from_id));
        }
    }
    fn remove_vertex(&mut self, vertex_id: usize) {
        self.edges
            .retain(|e| e.start != vertex_id && e.end != vertex_id);

        for v in &mut self.vertices {
            v.remove_edge(vertex_id);
        }

        self.vertices.retain(|v| v.id != vertex_id);
    }
}
fn main() {}
