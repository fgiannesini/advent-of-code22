use std::fs;

pub fn read(file_name: &str) -> String {
    let input_as_string = fs::read_to_string(format!("src/{0}/{0}.txt", file_name)).expect("File read");
    let mut input_as_chars = input_as_string.chars();
    input_as_chars.next_back();
    return input_as_chars.as_str().to_string();
}

pub fn read_test(file_name: &str) -> String {
    let input_as_string = fs::read_to_string(format!("src/{0}/{0}-test.txt", file_name)).expect("File read");
    let mut input_as_chars = input_as_string.chars();
    input_as_chars.next_back();
    return input_as_chars.as_str().to_string();
}
