fn calc_checksum(data: Vec<u64>) -> u64 {
    let mut checksum: u64 = 0;
    for (position, value) in data.iter().enumerate() {
        // println!(
        //     "{}*{}={}",
        //     position as u64,
        //     *value as u64,
        //     position as u64 * *value as u64
        // );
        checksum += position as u64 * *value as u64;
    }
    return checksum;
}

fn uncompress_disk_map(disk_map: &String) -> Vec<i64> {
    // let _sample_disk_map = "2333133121414131402";

    let mut disk_data: Vec<i64> = vec![];

    let mut space: bool = false;
    let mut file_id: i64 = 0;
    for char in disk_map.chars() {
        let num = char.to_digit(10).unwrap() as u64;
        if space {
            disk_data.append(&mut [-1].repeat(num as usize));
        } else {
            disk_data.append(&mut [file_id].repeat(num as usize));
            file_id += 1;
        }
        space = !space;
    }

    return disk_data;
}

fn defrag(disk_data: &mut Vec<i64>) -> Vec<u64> {
    let mut defraged_data: Vec<u64> = vec![];

    for i in 0..disk_data.len() - 1 {
        if i >= disk_data.len() {
            break;
        }
        let val = disk_data[i];
        if val == -1 {
            //find last data block and move here
            let block_to_move = loop {
                let x = disk_data.pop().unwrap();
                if x != -1 {
                    break x;
                };
            };
            defraged_data.push(block_to_move as u64);
        } else {
            defraged_data.push(disk_data[i] as u64);
        }
    }

    return defraged_data;
}

pub fn run(lines: Vec<String>) {
    let mut _p2_result: u64 = 0;

    let disk_map = &lines[0];

    let mut disk_data = uncompress_disk_map(disk_map);
    // println!("full_disk_data= {:?}", disk_data);
    let defraged_disk = defrag(&mut disk_data);
    // println!("defraged disk= {:?}", defraged_disk);

    let result = calc_checksum(defraged_disk);
    println!("Day 9, Part 1. bad_result= {}", result);
    // 6349869590443 - high

    println!("Day 9, Part 2. bad_result= {}", _p2_result);

    println!("");
}
