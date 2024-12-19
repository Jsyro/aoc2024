use std::collections::HashSet;

use crate::utils::point::Point;
#[derive(Default, Debug)]
struct Region {
    pub _plant_type: u8,
    pub area: usize,
    pub perimeter: usize,
}

impl Region {
    pub fn new(plant_type: u8, area: usize, perimeter: usize) -> Self {
        Self {
            _plant_type: plant_type,
            area,
            perimeter,
        }
    }
}

fn explore_region(
    char_map: &Vec<Vec<u8>>,
    visited: &mut HashSet<Point>,
    point: Point,
    plant: u8,
    region: &mut Region,
) -> bool {
    // println!("");
    // println!(
    //     "exploring {:?} region at {:?}, plant={}",
    //     region, point, plant
    // );
    assert_eq!(char_map[point.y as usize][point.x as usize], plant);
    if visited.contains(&point) {
        //do nothing, already processed
        // println!("already visited, return true");
        return true;
    }
    // println!("new plot, increase region size and mark visited");
    visited.insert(point);
    region.area += 1;

    for p in Point::orthogonal()
        .map(|o| o + point)
        .collect::<Vec<Point>>()
    {
        if let Some(row) = char_map.get(p.y as usize) {
            if let Some(char) = row.get(p.x as usize) {
                if *char == plant {
                    //neighbour is part of this region
                    // println!(
                    //     "neighbour, {:?} is same plant_type={}, continue exploring this region",
                    //     p, *char
                    // );

                    explore_region(char_map, visited, p, *char, region);
                } else {
                    region.perimeter += 1;
                    // println!(
                    //     "neighbour {:?} is different plant_type={}, increase perimeter and stop exploring",
                    //     p, *char
                    // );

                    //neighbour is part of new region
                    continue;
                }
            } else {
                // println!(
                //     "neighbour {:?} is out of bounds, increase perimeter and stop exploring",
                //     p
                // );
                region.perimeter += 1;
            }
        } else {
            // println!(
            //     "neighbour {:?} is out of bounds, increase perimeter and stop exploring",
            //     p
            // );
            region.perimeter += 1;
        }
    }
    false
}

pub fn run(lines: Vec<String>) {
    let mut result: usize = 0;
    let mut _p2_result: usize = 0;

    let char_map: Vec<Vec<u8>> = lines.iter().map(|line| line.as_bytes().to_vec()).collect();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut regions: Vec<Region> = Vec::new();

    for (y, row) in lines
        .iter()
        .map(|line| line.as_bytes().to_vec())
        .enumerate()
    {
        for (x, plant) in row.into_iter().enumerate() {
            let point = Point::new(y as i32, x as i32);
            // println!("next top level iteration at {:?}, {:?}", point, plant);

            if visited.contains(&point) {
                continue;
            }
            let mut region = Region::new(plant, 0, 0);
            explore_region(&char_map, &mut visited, point, plant, &mut region);
            regions.push(region);
        }
    }

    for region in regions {
        // println!("{:?}", region);
        result += region.area * region.perimeter;
    }

    println!("Day 12, Part 1. result= {}", result);
    println!("Day 12, Part 2. bad_result= {}", _p2_result);
    println!("");
}
