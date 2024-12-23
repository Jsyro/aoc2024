use crate::utils::point::Point;
use std::collections::HashSet;

use std::cmp::{max, min};
#[derive(Default, Debug, PartialEq)]
struct Button {
    y: usize,
    x: usize,
    cost: isize,
}
impl Button {
    // pub fn new(y: usize, x: usize, cost: usize) -> Self {
    //     Self { y, x, cost }
    // }

    fn distance(&self) -> usize {
        return self.x + self.y;
    }

    fn weighted_distance(&self) -> f32 {
        return (self.x + self.y) as f32 / self.cost as f32;
    }

    fn press(&self, loc: &mut Point, total_cost: &mut isize, num_presses: isize) {
        *total_cost += self.cost * num_presses;
        *loc += Point {
            x: self.x as isize * num_presses,
            y: self.y as isize * num_presses,
        };
        // println!(
        //     "pressing button {} times to move: {:?}",
        //     num_presses,
        //     [self.y as isize * num_presses, self.x as isize * num_presses]
        // );
    }

    // fn unpress(&self, loc: &mut Point, total_cost: &mut usize, num_presses: isize) {
    //     *total_cost -= self.cost * num_presses as usize;
    //     *loc -= Point {
    //         x: self.x as isize * num_presses,
    //         y: self.y as isize * num_presses,
    //     };
    // }
}

fn find_optimal_path(a: &Button, b: &Button, prize_loc: Point) -> isize {
    let mut total_cost: isize = 0;
    let mut curr_loc = Point { x: 0, y: 0 };

    let ordered_buttons = if a.weighted_distance() > b.weighted_distance() {
        [a, b]
    } else {
        [b, a]
    };

    let steps_y = prize_loc.y as usize / ordered_buttons[0].y;
    let steps_x = prize_loc.x as usize / ordered_buttons[0].x;
    let num_better_presses = min(steps_x, steps_y);
    ordered_buttons[0].press(&mut curr_loc, &mut total_cost, num_better_presses as isize);

    let mut offset = prize_loc.difference(&curr_loc);

    let mut num_presses: isize = 0;
    let mut offset_sum: isize = 0;
    // println!(
    //     "{:6} presses to intitial interect offset: {:?}",
    //     num_better_presses, offset
    // );

    // two cases, better button is X bias or Y bias
    // X-bias:

    // step 0: x_offset is small, y_offset is positive.
    // step 1: x_offset is negative, y_offset is small.
    // step 2: x_offset is small, y_offset is positive.

    // Y-bias:

    // step 0: x_offset is large, y_offset is small
    // step 1: x_offset is small, y_offset is large negative.
    let mut buttons = [ordered_buttons[0], ordered_buttons[1]];
    let mut button = ordered_buttons[0];
    let mut y_bias: bool = offset[0] < offset[1];
    let mut visited: HashSet<Point> = HashSet::new();

    // println!("y_bias: {:?}", y_bias);
    while offset.iter().any(|o| *o != 0) {
        //rotate
        visited.insert(curr_loc);

        buttons.rotate_left(1);
        button = buttons[0];
        let b_arr = [button.y, button.x];
        let fnum_presses: f32 = offset[y_bias as usize] as f32 / b_arr[y_bias as usize] as f32;
        num_presses = offset[y_bias as usize] / b_arr[y_bias as usize] as isize;

        // should never have 0 presses. if fractional, round up
        if num_presses as f32 != fnum_presses {
            if fnum_presses < 0.0 {
                num_presses -= 1;
            } else if fnum_presses > 0.0 && fnum_presses < 1.0 {
                num_presses = 1;
            }
        }

        button.press(&mut curr_loc, &mut total_cost, num_presses);
        offset = prize_loc.difference(&curr_loc);
        // println!(
        //     "{:6} of {:?} presses to new offset: {:?}",
        //     num_presses, button, offset
        // );
        if offset.iter().all(|o| *o == 0) {
            break;
        }

        offset_sum = offset[0].abs() + offset[1].abs();
        if offset_sum > 30000000000000 || visited.contains(&curr_loc) {
            // if we can't reach the prize, or are at a location we have arleady been too we're done
            return 0; // impossible
        } else {
            visited.insert(curr_loc);
        }
        y_bias = !y_bias
    }

    // println!("total_cost: {:?}", total_cost);

    total_cost
}

fn parse_button(line: &str, cost: isize) -> Button {
    let y_str = line.split(" ").collect::<Vec<&str>>()[3]
        .split("+")
        .collect::<Vec<&str>>()[1];
    let x_str = line.split(" ").collect::<Vec<&str>>()[2]
        .split("+")
        .collect::<Vec<&str>>()[1]
        .split(",")
        .collect::<Vec<&str>>()[0];
    return Button {
        y: y_str.parse::<usize>().unwrap(),
        x: x_str.parse::<usize>().unwrap(),
        cost: cost,
    };
}

pub fn run(lines: Vec<String>) {
    let mut result: usize = 0;
    let mut _p2_result: usize = 0;

    let mut a: Button = Default::default();
    let mut b: Button = Default::default();

    for line in lines {
        if line.starts_with("Button A") {
            a = parse_button(&line, 3);
            // println!("Button A Configured: {:?}", a);
        } else if line.starts_with("Button B") {
            b = parse_button(&line, 1);
            // println!("Button B Configured: {:?}", b);
        } else if line.starts_with("Prize") {
            let y_str = line.split(" ").collect::<Vec<&str>>()[2]
                .split("=")
                .collect::<Vec<&str>>()[1];
            let x_str = line.split(" ").collect::<Vec<&str>>()[1]
                .split("=")
                .collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()[0];
            let prize_loc = Point {
                y: y_str.parse::<isize>().unwrap(),
                x: x_str.parse::<isize>().unwrap(),
            };
            let p2_prize_loc = Point {
                y: 10000000000000 + y_str.parse::<isize>().unwrap(),
                x: 10000000000000 + x_str.parse::<isize>().unwrap(),
            };

            // println!("P1 Prize Location: {:?}", prize_loc);
            // then execute
            result += find_optimal_path(&a, &b, prize_loc) as usize;
            // println!("P2 Prize Location: {:?}", p2_prize_loc);
            _p2_result += find_optimal_path(&a, &b, p2_prize_loc) as usize;

            a = Default::default();
            b = Default::default();
        }
    }

    println!("Day 13, Part 1. bad_result= {}", result);
    println!("Day 13, Part 2. bad_result= {}", _p2_result);
    println!("");
}
