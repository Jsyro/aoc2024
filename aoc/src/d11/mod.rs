use std::time::Instant;

pub fn run(lines: Vec<String>) {
    let mut result: u64 = 0;
    let mut _p2_result: u64 = 0;

    let mut stones: Vec<u64> = lines[0]
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let mut new_stones: Vec<u64> = Vec::with_capacity(10);
    let start_time = Instant::now();
    for _generation in 0..40 {
        let current_time = Instant::now();
        println!(
            "total time to generation {:2}, {:10} stones - {:?}",
            _generation,
            stones.len(),
            current_time.duration_since(start_time)
        );
        new_stones.clear();
        for stone in stones.iter() {
            if *stone == 0 {
                new_stones.push(1);
            } else if stone.to_string().len() % 2 == 0 {
                let mut digits = Vec::new();
                let mut n = *stone;
                while n > 0 {
                    digits.push(n % 10);
                    n /= 10;
                }
                digits.reverse();
                let mid = digits.len() / 2;
                let s1 = digits[..mid].iter().fold(0, |acc, &d| acc * 10 + d);
                let s2 = digits[mid..].iter().fold(0, |acc, &d| acc * 10 + d);
                new_stones.push(s1);
                new_stones.push(s2);
            } else {
                let new_stone = *stone * 2024;
                new_stones.push(new_stone);
            }
        }
        if _generation == 24 {
            result = new_stones.len() as u64;
        }

        std::mem::swap(&mut stones, &mut new_stones);
    }
    _p2_result = stones.len() as u64;

    println!("Day 11, Part 1. result= {}", result);
    println!("Day 11, Part 2. result= {}", _p2_result);
    println!("");
}
