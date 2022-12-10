use crate::utils::file;

pub fn run() {
    let input = file::read("day3");
    println!("{}", sum_priorities_part_one(input.as_str()));
}

fn sum_priorities_part_one(input: &str) -> i32 {
    return input.split("\n").map(|rucksack| compute_priority(rucksack)).sum();
}

fn compute_priority(rucksack: &str) -> i32 {
    let compartment_size = rucksack.len() / 2;
    let (first_compartment, second_compartment) = rucksack.split_at(compartment_size);
    let duplicate: char = first_compartment.chars().find(|first_char| second_compartment.chars().any(|second_char| first_char == &second_char)).unwrap();
    let origin = if duplicate.is_uppercase() {
        'A' as i32 - 26 - 1
    } else {
        'a' as i32 - 1
    };
    return duplicate as i32 - origin;
}

#[cfg(test)]
mod tests {
    use crate::day3::day3::sum_priorities_part_one;
    use crate::utils::file;

    #[test]
    fn should_count_score_part_one() {
        let result = sum_priorities_part_one(file::read_test("day3").as_str());
        assert_eq!(result, 157)
    }
}
