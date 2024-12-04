pub mod day3 {
    use regex::Regex;

    pub fn run(lines: Vec<String>) {
        let mut result: i32 = 0;
        let mut p2_result: i32 = 0;

        let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
        for line in &lines {
            // Part 1
            let result_iter = re.captures_iter(line);
            for (_, [v1, v2]) in result_iter.map(|pair| pair.extract()) {
                let i1: i32 = v1.parse().unwrap();
                let i2: i32 = v2.parse().unwrap();
                //println!("{}*{}", i1, i2);
                result += i1 * i2;
            }
            // Part 2 - Brute Force
        }
        println!("Day 3, Part 1. result= {}", result);

        for line in &lines {
            let mut segments: Vec<&str> = line.split("don't()").collect();
            //1st segment is valid
            //starts of subsiquent segments are invalid until another 'do()'
            let mut valid_segments: Vec<&str> = [segments.remove(0)].to_vec();

            for seg in segments {
                let mut all_sub_segments: Vec<&str> = seg.split("do()").collect();

                //first subsegments are ignored, everything after another do is valid
                let _ = all_sub_segments.remove(0);

                // println!("{:?}", all_sub_segments);

                valid_segments.append(&mut all_sub_segments);
            }
            let valid_string = valid_segments.join("");

            let result_iter = re.captures_iter(&valid_string);
            for (_, [v1, v2]) in result_iter.map(|pair| pair.extract()) {
                let i1: i32 = v1.parse().unwrap();
                let i2: i32 = v2.parse().unwrap();
                // println!("{}*{}", i1, i2);
                p2_result += i1 * i2;
            }
        }
        println!("Day 3, Part 2. result= {}", p2_result);

        println!("");
    }
}
