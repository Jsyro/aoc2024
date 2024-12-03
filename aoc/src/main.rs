use std::fs::read_to_string;
mod d1;
mod d2;

fn main() {
    fn read_lines(filename: &str) -> Vec<String> {
        read_to_string(filename)
            .unwrap() // panic on possible file-reading errors
            .lines() // split the string into an iterator of string slices
            .map(String::from) // make each slice into a string
            .collect() // gather them together into a vector
    }

    println!("Hello, world!");
    d1::day1::run(read_lines("d1/input.txt"));
    d2::day2::run(read_lines("d2/input.txt"));
}
