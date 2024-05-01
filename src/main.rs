use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
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
    outedges: Vec<Vec<(usize, u32)>>,
}

type Vertex = usize;

fn main() {
    let points: Vec<Point> = read_points("roadNet-CA.txt", 30000);
    let mut sorted_points = points.clone();
    sorted_points.sort_by_key(|p| p.x);

    // Populate graph data
    let mut graph = Graph {
        n: sorted_points.len(),
        outedges: vec![Vec::new(); sorted_points.len()],
    };

    // Example: Populate outedges based on some adjacency information
    for i in 0..graph.n {
        // Assuming each vertex has an edge to its immediate neighbors (for demonstration purposes)
        if i > 0 {
            graph.outedges[i].push((i - 1, 1)); // Add edge to the previous vertex with distance 1
        }
        if i < graph.n - 1 {
            graph.outedges[i].push((i + 1, 1)); // Add edge to the next vertex with distance 1
        }
    }

    // Compute distances using BFS and store in a HashMap
    let mut distances = HashMap::new();
    for i in 0..graph.n {
        distances.insert(i, dijkstra(i, &graph));
    }

    // Calculate statistics
    let mut all_distances = Vec::new();
    for i in 0..graph.n {
        let distances_from_i = &distances[&i];
        all_distances.extend_from_slice(&distances_from_i.values().cloned().collect::<Vec<u32>>());
    }

    let total_distance: f64 = all_distances.iter().map(|&d| d as f64).sum();
    let num_distances = all_distances.len() as f64;
    let average_distance = total_distance / num_distances;
    let max_distance = *all_distances.iter().max().unwrap_or(&0);
    let median_distance = median(&mut all_distances);
    let variance = variance(&all_distances, average_distance);
    let standard_deviation = variance.sqrt();

    // Print statistics
    println!("Average distance: {}", average_distance);
    println!("Max distance: {}", max_distance);
    println!("Median distance: {}", median_distance);
    println!("Standard deviation: {}", standard_deviation);

    // Compute distances using Dijkstra's algorithm and store in a HashMap
    let distances_from_node_0 = dijkstra(0, &graph);
    for (node, distance) in distances_from_node_0 {
        println!("Shortest distance from node 0 to node {}: {}", node, distance);
    }
}

fn read_points(filename: &str, sample_size: usize) -> Vec<Point> {
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut points = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        if let Ok(line) = line {
            let mut parts = line.trim().split_whitespace();
            if let (Some(x_str), Some(z_str)) = (parts.next(), parts.next()) {
                if let (Ok(x), Ok(z)) = (x_str.parse::<i32>(), z_str.parse::<i32>()) {
                    points.push(Point { x, z });
                }
            }
        }
        if index + 1 >= sample_size {
            break;
        }
    }
    points
}

fn dijkstra(start: Vertex, graph: &Graph) -> HashMap<Vertex, u32> {
    let mut distance: HashMap<Vertex, u32> = HashMap::new();
    let mut heap = BinaryHeap::new();

    distance.insert(start, 0);
    heap.push(State { cost: 0, vertex: start });

    while let Some(State { cost, vertex }) = heap.pop() {
        if let Some(d) = distance.get(&vertex) {
            if cost > *d {
                continue;
            }
        }

        for &(neighbor, edge_cost) in &graph.outedges[vertex] {
            let new_cost = cost + edge_cost;

            if !distance.contains_key(&neighbor) || new_cost < *distance.get(&neighbor).unwrap_or(&u32::MAX) {
                distance.insert(neighbor, new_cost);
                heap.push(State { cost: new_cost, vertex: neighbor });
            }
        }
    }

    distance
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    cost: u32,
    vertex: Vertex,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn median(data: &mut [u32]) -> f64 {
    data.sort();
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        (data[mid - 1] + data[mid]) as f64 / 2.0
    } else {
        data[mid] as f64
    }
}

fn variance(data: &[u32], mean: f64) -> f64 {
    let mut sum_sq_diff = 0.0;
    for &x in data {
        sum_sq_diff += ((x as f64) - mean).powi(2);
    }
    sum_sq_diff / (data.len() as f64)
}
