use std::collections::HashSet;

use crate::toolbox::Point;

struct Region {
    area: u32,
    perimeter: u32,
}

fn explore_region(
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<Point>,
    region: &mut Region,
    plant: char,
    point: Point,
) -> bool {
    if let Some(row) = map.get(point.y as usize) {
        if let Some(char) = row.get(point.x as usize) {
            if *char == plant {
                if visited.contains(&point) {
                    return true;
                }
                visited.insert(point);
                region.area += 1;

                for neighbour in Point::orthogonal() {
                    println!("neighbour: [{},{}]", neighbour.x, neighbour.y);
                }
            }
        }
    }
    false
}

pub fn run(lines: Vec<String>) {
    let mut result: u64 = 0;
    let mut _p2_result: u64 = 0;

    let char_map: &Vec<Vec<char>> = &lines.iter().map(|l| l.chars().collect()).collect();
    let visited: HashSet<Point> = HashSet::new();
    let plots: Vec<&(char, u32, u32)> = Vec::new();

    for (y, row) in lines.iter().enumerate() {
        for (x, plant) in row.chars().enumerate() {
            let point = Point::new(x as i32, y as i32);
            for neighbour in point.orthogonal() {
                println!("neighbour: [{},{}]", neighbour.x, neighbour.y);
            }
        }
    }

    println!("Day 12, Part 1. result= {}", result);
    println!("Day 12, Part 2. result= {}", _p2_result);
    println!("");
}
