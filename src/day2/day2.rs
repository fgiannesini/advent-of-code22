use crate::utils::file;

pub fn run() {
    let input = file::read("day2");
    println!("{}", play(input.as_str()));
}

fn play(input: &str) -> i32 {
    return input.split("\n").map(|round| resolve(round)).sum();
}

fn resolve(round: &str) -> i32 {
    let propositions: Vec<&str> = round.split(" ").collect();
    let oponent = propositions[0];
    let me = propositions[1];
    let shape_score = match me {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    };
    let round_score = match (oponent, me) {
        ("A", "X") => 3,
        ("A", "Y") => 6,
        ("A", "Z") => 0,
        ("B", "X") => 0,
        ("B", "Y") => 3,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("C", "Y") => 0,
        ("C", "Z") => 3,
        _ => 0
    };
    return shape_score + round_score;
}

#[cfg(test)]
mod tests {
    use crate::day2::day2::play;
    use crate::utils::file;

    #[test]
    fn should_play() {
        let result = play(file::read_test("day2").as_str());
        assert_eq!(result, 15)
    }
}
