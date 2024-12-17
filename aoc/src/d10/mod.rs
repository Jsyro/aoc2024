fn calculate_trailhead_values(map: &Vec<Vec<u8>>, pos: [usize; 2]) -> (u64, u64) {
    if map[pos[0]][pos[1]] != 0 {
        return (0, 0);
    } else {
        let mut p1_trailhead_value = valid_next_steps(map, pos, 0);
        let p2_trailhead_value = p1_trailhead_value.clone();
        p1_trailhead_value.sort();
        p1_trailhead_value.dedup();

        // println!(
        //     "trailhead at {:?} has p1_value= {}, p2_value= {}",
        //     pos,
        //     p1_trailhead_value.len(),
        //     p2_trailhead_value.len(),
        // );
        return (
            p1_trailhead_value.len() as u64,
            p2_trailhead_value.len() as u64,
        );
    }
}

fn valid_next_steps(map: &Vec<Vec<u8>>, pos: [usize; 2], val: u8) -> Vec<[usize; 2]> {
    // println!("at {:?}={}", pos, val);
    if val == 9 {
        return vec![pos];
    }

    let possible_steps: Vec<[i16; 2]> = vec![
        [pos[0] as i16 + 1, pos[1] as i16],
        [pos[0] as i16 - 1, pos[1] as i16],
        [pos[0] as i16, pos[1] as i16 + 1],
        [pos[0] as i16, pos[1] as i16 - 1],
    ]
    .iter()
    .filter(|p| p[0] >= 0 && p[0] < map.len() as i16 && p[1] >= 0 && p[1] < map[0].len() as i16)
    .map(|y| *y)
    .collect();

    let valid_steps: Vec<[usize; 2]> = possible_steps
        .into_iter()
        .filter(|[y, x]| map[*y as usize][*x as usize] == val + 1)
        .map(|[a, b]| [a as usize, b as usize])
        .collect();

    // println!("valid_steps={:?}", valid_steps);

    return valid_steps
        .iter()
        .map(|vs| valid_next_steps(map, *vs, val + 1))
        .flatten()
        .collect();
}

pub fn run(lines: Vec<String>) {
    let mut result: u64 = 0;
    let mut _p2_result: u64 = 0;

    let char_map: &Vec<Vec<u8>> = &lines
        .iter()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let (p1_val, p2_val) = calculate_trailhead_values(char_map, [y, x]);

            result += p1_val;
            _p2_result += p2_val;
        }
    }
    println!("Day 10, Part 1. result= {}", result);
    println!("Day 10, Part 2. result= {}", _p2_result);
    println!("");
}
