use std::env;
use std::fs::read_to_string;
mod d1;
mod d10;
mod d11;
mod d2;
mod d3;
mod d4;
mod d5;
mod d6;
mod d7;
mod d8;
mod d9;

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
    d1::run(read_lines("src/d1/input.txt"));
    d2::run(read_lines("src/d2/input.txt"));
    d3::run(read_lines("src/d3/input.txt"));
    d4::run(read_lines("src/d4/input.txt"));
    d5::run(read_lines("src/d5/input.txt"));
    d6::run(read_lines("src/d6/input.txt"));
    d7::run(read_lines("src/d7/input.txt"));
    d8::run(read_lines("src/d8/input.txt"));
    d9::run(read_lines("src/d9/input.txt"));
    d10::run(read_lines("src/d10/input.txt"));
    d11::run(read_lines("src/d11/input.txt"));
}
