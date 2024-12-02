use std::fmt::Debug;
use std::fs::read_to_string;

#[derive(Debug)]
#[derive(Clone)]
struct Report(Vec<i32>);

fn read_line(line: &str) -> Report {
    let parts: Vec<i32> = line.split(" ").map(|l|l.parse().unwrap()).collect();
    Report(parts)
}

fn allowed_deviation(a: i32, b: i32) -> bool {
    (1..4).contains(&(a-b))
}

fn is_safe(levels: Vec<i32>) -> bool {
    let result = levels.clone().into_iter().skip(1)
        .fold((*levels.get(0).unwrap(), true, true), |i, next|
            (next, i.1 && allowed_deviation(i.0, next), i.2 && allowed_deviation(next, i.0)));
    result.1 || result.2
}

fn is_safe_ignoring_bad(levels: Vec<i32>) -> bool {
    (0..levels.len()).any(|i| {
        let mut levels = levels.clone();
        levels.remove(i);
        is_safe(levels)
    })
}

fn main() {
    let reports: Vec<Report> = read_to_string( "src/values.txt").unwrap().lines().map(read_line).collect();

    println!("Part 1: {}", reports.clone().into_iter().filter(|r| is_safe(r.clone().0)).count());
    println!("Part 2: {}", reports.clone().into_iter().filter(|r| is_safe_ignoring_bad(r.clone().0)).count());
}
