use day_2::score_rps;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();

    println!("{}", score_rps(&file, false));
}