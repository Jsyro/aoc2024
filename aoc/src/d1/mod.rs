pub mod day1 {
    pub fn run(lines: Vec<String>) {
        let mut left = Vec::new();
        let mut right = Vec::new();

        for line in &lines {
            let vals: Vec<&str> = line.split_whitespace().collect();
            left.push(vals[0].parse::<u32>().unwrap());
            right.push(vals[1].parse::<u32>().unwrap());
        }

        left.sort();
        right.sort();

        //Part 1
        let mut total_diff: u32 = 0;
        for i in 0..left.len() {
            total_diff += left[i].abs_diff(right[i]);
        }
        println!("Day1: Part1= {}", total_diff);

        //Part 2
        let mut total_similarity_score: u32 = 0;
        for i in 0..left.len() {
            total_similarity_score +=
                left[i] * right.iter().filter(|&n| *n == left[i]).count() as u32;
        }
        println!("Day1: Part2= {}", total_similarity_score);
    }
}
