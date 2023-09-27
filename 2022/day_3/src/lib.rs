use std::collections::HashMap;
use std::collections::HashSet;
pub fn rucksack_organization(input: &str) -> String {

    let result = input
        .split("\r\n")
        .map(|sack| {

            let mut hash: HashMap<&u8, u8> = HashMap::new();
            let (first, last) = sack.split_at(sack.len() / 2);
            println!("first: {}, last: {}", first, last);
            for i in first.as_bytes() {
                hash.insert(i, check_char(&i));
            }
            let mut value = 0;
            for i in last.as_bytes(){
                if hash.contains_key(i){
                    if let Some(x) = hash.get(i){
                        value = *x as i32;
                    }
                }
            }


            return value;

        }).sum::<i32>();

    result.to_string()

}

pub fn rucksack_organization_p2(input: &str) -> String {
    let lines = input.lines();
    let mut index = 0;
    let mut sum = 0;
    let mut hash: HashMap<&u8, u8> = HashMap::new();
    for line in lines {
        if index == 0 {
            hash.clear();
        }
        let mut set = HashSet::new();
        for i in line.as_bytes() {
            if !set.contains(i) {
                set.insert(i);
            }
        }
        for key in set {
            hash.entry(key).and_modify(|counter| *counter += 1).or_insert(1);
        }
        index += 1;
        if index == 3 {
            index = 0;

            for (key, _val) in &hash {
                if hash.get(key) == Some(&3){
                    sum += check_char(key) as i32;
                }
            }
        }

    }
    sum.to_string()
}

fn check_char(input: &u8) -> u8 {
    match input {
        65..= 90 => input - 38,
        _ => input - 96,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let result = rucksack_organization(input);
        println!("{}", result);
        assert_eq!(result, "157");
    }
}
