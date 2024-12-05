use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
#[derive(Clone)]
struct Order(i32, i32);
type Pages = Vec<i32>;

fn read_order(line: &str) -> Order {
    let parts: Vec<&str> = line.split("|").collect();
    Order(parts.first().unwrap().parse::<i32>().unwrap(), parts.last().unwrap().parse::<i32>().unwrap())
}

fn read_pages(line: &str) -> Pages {
    line.split(",").map(|p|p.parse::<i32>().unwrap()).collect()
}

fn is_valid(pages: &Pages, orders: HashMap<i32, Vec<i32>>) -> bool {
    let mut before: Vec<i32> = vec![];
    pages.iter().all(|p| {
        if ! orders.get(p).unwrap().clone().iter().all(|o|!before.contains(&o)) {
            false
        } else {
            before.push(*p); true
        }
    })
}

fn calculate_pages(pages: &Pages) -> i32 {
    *pages.get(pages.len() / 2).unwrap()
}

fn order_pages_sort(pages: Pages, orders: HashMap<i32, Vec<i32>>) -> Pages {
    let mut ordered: Pages = pages.clone();
    ordered.sort_by(|a, b| if orders.get(a).unwrap().clone().contains(b) {
        Ordering::Less
    } else {
        Ordering::Greater
    });
    ordered
}

fn order_pages_loop(pages: Pages, orders: HashMap<i32, Vec<i32>>) -> Pages {
    let mut ordered: Pages = vec![];
    while ordered.len() != pages.len()  {
        pages.clone().into_iter().for_each(|p|{
            if !ordered.contains(&p) && orders.get(&p).unwrap().iter().all(|b| ordered.contains(&b) || !pages.contains(&b)) {
                ordered.push(p);
            }
        });
    }
    ordered
}

fn main() {
    let pages: Vec<Pages> = read_to_string( "src/pages.txt").unwrap().lines().map(read_pages).collect();
    let mut orders: HashMap<i32, Vec<i32>> = HashMap::new();
    read_to_string( "src/orders.txt").unwrap().lines().map(read_order).for_each(|o|orders.entry(o.0).or_insert(vec![]).push(o.1));

    let part1 = pages.clone().into_iter()
        .filter(|p|is_valid(&p, orders.clone()))
        .fold(0,|sum, p|sum + calculate_pages(&p));
    println!("Part 1: {}", part1); assert_eq!(4924, part1);

    let part2 = pages.clone().into_iter()
        .filter(|p|!is_valid(&p, orders.clone()))
        .map(|p|order_pages_sort(p.clone(), orders.clone()))
        .fold(0, |sum, p|sum + calculate_pages(&p));
    println!("Part 2 (by sort): {}", part2); assert_eq!(6085, part2);

    let part2 = pages.clone().into_iter()
        .filter(|p|!is_valid(&p, orders.clone()))
        .map(|p|order_pages_loop(p.clone(), orders.clone()))
        .fold(0, |sum, p|sum + calculate_pages(&p));
    println!("Part 2 (by loop): {}", part2); assert_eq!(6085, part2);
}
