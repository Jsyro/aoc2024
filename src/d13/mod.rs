fn find_optimal_path(a_press: [usize; 2], b_press: [usize; 2], prize_loc: [usize; 2]) -> usize {
    let mut result: usize = 0;

    return result;
}

pub fn run(lines: Vec<String>) {
    let mut result: usize = 0;
    let mut _p2_result: usize = 0;

    let mut a_press: [usize; 2] = Default::default();
    let mut b_press: [usize; 2] = Default::default();
    let mut prize_loc: [usize; 2] = Default::default();

    for line in lines {
        if line.starts_with("Button A") {
            let y_str = line.split(" ").collect::<Vec<&str>>()[3]
                .split("+")
                .collect::<Vec<&str>>()[1];
            let x_str = line.split(" ").collect::<Vec<&str>>()[2]
                .split("+")
                .collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()[0];

            a_press[0] = y_str.parse::<usize>().unwrap();
            a_press[1] = x_str.parse::<usize>().unwrap();
            println!("Button A Configured: {:?}", a_press);
        } else if line.starts_with("Button B") {
            let y_str = line.split(" ").collect::<Vec<&str>>()[3]
                .split("+")
                .collect::<Vec<&str>>()[1];
            let x_str = line.split(" ").collect::<Vec<&str>>()[2]
                .split("+")
                .collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()[0];

            b_press[0] = y_str.parse::<usize>().unwrap();
            b_press[1] = x_str.parse::<usize>().unwrap();
            println!("Button B Configured: {:?}", b_press);
        } else if line.starts_with("Prize") {
            let y_str = line.split(" ").collect::<Vec<&str>>()[2]
                .split("=")
                .collect::<Vec<&str>>()[1];
            let x_str = line.split(" ").collect::<Vec<&str>>()[1]
                .split("=")
                .collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()[0];

            prize_loc[0] = y_str.parse::<usize>().unwrap();
            prize_loc[1] = x_str.parse::<usize>().unwrap();
            println!("Prize Location: {:?}", prize_loc);
        } else {
            println!("Empty line");

            let combination = find_optimal_path(a_press, b_press, prize_loc);

            a_press = Default::default();
            b_press = Default::default();
            prize_loc = Default::default();
        }
    }

    println!("Day 13, Part 1. bad_result= {}", result);
    println!("Day 13, Part 2. bad_result= {}", _p2_result);
    println!("");
}
