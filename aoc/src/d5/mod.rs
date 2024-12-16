pub mod day5 {
    use std::collections::{HashMap, HashSet};

    pub fn run(lines: Vec<String>) {
        let mut result: i32 = 0;
        let mut p2_result: i32 = 0;

        // let empty_line_idx = lines.partition_point(|l| l == "");

        let rule_lines = lines[0..1176].to_vec();
        let update_lines = lines[1176 + 1..lines.len()].to_vec();
        // println!("{:?}", rule_lines);
        // println!("{:?}", update_lines);

        let ordering_rules_list: Vec<Vec<usize>> = rule_lines
            .iter()
            .map(|rl| rl.split("|").map(|x| x.parse::<usize>().unwrap()).collect())
            .collect();
        // println!("{:?}", ordering_rules_list);

        let mut ordering_rules_map: HashMap<usize, HashSet<usize>> = Default::default();

        for or in ordering_rules_list {
            ordering_rules_map
                .entry(or[0])
                .and_modify(|l| {
                    l.insert(or[1]);
                    return;
                })
                .or_insert(HashSet::from_iter(vec![or[1]].iter().cloned())); //TODO fix this
        }

        // println!("{:?}", ordering_rules_map);

        let updates: Vec<Vec<usize>> = update_lines
            .iter()
            .map(|ul| ul.split(",").map(|x| x.parse::<usize>().unwrap()).collect())
            .collect();
        // println!("{}", updates.len());

        for (_update_line_num, page_update) in updates.iter().enumerate() {
            let middle_value = page_update[page_update.len() / 2];

            let mut p1_report_good_ind = true;
            for (index, page_num) in page_update.iter().enumerate() {
                let previous_pages_set: HashSet<usize> =
                    page_update[0..index].iter().cloned().collect();

                // if and ordering rules (A must be after B) and instances of (A before B) intersect, report is bad
                let intersect: HashSet<_> = ordering_rules_map[page_num]
                    .intersection(&previous_pages_set)
                    .collect();

                let previous_pages: Vec<(usize, &usize)> = page_update[0..index]
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| intersect.contains(x))
                    .collect();
                // println!("previous pages enumerated = {:?}", previous_pages);
                if !intersect.is_empty() {
                    // println!(
                    //     "{} is a bad report, page_num {} at index {} has contray rules ={:?}",
                    //     _update_line_num, page_num, index, intersect
                    // );
                    p1_report_good_ind = false;

                    // PART TWO goes here, how to switch when

                    // println!(
                    //     "Report # {} Going to swap report {} in position {} with position {}",
                    //     _update_line_num, page_num, index, previous_pages[0].0
                    // );
                    let mut p2_update_copy: Vec<&usize> = page_update.iter().clone().collect();
                    p2_update_copy.swap(index, previous_pages[0].0);

                    p2_result += *p2_update_copy[p2_update_copy.len() / 2] as i32;
                    break;
                }
                break;
            }
            if p1_report_good_ind {
                result += middle_value as i32;
            }
        }

        println!("Day 5, Part 1. result= {}", result);

        println!("Day 5, Part 2. bad result= {}", p2_result);

        println!("");
    }
}
