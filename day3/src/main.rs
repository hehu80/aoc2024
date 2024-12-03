use std::fs::read_to_string;
use regex::Regex;

fn mul_all(memory: &String) -> i32 {
    let reg_ex_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    reg_ex_mul.captures_iter(memory.trim()).map(|c| {
        let (_full, [first, second]) = c.extract();
        first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap()
    }).sum::<i32>()
}

fn mul_conditional(memory: &String) -> i32 {
    let reg_ex_memory = Regex::new(r"do\(\)|don't\(\)|mul\(\d+,\d+\)").unwrap();
    let reg_ex_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut enabled = true;
    reg_ex_memory.captures_iter(memory.trim()).map(|c|c.get(0).unwrap().as_str()).map(|m| {
        if m.starts_with("do") {
            enabled = m == "do()";
        } else if enabled {
            let (_full, [first, second]) = reg_ex_mul.captures(m).unwrap().extract();
            return first.parse::<i32>().unwrap() * second.parse::<i32>().unwrap();
        }
        0
    }).sum::<i32>()
}

fn main() {
    let memory = read_to_string( "src/memory.txt").unwrap();

    println!("Part 1: {}", mul_all(&memory));
    println!("Part 1: {}", mul_conditional(&memory));
}
