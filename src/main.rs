use std::io::{self, BufRead};

fn main() {
    println!("Please input the number list, separated by space:");

    let numbers: Vec<f64> = io::stdin()
    .lock()
    .lines()
    .next()
    .unwrap()
    .unwrap()
    .split_whitespace()
    .map(|s| s.parse().unwrap())
    .collect();

    let sum: f64 = numbers.iter().sum();
    let count = numbers.len() as f64;
    let average = sum / count;

    println!("average: {}", average);
}