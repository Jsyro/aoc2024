pub mod day6 {

    pub fn run(lines: Vec<String>) {
        let mut result: i32 = 0;
        let mut p2_result: i32 = 0;

        let mut char_map: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

        let up: [i16; 2] = [-1, 0];
        let right: [i16; 2] = [0, 1];
        let dn: [i16; 2] = [1, 0];
        let left: [i16; 2] = [0, -1];

        let mut directions = [up, right, dn, left];

        let mut curr_pos: [usize; 2] = [0, 0];

        for (y_pos, line) in lines.iter().enumerate() {
            let start_x_pos = line.find("^").unwrap_or(0);
            if start_x_pos != 0 {
                curr_pos = [y_pos, start_x_pos];
                break;
            }
        }
        // println!("found starting position at{:?}", curr_pos);

        loop {
            let next_y_pos = curr_pos[0] as i16 + directions[0][0];
            let next_x_pos = curr_pos[1] as i16 + directions[0][1];

            if (next_y_pos < 0 || (lines.len() as i16) <= next_y_pos)
                || (next_x_pos < 0 || (lines[0].len() as i16) <= next_x_pos)
            {
                // println!("Route Complete, last coordinate {:?}", curr_pos);
                break;
            }

            let char_ahead = char_map[next_y_pos as usize][next_x_pos as usize];
            // println!(
            //     "looking at {} at {:?}",
            //     char_ahead,
            //     [next_y_pos, next_x_pos]
            // );

            if char_ahead == '#' {
                //obstacle, turn right
                // println!("turning right");
                directions.rotate_left(1);
            } else if ['X', '.'].contains(&char_ahead) {
                if char_map[curr_pos[0]][curr_pos[1]] != 'X' {
                    result += 1;
                    //leave an x behind
                    char_map[curr_pos[0]][curr_pos[1]] = 'X';
                }
                //step forward
                curr_pos = [next_y_pos as usize, next_x_pos as usize];
                // println!("stepping forward");
            } else {
                // println!("UNEXPECTED CHARACTER: {}", char_ahead)
            }
        }

        // let mut x_count = 0;
        // for char_arr in char_map {
        //     x_count += char_arr.iter().filter(|c| **c == 'X').count();
        //     println!("{:?}", char_arr);
        // }

        //final x is not being counted, just add 1
        println!("Day 6, Part 1. bad_result= {}", result + 1);
        p2_result += -1;
        println!("Day 6, Part 2. bad_result= {}", p2_result);

        println!("");
    }
}
