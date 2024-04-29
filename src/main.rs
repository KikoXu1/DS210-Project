use std::collections::{VecDeque, HashMap};
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

    // Compute distances using BFS and store in a HashMap
    let mut distances = HashMap::new();
    for i in 0..graph.n {
        distances.insert(i, compute_distances(i, &graph));
    }

    // Calculate statistics
    let mut all_distances = Vec::new();
    for i in 0..graph.n {
        let distances_from_i = &distances[&i];
        all_distances.extend_from_slice(&distances_from_i[..i]);
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

fn compute_distances(start: Vertex, graph: &Graph) -> u32 {
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

    let mut total_distance = 0;
    for &dist in &distance {
        total_distance += dist.unwrap_or(0);
    }

    print!("vertex:distance");
    for (v, &dist) in distance.iter().enumerate() {
        print!("   {}:{}", v, dist.unwrap_or(0));
    }
    println!();
    
    total_distance
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