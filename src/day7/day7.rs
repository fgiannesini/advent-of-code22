use crate::utils::file;

#[allow(dead_code)]
pub fn run() {
    let input = file::read("day7");
    println!("{}", get_directories_size_below_100_000(input.as_str()));
    println!("{}", get_folder_to_delete(input.as_str()));
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
            .filter(|(_index, &parent_index)| parent_index.map(|i| i == self.current).unwrap_or(false))
            .map(|(index, &_parent_index)| index)
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

    fn get_folder_to_delete(&self) -> i32 {
        let remaining_size = 70000000 - self.files_size[0];
        return *self.files_size.iter()
            .filter(| size| *size + remaining_size as i32 > 30000000)
            .min()
            .unwrap();
    }
}

fn get_directories_size_below_100_000(input: &str) -> i32 {
    let directory_tree = build_directory_tree(input);
    return directory_tree.sum_sizes_under(100000);
}

fn get_folder_to_delete(input: &str) -> i32 {
    let directory_tree = build_directory_tree(input);
    return directory_tree.get_folder_to_delete();
}

fn build_directory_tree(input: &str) -> DirectoryTree {
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
    directory_tree
}

#[cfg(test)]
mod tests {
    use crate::day7::day7::{get_directories_size_below_100_000, get_folder_to_delete};
    use crate::utils::file;

    #[test]
    fn should_get_directories_size_below_100_000() {
        let result = get_directories_size_below_100_000(file::read_test("day7").as_str());
        assert_eq!(result, 95437)
    }

    #[test]
    fn should_get_size_to_delete() {
        let result = get_folder_to_delete(file::read_test("day7").as_str());
        assert_eq!(result, 24933642)
    }
}
