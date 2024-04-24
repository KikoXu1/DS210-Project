use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct Point {
    x: i32,
    z: i32,
}

#[derive(Debug)]
struct Graph {
    n: usize,
    outedges: Vec<Vec<usize>>,
}

type Vertex = usize;
fn main() {
    let points: Vec<Point> = read_points("roadNet-CA.txt");
    let mut sorted_points = points.clone();
    sorted_points.sort_by_key(|p| p.x);

    // Print sorted points
    for point in &sorted_points {
        println!("x: {}, z: {}", point.x, point.z);
    }

// Populate graph data
let mut graph = Graph {
    n: sorted_points.len(),
    outedges: vec![Vec::new(); sorted_points.len()], // Initialize outedges with empty vectors
};

// Example: Populate outedges based on some adjacency information
for i in 0..graph.n {
    // Assuming each vertex has an edge to its immediate neighbors (for demonstration purposes)
    if i > 0 {
        graph.outedges[i].push(i - 1); // Add edge to the previous vertex
    }
    if i < graph.n - 1 {
        graph.outedges[i].push(i + 1); // Add edge to the next vertex
    }
}

// Compute distances using BFS
for i in 0..graph.n {
    println!("Distances from node {}", i);
    compute_and_print_distance_bfs(i, &graph);
}
}
fn read_points(filename: &str) -> Vec<Point> {
let file = File::open(filename).expect("Failed to open file");
let reader = BufReader::new(file);
let mut points = Vec::new();

for line in reader.lines() {
    if let Ok(line) = line {
        let mut parts = line.trim().split_whitespace();
        if let (Some(x_str), Some(z_str)) = (parts.next(), parts.next()) {
            if let (Ok(x), Ok(z)) = (x_str.parse::<i32>(), z_str.parse::<i32>()) {
                points.push(Point { x, z });
            }
        }
    }
}
points
}

fn compute_and_print_distance_bfs(start: Vertex, graph: &Graph) {
let mut distance: Vec<Option<u32>> = vec![None; graph.n];
distance[start] = Some(0); // <= we know this distance
let mut queue: VecDeque<Vertex> = VecDeque::new();
queue.push_back(start);
while let Some(v) = queue.pop_front() {
    // new unprocessed vertex
    for &u in &graph.outedges[v] {
        if distance[u].is_none() {
            // consider all unprocessed neighbors of v
            distance[u] = Some(distance[v].unwrap_or(0) + 1);
            queue.push_back(u);
        }
    }
}
print!("vertex:distance");
for (v, &dist) in distance.iter().enumerate() {
    print!("   {}:{}", v, dist.unwrap_or(0));
}
println!();
}

