use std::collections::HashSet;
use crate::utils::file;

pub fn run() {
    let input = file::read("day6");
    println!("{}", do_something(input.as_str()));
}

fn do_something(input: &str) -> i32 {
    return input.chars().collect::<Vec<char>>()
        .windows(4)
        .map(|buffer| buffer.iter().collect::<HashSet<&char>>().len())
        .position(|size| size == 4).unwrap() as i32 + 4;
}

#[cfg(test)]
mod tests {
    use crate::day6::day6::do_something;
    use crate::utils::file;

    #[test]
    fn should_do_something() {
        let result = do_something(file::read_test("day6").as_str());
        assert_eq!(result, 7)
    }
}
