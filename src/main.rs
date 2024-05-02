mod graph;
mod statistics;
mod file_io;

use file_io::read_points;
use graph::compute_distance_bfs;
use statistics::{median, variance};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Point {
    pub x: i32,
    pub z: i32,
}

fn main() {
    let points: Vec<Point> = read_points("roadNet-CA.txt", 30000);
    let mut sorted_points = points.clone();
    sorted_points.sort_by_key(|p| p.x);

    // Populate graph data
    let mut graph = graph::Graph {
        n: sorted_points.len(),
        outedges: vec![Vec::new(); sorted_points.len()],
    };

    // Populate outedges based on some adjacency information
    for i in 0..graph.n {
        if i > 0 {
            graph.outedges[i].push((i - 1, 1));
        }
        if i < graph.n - 1 {
            graph.outedges[i].push((i + 1, 1));
        }
    }

    // Compute distances using BFS and store in a HashMap
    let mut distances = HashMap::new();
    for i in 0..graph.n {
        distances.insert(i, compute_distance_bfs(i, &graph));
    }

    // Calculate statistics
    let mut all_distances = Vec::new();
    for i in 0..graph.n {
        let distances_from_i = &distances[&i];
        all_distances.extend_from_slice(&distances_from_i.values().cloned().collect::<Vec<u32>>());
    }

    // Compute average shortest path lengths for each node
    let mut average_shortest_paths = Vec::new();
    for i in 0..graph.n {
        let distances_from_i = &distances[&i];
        let total_distance_from_i: u32 = distances_from_i.values().sum();
        let average_distance_from_i = total_distance_from_i as f64 / graph.n as f64;
        average_shortest_paths.push(average_distance_from_i);
    }

    //Computing the statistics
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

    println!("Distribution of average shortest path lengths for each node:");
    for (node, avg_distance) in average_shortest_paths.iter().enumerate() {
        println!("Node {}: {}", node, avg_distance);
    }

    // Compute distances using Dijkstra's algorithm and store in a HashMap
    let distances_from_node_0 = compute_distance_bfs(0, &graph);
    for (node, distance) in distances_from_node_0 {
        println!("Shortest distance from node 0 to node {}: {}", node, distance);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_distance_bfs() {
        // Create a sample graph
        let graph = graph::Graph {
            n: 5,
            outedges: vec![
                vec![(1, 1)],
                vec![(0, 1), (2, 1)],
                vec![(1, 1), (3, 1)],
                vec![(2, 1), (4, 1)],
                vec![(3, 1)],
            ],
        };

        // Compute distances from vertex 0
        let distances_from_0 = compute_distance_bfs(0, &graph);

        // Check if the distances are correct
        assert_eq!(distances_from_0[&0], 0);
        assert_eq!(distances_from_0[&1], 1);
        assert_eq!(distances_from_0[&2], 2);
        assert_eq!(distances_from_0[&3], 3);
        assert_eq!(distances_from_0[&4], 4);
    }

    #[test]
    fn test_median() {
        let mut data = vec![5, 2, 3, 1, 4];
        assert_eq!(median(&mut data), 3.0);

        let mut data = vec![5, 2, 3, 1, 4, 6];
        assert_eq!(median(&mut data), 3.5);
    }

    #[test]
    fn test_variance() {
        let data = vec![2, 4, 4, 4, 5, 5, 7, 9];
        assert_eq!(variance(&data, 5.0), 4.0);

        let data = vec![600, 470, 170, 430, 300];
        assert_eq!(variance(&data, 394.0), 21704.0);
    }
}