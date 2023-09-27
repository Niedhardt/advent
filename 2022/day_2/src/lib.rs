pub fn score_rps(input: &str, first: bool) -> String {
    //println!("input: {}", input);
    let result = input
        .split("\r\n")
        .map(|round| {
           // println!("{}", round);
            let fight: Vec<&str> = round.split_whitespace().collect();
            if first {
                return score_rps_match(fight);
            }
            return score_rps_match_result(fight);



        })
        .sum::<i32>();
    println!("{}", result);

    "result".to_string()

}


fn score_rps_match_result(input: Vec<&str>) -> i32 {
    match input[1] {
        //draw (A rock), (B Paper) and (C Scissor)
        // (Y
        "Y" => match input[0] {
            "A" => 4,
            "B" => 5,
            _ => 6,
        },

        //lose
        "X" => match input[0] {
            "A" => 3,
            "B" => 1,
            _ => 2,
        },
        //win
        _ => match input[0] {
            "A" => 8,
            "B" => 9,
            _ => 7,
        }

    }
}
fn score_rps_match(input: Vec<&str>) -> i32 {
    match input[0] {
        "A" => {
            match input[1] {
                "Y" => 8,
                "X" => 4,
                _ => 3
            }
        },
        "B" => {
            match input[1] {
                "Y" => 5,
                "X" => 1,
                _ => 9
            }
        },

        _ => {
            match input[1] {
                "Y" => 2,
                "X" => 7,
                _ => 6
            }
        },
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
