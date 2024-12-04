pub mod day4 {
    use std::ops::Index;

    pub fn run(lines: Vec<String>) {
        let mut result: i32 = 0;
        let mut p2_result: i32 = 0;

        let char_map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

        //establish all 8 radial directions starting clockwise
        let up: [i8; 2] = [1, 0];
        let up_right: [i8; 2] = [1, 1];
        let right: [i8; 2] = [0, 1];
        let dn_right: [i8; 2] = [-1, 1];
        let dn: [i8; 2] = [-1, 0];
        let dn_left: [i8; 2] = [-1, -1];
        let left: [i8; 2] = [0, -1];
        let up_left: [i8; 2] = [1, -1];

        let directions = [up, up_right, right, dn_right, dn, dn_left, left, up_left];

        for y in 0..1 {
            for x in 0..lines[y].len() {
                if char_map[y][x] == 'X' {
                    //horizontal
                    if x < lines[y].len() - 3
                        && char_map[y][x..x + 4] == "XMAS".chars().collect::<Vec<char>>()
                    {
                        println!("Found XMAS at {},{} going right", y, x);
                        result += 1;
                    }
                    if x > 3 && char_map[y][(x - 3)..x + 1] == "SAMX".chars().collect::<Vec<char>>()
                    {
                        println!("Found XMAS at {},{} going left", y, x);
                        result += 1;
                    }

                    // different approach
                    let mut substrings: [[char; 4]; 8];
                    if char_map[y][x] == 'X' {
                        for (x, &dir) in directions.iter().enumerate() {
                            substrings[x] = [
                                char_map[y][x],
                                char_map[y + dir[0] * 1][x + dir[1] * 1],
                                char_map[y + dir[0] * 2][x + dir[1] * 2],
                                char_map[y + dir[0] * 3][x + dir[1] * 3],
                            ]
                        }
                        println!("{:?}", substrings);
                    }
                }
            }
        }

        println!("Day 4, Part 1. result= {}", result);

        println!("Day 4, Part 2. result= {}", p2_result);

        println!("");
    }
}
