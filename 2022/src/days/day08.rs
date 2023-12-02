type Map = Vec<Vec<u8>>;

pub fn run(input: String) {
    println!("Number of trees visible from outside: {}", part_one(&input));
    println!("Maximum scenic score: {}", part_two(&input));
}

fn part_one(input: &String) -> u32 {
    let mut sum: u32 = 0;
    let input = parse_input(input);
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let curr_tree = input[i][j];
            let mut visible: u8 = 0b1111;
            if i == 0 || j == 0 || i == input.len() - 1 || j == input[i].len() - 1 {
                sum += 1;
                continue;
            }

            // visible from the north?
            for tree in 0..i {
                if input[tree][j] >= curr_tree {
                    visible >>= 1;
                    break;
                }
            }

            // visible from the south?
            for tree in i + 1..input.len() {
                if input[tree][j] >= curr_tree {
                    visible >>= 1;
                    break;
                }
            }

            // visible from the west?
            for tree in 0..j {
                if input[i][tree] >= curr_tree {
                    visible >>= 1;
                    break;
                }
            }

            // visible from the south?
            for tree in j + 1..input[i].len() {
                if input[i][tree] >= curr_tree {
                    visible >>= 1;
                    break;
                }
            }

            sum += 1 & visible as u32;
        }
    }
    sum
}

fn part_two(input: &String) -> u32 {
    let mut max_scenic_score: u32 = 0;
    let input = parse_input(input);
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            let curr_tree = input[i][j];
            let mut score = 1;
            let mut distance = 0;
            if i == 0 || j == 0 || i == input.len() - 1 || j == input[i].len() - 1 {
                continue;
            }

            // visible from the north?
            for tree in (0..i).rev() {
                distance += 1;
                if input[tree][j] >= curr_tree {
                    break;
                }
            }

            score *= distance;
            distance = 0;

            // visible from the south?
            for tree in i + 1..input.len() {
                distance += 1;
                if input[tree][j] >= curr_tree {
                    break;
                }
            }

            score *= distance;
            distance = 0;

            // visible from the west?
            for tree in (0..j).rev() {
                distance += 1;
                if input[i][tree] >= curr_tree {
                    break;
                }
            }

            score *= distance;
            distance = 0;

            // visible from the south?
            for tree in j + 1..input[i].len() {
                distance += 1;
                if input[i][tree] >= curr_tree {
                    break;
                }
            }

            score *= distance;

            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }
    max_scenic_score
}

fn parse_input(input: &String) -> Map {
    input
        .lines()
        .map(|line| line.bytes().map(|b| b - 48).collect())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 21)
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 8)
    }
}
