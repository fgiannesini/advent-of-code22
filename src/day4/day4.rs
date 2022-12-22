use crate::utils::file;

#[allow(dead_code)]
pub fn run() {
    let input = file::read("day4");
    println!("{}", count_included_pairs(input.as_str()));
    println!("{}", count_overlapped_pairs(input.as_str()));
}

fn count_included_pairs(input: &str) -> i32 {
    return input.split("\n").filter(|pairs| is_included(pairs.replace('\r', "").as_str())).count() as i32;
}

fn is_included(pairs: &str) -> bool {
    let parts: Vec<i32> = pairs.split(&[',', '-']).map(|part| part.parse::<i32>().unwrap()).collect();
    return parts[0] <= parts[2] && parts[1] >= parts[3] || parts[0] >= parts[2] && parts[1] <= parts[3]
}


fn count_overlapped_pairs(input: &str) -> i32 {
    return input.split("\n").filter(|pairs| is_overlapped(pairs.replace('\r', "").as_str())).count() as i32;
}

fn is_overlapped(pairs: &str) -> bool {
    let parts: Vec<i32> = pairs.split(&[',', '-']).map(|part| part.parse::<i32>().unwrap()).collect();
    return parts[0] >= parts[2] && parts[0] <= parts[3] || parts[1] >= parts[2] && parts[1] <= parts[3]
        || parts[2] >= parts[0] && parts[2] <= parts[1] || parts[3] >= parts[0] && parts[3] <= parts[1]
}

#[cfg(test)]
mod tests {
    use crate::day4::day4::{count_included_pairs, count_overlapped_pairs};
    use crate::utils::file;

    #[test]
    fn should_count_included_part_one() {
        let result = count_included_pairs(file::read_test("day4").as_str());
        assert_eq!(result, 2)
    }

    #[test]
    fn should_count_overlapped_part_two() {
        let result = count_overlapped_pairs(file::read_test("day4").as_str());
        assert_eq!(result, 4)
    }
}
