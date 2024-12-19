use std::collections::HashSet;

use crate::utils::point::Point;
#[derive(Default, Debug)]
struct Region {
    pub plant_type: u8,
    pub area: i32,
    pub perimeter: i32,
}

impl Region {
    pub fn new(plant_type: &u8, area: i32, perimeter: i32) -> Self {
        Self {
            plant_type: *plant_type,
            area,
            perimeter,
        }
    }
}

fn explore_region(
    char_map: &Vec<Vec<u8>>,
    visited: &mut HashSet<Point>,
    point: Point,
    plant: &u8,
    region: &mut Region,
) -> bool {
    println!("");
    println!(
        "exploring region {:?} at {:?}, plant={}",
        region, point, *plant as char
    );
    if visited.contains(&point) {
        //do nothing, already processed
        println!("already visited, return true");
        return true;
    }
    println!("new plot, increase region size and mark visited");
    visited.insert(point);
    region.area += 1;

    for p in Point::orthogonal()
        .map(|o| o + point)
        .collect::<Vec<Point>>()
    {
        if let Some(row) = char_map.get(p.y as usize) {
            if let Some(char) = row.get(p.x as usize) {
                if char == plant {
                    //neighbour is part of this region
                    println!(
                        "neighbour, {:?} is same plant_type={}, continue exploring this region",
                        p, *char as char
                    );

                    explore_region(char_map, visited, p, char, region);
                } else {
                    region.perimeter += 1;
                    println!(
                        "neighbour {:?} is different plant_type={}, increase perimeter and start exploring new region",
                        p, *char as char
                    );

                    //neighbour is part of new region
                    return false;
                    // explore_region(char_map, visited, point, plant, &mut Region::new());
                }
            }
        }
    }
    false
}

pub fn run(lines: Vec<String>) {
    let mut result: u64 = 0;
    let mut _p2_result: u64 = 0;

    let char_map: Vec<Vec<u8>> = lines.iter().map(|line| line.as_bytes().to_vec()).collect();
    let mut visited: HashSet<Point> = HashSet::new();
    // let plots: Vec<&(char, u32, u32)> = Vec::new();

    for (y, row) in lines
        .iter()
        .map(|line| line.as_bytes().to_vec())
        .enumerate()
    {
        for (x, _plant) in row.iter().enumerate() {
            let point = Point::new(x as i32, y as i32);

            if visited.contains(&point) {
                continue;
            }
            let mut region = Region::new(_plant, 0, 0);
            explore_region(&char_map, &mut visited, point, _plant, &mut region);
        }
    }

    println!("Day 12, Part 1. result= {}", result);
    println!("Day 12, Part 2. result= {}", _p2_result);
    println!("");
}
