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
    fn get_vertex_by_value(&self, value: String) -> Option<&Vertex> {
        self.vertices.iter().find(|v| v.get_data() == value)
    }
    fn get_vertex(&self, id: usize) -> Option<&Vertex> {
        self.vertices.iter().find(|v| v.id == id)
    }
    pub fn print(self, is_weighted: bool) {
        if self.edges.len() == 0 {
            println!("{} --> ", self.get_vertex(0).unwrap().get_data());
        } else {
            let mut message = String::new();
            for (i, e) in self.edges.iter().enumerate() {
                if i == 0 {
                    let first_data = self.get_vertex(e.start).unwrap().get_data();
                    message.push_str(first_data);
                    message.push_str(" --> ");
                }
                let next_data = self.get_vertex(e.end).unwrap().get_data();
                message.push_str(next_data);
                if is_weighted {
                    message.push_str(" (");
                    message.push_str(&e.weight.unwrap().to_string());
                    message.push_str(")");
                }
                message.push_str(", ");
            }
            println!("{}", message);
        }
    }
}
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] // <--- Add this!
    fn create_graph() {
        let mut g = Graph::new(true, true);
        g.add_vertex(String::from("1"));
        g.add_vertex(String::from("2"));
        g.add_edge(1, 2, Some(3));
        // You might want to assert something here
        assert_eq!(g.is_directed, true);
    }
}
