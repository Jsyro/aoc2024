pub mod day4 {

    pub fn run(lines: Vec<String>) {
        let mut result: i32 = 0;
        let mut p2_result: i32 = 0;

        let char_map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

        //establish all 8 radial directions starting clockwise
        let dn: [i16; 2] = [1, 0];
        let dn_right: [i16; 2] = [1, 1];
        let right: [i16; 2] = [0, 1];
        let up_right: [i16; 2] = [-1, 1];
        let up: [i16; 2] = [-1, 0];
        let up_left: [i16; 2] = [-1, -1];
        let left: [i16; 2] = [0, -1];
        let dn_left: [i16; 2] = [1, -1];

        let directions = [dn, dn_right, right, up_right, up, up_left, left, dn_left];

        for y in 0..lines.len() {
            for x in 0..lines[y].len() {
                // different approach, compose radial coordinates, then strings
                if char_map[y][x] == 'X' {
                    for &dir in directions.iter() {
                        let pos_4x = x as i16 + dir[1] * 3;
                        let pos_4y = y as i16 + dir[0] * 3;
                        if 0 > pos_4x || pos_4x >= lines[y].len() as i16 {
                            continue;
                        }

                        if 0 > pos_4y || pos_4y >= lines.len() as i16 {
                            continue;
                        }

                        let pos: [[usize; 2]; 4] = [
                            [y, x],
                            [
                                usize::try_from(y as i16 + dir[0] * 1).unwrap(),
                                usize::try_from(x as i16 + dir[1] * 1).unwrap(),
                            ],
                            [
                                usize::try_from(y as i16 + dir[0] * 2).unwrap(),
                                usize::try_from(x as i16 + dir[1] * 2).unwrap(),
                            ],
                            [
                                usize::try_from(y as i16 + dir[0] * 3).unwrap(),
                                usize::try_from(x as i16 + dir[1] * 3).unwrap(),
                            ],
                        ];
                        // println!("{:?}", pos);
                        //
                        let mut substring = String::from(char_map[pos[0][0]][pos[0][1]]);
                        substring.push(char_map[pos[1][0]][pos[1][1]]);
                        substring.push(char_map[pos[2][0]][pos[2][1]]);
                        substring.push(char_map[pos[3][0]][pos[3][1]]);
                        // println!("dir_vector={:?}{:?}", dir, substring);
                        if substring == "XMAS" {
                            // println!(
                            //     "Found {} at pos{:?} in direction {:?}",
                            //     substring,
                            //     [y, x],
                            //     dir
                            // );
                            result += 1;
                        }
                    }
                }
                //Part 2
                let x_directions = [dn_right, dn_left, up_left, up_right];

                if char_map[y][x] == 'A' {
                    if 1 > x || x >= lines[y].len() - 1 {
                        continue;
                    }

                    if 1 > y || y >= lines.len() - 1 {
                        continue;
                    }
                    let mut mas_count = 0;
                    for dir in x_directions {
                        let pos: [[usize; 2]; 3] = [
                            [
                                usize::try_from(y as i16 + dir[0] * -1).unwrap(),
                                usize::try_from(x as i16 + dir[1] * -1).unwrap(),
                            ],
                            [y, x],
                            [
                                usize::try_from(y as i16 + dir[0] * 1).unwrap(),
                                usize::try_from(x as i16 + dir[1] * 1).unwrap(),
                            ],
                        ];
                        let mut substring = String::from(char_map[pos[0][0]][pos[0][1]]);
                        substring.push(char_map[pos[1][0]][pos[1][1]]);
                        substring.push(char_map[pos[2][0]][pos[2][1]]);

                        if substring == "MAS" {
                            mas_count += 1;
                        }
                    }
                    if mas_count == 2 {
                        // println!("Found X-MAS");
                        p2_result += 1;
                    }
                }
            }
        }

        println!("Day 4, Part 1. result= {}", result);

        println!("Day 4, Part 2. result= {}", p2_result);

        println!("");
    }
}
