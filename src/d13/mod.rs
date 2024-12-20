use crate::utils::point::Point;

#[derive(Default, Debug)]
struct Button {
    y: usize,
    x: usize,
    cost: usize,
}
impl Button {
    pub fn new(y: usize, x: usize, cost: usize) -> Self {
        Self { y, x, cost }
    }

    fn weighted_distance(&self) -> f32 {
        return (self.x + self.y) as f32 / self.cost as f32;
    }
    fn direction(&self) -> f32 {
        return (self.y / self.x) as f32;
    }
    fn press(&self, loc: &mut Point, total_cost: &mut usize) {
        *total_cost += self.cost;
        *loc += Point {
            x: self.x as isize,
            y: self.y as isize,
        };
    }

    fn unpress(&self, loc: &mut Point, total_cost: &mut usize) {
        *total_cost -= self.cost;
        *loc -= Point {
            x: self.x as isize,
            y: self.y as isize,
        };
    }
}

fn find_optimal_path(a: &Button, b: &Button, prize_loc: Point) -> usize {
    let mut total_cost: usize = 0;
    let mut curr_loc: Point = Point { x: 0, y: 0 };

    let (better_button, lesser_button) = if a.weighted_distance() < b.weighted_distance() {
        (a, b)
    } else {
        (b, a)
    };
    // println!(
    //     "The better button for total distance is: {:?}",
    //     *better_button
    // );

    let mut max_num_presses_before_oob: usize = 0;
    loop {
        let num_y = prize_loc.y as usize / better_button.y;
        let num_x = prize_loc.x as usize / better_button.x;

        let num_presses = min(num_y, num_x);

        curr_loc = Point {
            x: (num_presses * better_button.x) as isize,
            y: (num_presses * better_button.y) as isize,
        };
        total_cost += num_presses * better_button.cost;

        better_button.press(&mut curr_loc, &mut total_cost);
    }

    loop {
        // println!("current_location: {:?}", curr_loc);
        if curr_loc.x == 0 && curr_loc.y == 0 {
            return 0; // no possible solution.
        }
        better_button.unpress(&mut curr_loc, &mut total_cost);
        let missing_y: usize = prize_loc.y as usize - curr_loc.y as usize;
        let missing_x: usize = prize_loc.x as usize - curr_loc.x as usize;

        if missing_y % lesser_button.y != 0 || missing_x % lesser_button.x != 0 {
            //unpress again
            continue;
        }

        if missing_y / lesser_button.y != missing_x / lesser_button.x {
            // both divisible, but not equal, continue unpressing
            continue;
        }

        while curr_loc.x != prize_loc.x || curr_loc.y != prize_loc.y {
            //spam lesser button ()
            lesser_button.press(&mut curr_loc, &mut total_cost);
        }
        break;
    }

    println!("total_cost: {:?}", total_cost);
    //TODO
    total_cost
}

fn parse_button(line: &str, cost: usize) -> Button {
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
            let mut prize_loc = Point {
                y: y_str.parse::<isize>().unwrap(),
                x: x_str.parse::<isize>().unwrap(),
            };
            let mut p2_prize_loc = Point {
                y: 10000000000000 + y_str.parse::<isize>().unwrap(),
                x: 10000000000000 + x_str.parse::<isize>().unwrap(),
            };

            println!("P1 Prize Location: {:?}", prize_loc);
            // then execute
            result += find_optimal_path(&a, &b, prize_loc);
            println!("P2 Prize Location: {:?}", p2_prize_loc);
            _p2_result += find_optimal_path(&a, &b, p2_prize_loc);

            a = Default::default();
            b = Default::default();
        }
    }

    println!("Day 13, Part 1. bad_result= {}", result);
    println!("Day 13, Part 2. bad_result= {}", _p2_result);
    println!("");
}
