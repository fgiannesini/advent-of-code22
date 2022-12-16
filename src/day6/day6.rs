use std::collections::HashSet;
use crate::utils::file;

pub fn run() {
    let input = file::read("day6");
    println!("{}", detect_packet_marker(input.as_str()));
    println!("{}", detect_message_marker(input.as_str()));
}

fn detect_packet_marker(input: &str) -> i32 {
    return detect_marker(input, 4);
}

fn detect_message_marker(input: &str) -> i32 {
    return detect_marker(input, 14);
}

fn detect_marker(input: &str, marker_size : usize) -> i32 {
    let marker_index = input.chars().collect::<Vec<char>>()
        .windows(marker_size)
        .map(|buffer| buffer.iter().collect::<HashSet<&char>>().len())
        .position(|size| size == marker_size).unwrap() + marker_size;
    return marker_index as i32;
}

#[cfg(test)]
mod tests {
    use crate::day6::day6::{detect_message_marker, detect_packet_marker};
    use crate::utils::file;

    #[test]
    fn should_detect_packet_marker() {
        let result = detect_packet_marker(file::read_test("day6").as_str());
        assert_eq!(result, 7)
    }

    #[test]
    fn should_detect_message_marker() {
        let result = detect_message_marker(file::read_test("day6").as_str());
        assert_eq!(result, 19)
    }
}
