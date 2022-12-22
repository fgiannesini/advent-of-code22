use crate::utils::file;

#[allow(dead_code)]
pub fn run() {
    let input = file::read("day3");
    println!("{}", sum_priorities_part_one(input.as_str()));
    println!("{}", sum_priorities_part_two(input.as_str()));
}

fn sum_priorities_part_one(input: &str) -> i32 {
    return input.split("\n").map(|rucksack| compute_priority(rucksack)).sum();
}

fn compute_priority(rucksack: &str) -> i32 {
    let compartment_size = rucksack.len() / 2;
    let (first_compartment, second_compartment) = rucksack.split_at(compartment_size);
    let duplicate: char = first_compartment.chars().find(|first_char| second_compartment.chars().any(|second_char| first_char == &second_char)).unwrap();
    return to_number(duplicate);
}

fn to_number(duplicate: char) -> i32 {
    let origin = if duplicate.is_uppercase() {
        'A' as i32 - 26 - 1
    } else {
        'a' as i32 - 1
    };
    return duplicate as i32 - origin;
}

fn sum_priorities_part_two(input: &str) -> i32 {
    return input.split("\n").collect::<Vec<&str>>().chunks(3).map(|rucksack_group| compute_priority_from_group(rucksack_group)).sum();
}

fn compute_priority_from_group(rucksack_group: &[&str]) -> i32 {
    let duplicates: Vec<char> = rucksack_group[0].chars().filter(|first_char| rucksack_group[1].chars().any(|second_char| first_char == &second_char)).collect();
    let duplicate: char = rucksack_group[2].chars().find(|first_char| duplicates.contains(first_char)).unwrap();
    return to_number(duplicate);
}

#[cfg(test)]
mod tests {
    use crate::day3::day3::{sum_priorities_part_one, sum_priorities_part_two};
    use crate::utils::file;

    #[test]
    fn should_count_score_part_one() {
        let result = sum_priorities_part_one(file::read_test("day3").as_str());
        assert_eq!(result, 157)
    }

    #[test]
    fn should_count_score_part_two() {
        let result = sum_priorities_part_two(file::read_test("day3").as_str());
        assert_eq!(result, 70)
    }
}
