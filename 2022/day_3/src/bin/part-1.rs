use day_3::rucksack_organization;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();

    println!("{}", rucksack_organization(&file));
}