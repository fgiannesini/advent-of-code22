use crate::utils::file;

pub fn run() {
    let input = file::read("day5");
    println!("{}", compute_top_of_stacks(input.as_str()));
}

#[derive(Debug, Clone)]
struct stack {
    values: Vec<String>,
}

#[derive(Debug, Clone)]
struct stack_group {
    values: Vec<stack>,
}

impl stack {
    pub fn new() -> Self {
        Self { values: vec![] }
    }
    fn add(&mut self, value: String) {
        self.values.push(value);
    }
    fn pop(&mut self) -> String {
        return self.values.pop().unwrap();
    }
    fn last(&self) -> String {
        return self.values.last().unwrap().clone();
    }
}

impl stack_group {
    pub fn new(stack_group: &str) -> Self {
        let mut lines: Vec<&str> = stack_group.split("\r\n").collect();
        let stacks_count = lines.pop().unwrap().split_whitespace().count();
        lines.reverse();
        let mut stacks: Vec<stack> = (0..stacks_count).map(|_| stack::new()).collect();
        lines.iter().for_each(|line| {
            let lineElements = Self::split(line);
            for index in 0..lineElements.len() {
                let elementValue = lineElements[index].clone();
                if !elementValue.is_empty() {
                    stacks[index].add(elementValue)
                }
            }
        });
        Self { values: stacks }
    }
    fn split(line: &&str) -> Vec<String> {
        return line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|c| c.iter().collect::<String>())
            .map(|string| string.replace(&[' ', '[', ']'], ""))
            .collect::<Vec<String>>();
    }

    fn apply_move(&mut self, element_count: i32, origin_index: i32, target_index: i32) {
        (0..element_count).for_each(|_| {
            let moving_element = self.values[origin_index as usize].pop();
            self.values[target_index as usize].add(moving_element)
        })
    }

    fn get_tops(&self) -> String {
        return self.values.iter()
            .map(|stack| stack.last())
            .collect::<Vec<String>>()
            .join("");
    }
}

fn compute_top_of_stacks(input: &str) -> String {
    let parts: Vec<&str> = input.split("\r\n\r\n").collect();
    let (stack_group_part, moves) = (parts[0], parts[1]);
    let mut stack_group = stack_group::new(stack_group_part);
    moves.split("\r\n").for_each(|line| {
        let moves_data: Vec<i32> = line.split_whitespace()
            .filter_map(|part| part.parse::<i32>().ok())
            .collect();
        stack_group.apply_move(moves_data[0], moves_data[1] - 1, moves_data[2] - 1);
    });
    return stack_group.get_tops();
}

#[cfg(test)]
mod tests {
    use crate::day5::day5::{compute_top_of_stacks};
    use crate::utils::file;

    #[test]
    fn should_compute_top_of_stacks() {
        let result = compute_top_of_stacks(file::read_test("day5").as_str());
        assert_eq!(result, "CMZ")
    }
}
