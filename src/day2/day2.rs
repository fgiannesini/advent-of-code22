use crate::utils::file;

pub fn run() {
    let input = file::read("day2");
    println!("{}", count_score_part_one(input.as_str()));
    println!("{}", count_score_part_two(input.as_str()));
}

fn count_score_part_one(input: &str) -> i32 {
    return input.split("\n").map(|round| resolve_part_one(round)).sum();
}

fn resolve_part_one(round: &str) -> i32 {
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

fn count_score_part_two(input: &str) -> i32 {
    return input.split("\n").map(|round| resolve_part_two(round)).sum();
}

fn resolve_part_two(round: &str) -> i32 {
    let propositions: Vec<&str> = round.split(" ").collect();
    let oponent = propositions[0];
    let me = propositions[1];
    let shape_score = match me {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0
    };
    let round_score = match (oponent, me) {
        ("A", "X") => 3,
        ("A", "Y") => 1,
        ("A", "Z") => 2,
        ("B", "X") => 1,
        ("B", "Y") => 2,
        ("B", "Z") => 3,
        ("C", "X") => 2,
        ("C", "Y") => 3,
        ("C", "Z") => 1,
        _ => 0
    };
    return shape_score + round_score;
}

#[cfg(test)]
mod tests {
    use crate::day2::day2::{count_score_part_one, count_score_part_two};
    use crate::utils::file;

    #[test]
    fn should_count_score_part_one() {
        let result = count_score_part_one(file::read_test("day2").as_str());
        assert_eq!(result, 15)
    }

    #[test]
    fn should_count_score_part_two() {
        let result = count_score_part_two(file::read_test("day2").as_str());
        assert_eq!(result, 12)
    }
}
