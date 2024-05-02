use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::Point;

pub fn read_points(filename: &str, sample_size: usize) -> Vec<Point> {
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