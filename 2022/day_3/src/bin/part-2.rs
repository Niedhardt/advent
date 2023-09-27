use day_3::rucksack_organization_p2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./input.txt").unwrap();

    println!("{}", rucksack_organization_p2(&file));
}