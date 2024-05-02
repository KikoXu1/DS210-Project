pub fn median(data: &mut [u32]) -> f64 {
    data.sort();
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        (data[mid - 1] + data[mid]) as f64 / 2.0
    } else {
        data[mid] as f64
    }
}

pub fn variance(data: &[u32], mean: f64) -> f64 {
    let mut sum_sq_diff = 0.0;
    for &x in data {
        sum_sq_diff += ((x as f64) - mean).powi(2);
    }
    sum_sq_diff / (data.len() as f64)
}