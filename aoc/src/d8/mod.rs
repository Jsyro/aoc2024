pub mod day8 {
    use itertools::Itertools;
    use std::collections::{hash_map::Entry, HashMap};

    pub fn run(lines: Vec<String>) {
        let mut result: u64 = 0;
        let mut p2_result: u64 = 0;

        let mut antennas: HashMap<char, Vec<[usize; 2]>> = Default::default();
        let mut anitnodes: Vec<[i32; 2]> = Default::default();
        let mut char_map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

        for y in 0..lines.len() {
            for x in 0..lines[y].len() {
                let char_ = char_map[y][x];
                if !['#', '.'].contains(&char_) {
                    //antenna found push to hashmap
                    match antennas.entry(char_) {
                        Entry::Vacant(e) => {
                            e.insert(vec![[y, x]]);
                        }
                        Entry::Occupied(mut e) => {
                            e.get_mut().push([y, x]);
                        }
                    }
                }
            }
        }

        println!("Antennas Found, {:?}", antennas);

        for (signal, tower_pos_list) in antennas.iter().clone() {
            for t in tower_pos_list.into_iter().combinations(2) {
                let t1_pos: [usize; 2] = *t[0];
                let t2_pos: [usize; 2] = *t[1];

                let diff_vec: [i32; 2] = [
                    (t2_pos[0] as i32 - t1_pos[0] as i32),
                    (t2_pos[1] as i32 - t1_pos[1] as i32),
                ];

                let an1_pos: [i32; 2] = [
                    t2_pos[0] as i32 + diff_vec[0],
                    t2_pos[1] as i32 + diff_vec[1],
                ];

                let an2_pos: [i32; 2] = [
                    t1_pos[0] as i32 - diff_vec[0],
                    t1_pos[1] as i32 - diff_vec[1],
                ];

                for an_pos in [an1_pos, an2_pos] {
                    if an_pos[0] < 0 || an_pos[0] >= lines.len() as i32 {
                        //an1_y out of range
                        continue;
                    }
                    if an_pos[1] < 0 || an_pos[1] >= lines[0].len() as i32 {
                        //an1_x out of range
                        continue;
                    }

                    if !anitnodes.contains(&an_pos) {
                        println!("New Antinode at {:?} with freq {}", an_pos, signal);
                        anitnodes.push(an_pos);
                        result += 1;
                        char_map[an_pos[0] as usize][an_pos[1] as usize] = '#';
                    } else {
                        println!("Antinode position already found, {:?}", an_pos);
                    }
                }
            }
        }
        for row in char_map {
            println!("{:?}", row);
        }
        println!("Day 8, Part 1. result= {}", result);
        println!("Day 8, Part 2. bad_result= {}", p2_result);

        println!("");
    }
}
