use crate::utils::file;

pub fn run() {
    let input = file::read("day8");
    println!("{}", get_visible_trees(input.as_str()));
}

#[derive(Debug)]
struct Field {
    trees: Vec<Vec<i32>>,
}

impl Field {
    fn new(input: &str) -> Self {
        Self {
            trees: input.split("\n")
                .map(|line| line.split("")
                    .map(|element| element.parse::<i32>())
                    .filter_map(|result| result.ok())
                    .collect::<Vec<i32>>()
                ).collect::<Vec<Vec<i32>>>()
        }
    }

    fn count_all_visible_trees(&self) -> i32 {
        let mut visible_trees = vec![vec![false; self.trees[0].len()]; self.trees.len()];
        let mut min;
        let height = visible_trees.len();
        let width = visible_trees[0].len();
        for i in 0..height {
            min = -1;
            for j in 0..width {
                let value = self.trees[i][j];
                if value > min {
                    visible_trees[i][j] = true;
                    min = value;
                }
            }
        }
        for i in 0..height {
            min = -1;
            for j in 0..width {
                let value = self.trees[i][width - j - 1];
                if value > min {
                    visible_trees[i][width - j - 1] = true;
                    min = value;
                }
            }
        }

        for j in 0..width {
            min = -1;
            for i in 0..height {
                let value = self.trees[i][j];
                if value > min {
                    visible_trees[i][j] = true;
                    min = value;
                }
            }
        }

        for j in 0..width {
            min = -1;
            for i in 0..height {
                let value = self.trees[height - i - 1][j];
                if value > min {
                    visible_trees[height - i - 1][j] = true;
                    min = value;
                }
            }
        }
        return visible_trees.iter()
            .map(|line| line.iter()
                .filter(|cell| **cell)
                .count() as i32
            ).sum();
    }
}

fn get_visible_trees(input: &str) -> i32 {
    let field = Field::new(input);
    return field.count_all_visible_trees();
}

#[cfg(test)]
mod tests {
    use crate::day8::day8::get_visible_trees;
    use crate::utils::file;

    #[test]
    fn should_count_trees() {
        let result = get_visible_trees(file::read_test("day8").as_str());
        assert_eq!(result, 21)
    }
}
