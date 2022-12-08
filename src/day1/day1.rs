use std::fs;

pub fn run() {
    let input = fs::read_to_string("src/day1/day1.txt").expect("File read");
    println!("{}", get_max_calories(input.as_str()));
    println!("{}", get_max_top_3_calories(input.as_str()))
}

fn get_max_calories(input: &str) -> i64 {
    let elfes = get_elfes(input);
    return elfes.iter()
        .map(|elfe| elfe.split("\n")
            .map(|calorie| calorie.parse::<i64>().unwrap())
            .sum())
        .max().unwrap();
}


fn get_max_top_3_calories(input: &str) -> i32 {
    let elfes: Vec<&str> = get_elfes(input);
    let mut map: Vec<i32> = elfes.iter()
        .map(|elfe| elfe.split("\n")
            .map(|calorie| calorie.parse::<i32>().unwrap())
            .sum())
        .collect();
    map.sort();
    map.reverse();
    return map[0..3].iter().sum();
}

fn get_elfes(input: &str) -> Vec<&str> {
    let mut chars = input.chars();
    chars.next_back();
    chars.as_str().split("\n\n").collect()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::day1::day1::{get_max_calories, get_max_top_3_calories};

    #[test]
    fn should_get_max_elf_calories() {
        let max_calories = get_max_calories(read_input().as_str());
        assert_eq!(max_calories, 24000)
    }

    #[test]
    fn should_get_max_top_3_elf_calories() {
        let max_calories = get_max_top_3_calories(read_input().as_str());
        assert_eq!(max_calories, 45000)
    }

    fn read_input() -> String {
        fs::read_to_string("src/day1/day1-test.txt").expect("File read")
    }
}
