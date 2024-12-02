use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;

#[derive(Debug)]
#[derive(Clone)]
struct Values(i32, i32);

fn read_line(line: &str) -> Values {
    let parts: Vec<&str> = line.split(" ").collect();
    Values(parts.first().unwrap().parse::<i32>().unwrap(), parts.last().unwrap().parse::<i32>().unwrap())
}

fn get_values_sorted<F>(i: &Vec<Values>, f: F) -> Vec<i32> where F: FnMut(&Values) -> i32 {
    let mut values: Vec<i32> = i.iter().map(f).collect();
    values.sort();
    values
}

fn main() {
    let lines: Vec<Values> = read_to_string( "src/values.txt").unwrap().lines().map(read_line).collect();
    let first = get_values_sorted(&lines, |v: &Values| v.0);
    let second = get_values_sorted(&lines, |v: &Values| v.1);

    let part1= first.clone().into_iter()
        .zip(second.clone().into_iter())
        .fold(0, |sum, (a, b)| sum + (a - b).abs());
    println!("Part 1: {}", part1);

    let mut counts: HashMap<i32, i32> = HashMap::new();
    second.into_iter().for_each(|v| *counts.entry(v).or_insert(0) += 1);
    let part2= first.clone().into_iter()
        .fold(0, |sum, a| sum + a * counts.get(&a).unwrap_or(&0));
    println!("Part 2: {}", part2);
}
