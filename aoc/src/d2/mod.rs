pub mod day2 {
    pub fn run(lines: Vec<String>) {
        let mut num_safe_reports: i32 = 0;
        let mut num_unsafe_reports: i32 = 0;
        let safe_jump = 3;

        let mut p2_num_safe_reports: i32 = 0;
        let mut p2_num_unsafe_reports: i32 = 0;
        let p2_safe_jump = 3;

        for line in &lines {
            let levels: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect();

            // Part 1
            let dif: Vec<i32> = levels.windows(2).map(|w| w[0] - w[1]).collect();

            if 0 - safe_jump <= *dif.iter().min().unwrap() && *dif.iter().max().unwrap() < 0 {
                num_safe_reports += 1;
                // println!("safe decreasing");
            } else if 0 < *dif.iter().min().unwrap() && *dif.iter().max().unwrap() <= safe_jump {
                num_safe_reports += 1;
                // println!("safe increasing");
            } else {
                num_unsafe_reports += 1;
                // println!("unsafe");
            }

            // Part 2 - Brute Force
            let mut possible_fixed_levels: Vec<Vec<i32>> = [].to_vec();

            // produce permitations
            for i in 0..levels.len() {
                let mut levels_p2 = levels.clone();
                levels_p2.remove(i);
                possible_fixed_levels.push(levels_p2);
            }

            // check jumps
            let mut report_safe_bool = false;
            for possible_fixed_level in possible_fixed_levels {
                let dif: Vec<i32> = possible_fixed_level
                    .windows(2)
                    .map(|w| w[0] - w[1])
                    .collect();
                if 0 - p2_safe_jump <= *dif.iter().min().unwrap() && *dif.iter().max().unwrap() < 0
                {
                    report_safe_bool = true;
                } else if 0 < *dif.iter().min().unwrap()
                    && *dif.iter().max().unwrap() <= p2_safe_jump
                {
                    report_safe_bool = true;
                }
            }

            //record result
            if report_safe_bool {
                p2_num_safe_reports += 1;
            } else {
                p2_num_unsafe_reports += 1;
            }
        }
        println!(
            "Day 2, Part 1. safe={}, unsafe={}",
            num_safe_reports, num_unsafe_reports
        );

        println!(
            "Day 2, Part 2. safe={}, unsafe={}",
            p2_num_safe_reports, p2_num_unsafe_reports
        );

        println!("");
    }
}
