use crate::utils::file;

pub fn run() {
    let input = file::read("day7");
    println!("{}", get_directories_size_below_100_000(input.as_str()));
}

#[derive(Debug)]
struct DirectoryTree {
    parents: Vec<Option<usize>>,
    names: Vec<String>,
    files_size: Vec<i32>,
    current: usize,
}

impl DirectoryTree {
    fn new() -> Self {
        return Self {
            parents: vec![None],
            names: vec!["/".to_string()],
            files_size: vec![0],
            current: 0,
        };
    }
    fn add_directory(&mut self, name: String) {
        self.parents.push(Some(self.current));
        self.names.push(name);
        self.files_size.push(0);
    }

    fn set_on_parent(&mut self) {
        self.current = self.parents[self.current].unwrap();
    }

    fn set_on(&mut self, name: String) {
        let indices = self.parents
            .iter()
            .enumerate()
            .filter(|(index, &parent_index)| parent_index.map(|i| i == self.current).unwrap_or(false))
            .map(|(index, &parent_index)| index)
            .collect::<Vec<usize>>();
        self.current = *indices.iter().find(|&&index| self.names[index] == name).unwrap();
    }

    fn add_file(&mut self, file_size: i32) {
        let mut current = self.current;
        self.files_size[current] = self.files_size[current] + file_size;
        while self.parents[current].is_some() {
            current = self.parents[current].unwrap();
            self.files_size[current] = self.files_size[current] + file_size;
        }
    }
    fn sum_sizes_under(&self, size_limit: i32) -> i32 {
        return self.files_size.iter().filter(|size| *size < &size_limit).sum();
    }
}

fn get_directories_size_below_100_000(input: &str) -> i32 {
    let mut directory_tree: DirectoryTree = DirectoryTree::new();
    let split = input.split("\r\n").collect::<Vec<&str>>();
    for line in split {
        match line {
            "$ cd /" => {}
            "$ ls" => {}
            line if line.starts_with("dir") => {
                let new_directory_name = line.split_whitespace().last().unwrap().to_string();
                directory_tree.add_directory(new_directory_name)
            }
            line if line.starts_with("$ cd ..") => {
                directory_tree.set_on_parent();
            }
            line if line.starts_with("$ cd") => {
                let directory_name = line.split_whitespace().collect::<Vec<&str>>().last().unwrap().to_string();
                directory_tree.set_on(directory_name);
            }
            line => {
                let file_size = line.split_whitespace().collect::<Vec<&str>>().first().map(|size_as_string| size_as_string.parse::<i32>()).unwrap().unwrap();
                directory_tree.add_file(file_size);
            }
        }
    }

    return directory_tree.sum_sizes_under(100000);
}

#[cfg(test)]
mod tests {
    use crate::day7::day7::get_directories_size_below_100_000;
    use crate::utils::file;

    #[test]
    fn should_detect_packet_marker() {
        let result = get_directories_size_below_100_000(file::read_test("day7").as_str());
        assert_eq!(result, 95437)
    }
}
