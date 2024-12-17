pub mod day9 {

    pub fn run(lines: Vec<String>) {
        let mut result: i32 = 0;
        let mut p2_result: i32 = 0;

        let char_map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

        println!("Day 4, Part 1. bad_result= {}", result);
        println!("Day 4, Part 2. bad_result= {}", p2_result);

        println!("");
    }
}
