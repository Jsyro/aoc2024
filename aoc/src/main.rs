use std::env;
use std::fs::read_to_string;
mod d1;
mod d2;
mod d3;
mod d4;
mod d5;

fn main() {
    fn read_lines(filename: &str) -> Vec<String> {
        read_to_string(filename)
            .unwrap() // panic on possible file-reading errors
            .lines() // split the string into an iterator of string slices
            .map(String::from) // make each slice into a string
            .collect() // gather them together into a vector
    }

    fn current_dir() -> std::io::Result<()> {
        let path = env::current_dir()?;
        println!("The current directory is {}", path.display());
        Ok(())
    }

    let _ = current_dir();
    d1::day1::run(read_lines("src/d1/input.txt"));
    d2::day2::run(read_lines("src/d2/input.txt"));
    d3::day3::run(read_lines("src/d3/input.txt"));
    d4::day4::run(read_lines("src/d4/input.txt"));
    d5::day5::run(read_lines("src/d5/input.txt"));
}
