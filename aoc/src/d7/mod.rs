pub mod day7 {
    use core::num;
    use itertools::{all, Itertools, MultiUnzip, Tuples};
    use std::default;

    pub fn run(lines: Vec<String>) {
        let mut result: i32 = 0;
        let mut p2_result: i32 = 0;

        for line in lines {
            let mut split_line = line.split(":");
            let test_value: u32 = split_line.next().unwrap().parse::<u32>().unwrap();

            let operands: Vec<u32> = split_line
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect();

            println!("{},{:?}", test_value, operands);

            //first approach, add all, then randomly replace with multiply.

            //false is mulitply, true is add
            for num_multiply in 1..operands.len() - 1 {
                // let mut curr_operators: Vec<bool> =
                //     [true].repeat(operands.len() - 1 - num_multiply);
                // curr_operators.append(&mut [false].repeat(num_multiply));

                let mut multiplication_positions: Vec<Vec<usize>> = (0..(operands.len() - 1))
                    .combinations(num_multiply)
                    .collect();
                println!("{:?}", multiplication_positions[0]);
                println!("{:?}", multiplication_positions[1]);
                println!("{:?}", multiplication_positions[2]);
                println!("{:?}", multiplication_positions[3]);
                println!("{:?}", multiplication_positions[4]);

                for mul_pos in multiplication_positions {
                    let mut curr_operators: Vec<bool> = Default::default();
                    for mp in mul_pos {
                        curr_operators[mp] = true;
                    }
                    println!("operators list={:?}", curr_operators);
             
                   break;

                // println!("OUTER LOOP={:?}", curr_operators);

                let mut all_permuts: Vec<_> = curr_operators
                    .iter()
                    .permutations(curr_operators.len())
                    .collect();

                all_permuts.sort();

                all_permuts.dedup_by(|x, y| {
                    let mut sip = x.iter().zip(y.iter());
                    return sip.all(|(xx, yy)| **xx == **yy);
                });

                let mut permuts_iter = all_permuts.iter();

                println!("1st PERMUTATIONs={:?}", permuts_iter.next());
                println!("2nd PERMUTATIONs={:?}", permuts_iter.next());
                println!("3rd PERMUTATIONs={:?}", permuts_iter.next());
                println!("4th PERMUTATIONs={:?}", permuts_iter.next());
                println!("5th PERMUTATIONs={:?}", permuts_iter.next());

                for curr_attempt in all_permuts.iter() {
                    // println!("ordered operands ={:?}", operands);
                    // println!("ordered operators={:?}", curr_attempt);

                    let mut eval_result = operands[0];
                    for (ind, op) in curr_attempt.iter().enumerate() {
                        if **op == true {
                            eval_result += operands[ind + 1];
                        } else {
                            eval_result *= operands[ind + 1];
                        }
                        if eval_result > test_value {
                            println!("greater than test_value, stop this permutation");
                            break;
                        }
                    }

                    println!("eval_result={:?}", eval_result);
                    if eval_result == test_value {
                        println!("TEST_VALUE CAN BE PRODUCED FROM OPERANDS");
                        result += eval_result as i32;
                        break;
                    }
                }
            }

            break;
        }

        println!("Day 7, Part 1. bad_result= {}", result);
        println!("Day 7, Part 2. bad_result= {}", p2_result);

        println!("");
    }
}
