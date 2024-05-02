use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub outedges: Vec<Vec<(usize, u32)>>,
}

pub type Vertex = usize;

pub fn compute_distance_bfs(start: Vertex, graph: &Graph) -> HashMap<Vertex, u32> {
    let mut distance: HashMap<Vertex, u32> = HashMap::new();
    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    distance.insert(start, 0);
    while let Some(v) = queue.pop_front() {
        // new unprocessed vertex
        for &(u, _) in &graph.outedges[*&v] {
            if !distance.contains_key(&u) {
                // consider all unprocessed neighbors of v
                let dist_v = *distance.get(&v).unwrap();
                let dist_u = dist_v + 1;
                distance.insert(u, dist_u);
                queue.push_back(u);
            }
        }
    }
    distance
}