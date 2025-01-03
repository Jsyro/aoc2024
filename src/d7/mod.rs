use itertools::Itertools;

pub fn run(lines: Vec<String>) {
    let mut result: u64 = 0;

    #[allow(unused_mut)]
    let mut p2_result: u64 = 0;

    for line in lines {
        let mut split_line = line.split(":");
        let test_value: u64 = split_line.next().unwrap().parse::<u64>().unwrap();

        let operands: Vec<u64> = split_line
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        //false is mulitply, true is add
        let mut answer_found = false;
        for num_multiply in 0..operands.len() {
            if answer_found {
                continue;
            }

            let multiplication_positions: Vec<Vec<usize>> = (0..(operands.len() - 1))
                .combinations(num_multiply)
                .collect();

            for pa_mul_pos in multiplication_positions {
                let mut curr_operators: Vec<bool> = [false].repeat(operands.len() - 1);
                for mp in pa_mul_pos {
                    curr_operators[mp] = true;
                }
                let mut eval_result = operands[0];

                for (ind, op) in curr_operators.iter().enumerate() {
                    if *op == false {
                        eval_result += operands[ind + 1];
                    } else {
                        eval_result *= operands[ind + 1];
                    }
                    if eval_result > test_value {
                        break;
                    }
                }

                if eval_result == test_value {
                    result += eval_result as u64;
                    // println!(
                    //     "found a permutation that matches.{:?},{:?}",
                    //     curr_operators, operands
                    // );
                    answer_found = true;
                    break;
                }
            }
        }
    }

    println!("Day 7, Part 1. result= {}", result);
    println!("Day 7, Part 2. bad_result= {}", p2_result);

    println!("");
}
