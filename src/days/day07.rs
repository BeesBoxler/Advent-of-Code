use std::collections::BTreeMap;

type DirSizes = BTreeMap<String, u32>;

const FILE_SYSTEM_SIZE: u32 = 70000000;
const REQUIRED_FREE_SPACE: u32 = 30000000;

pub fn run(input: String) {
    println!(
        "Sum of all folders below 100,000 units is {}",
        part_one(&input)
    );
    println!(
        "The Smallest single folder you can delete has size {}",
        part_two(&input)
    )
}

fn part_one(input: &String) -> u32 {
    let mut sum = 0;

    let dir_sizes = parse_directories(input);

    for (_, size) in dir_sizes.into_iter() {
        if size <= 100000 {
            sum += size;
        }
    }

    sum
}

fn part_two(input: &String) -> u32 {
    let dir_sizes = parse_directories(input);
    let reqired_free_space = REQUIRED_FREE_SPACE - (FILE_SYSTEM_SIZE - *dir_sizes.get("").unwrap());

    let mut candidates: Vec<u32> = dir_sizes
        .into_iter()
        .filter_map(|(_, size)| {
            if size > reqired_free_space {
                Some(size)
            } else {
                None
            }
        })
        .collect();
    candidates.sort();

    candidates[0]
}

fn parse_directories(input: &String) -> DirSizes {
    let mut lines = input.lines().peekable();

    let mut path: Vec<&str> = Vec::new();
    let mut dir_sizes = BTreeMap::new();

    while let Some(line) = lines.next() {
        let mut components = line.split(' ');

        if let Some(prefix) = components.next() {
            match prefix {
                "$" => match components.next().unwrap() {
                    "cd" => match components.next().unwrap() {
                        "/" => path.push(""),
                        ".." => {
                            let dir_size = *dir_sizes.entry(path.join("/")).or_default();
                            path.pop();
                            dir_sizes
                                .entry(path.join("/"))
                                .and_modify(|s| *s += dir_size)
                                .or_insert(dir_size);
                        }
                        dir => path.push(dir),
                    },
                    "ls" => loop {
                        let current_path = path.join("/");
                        if &lines.peek().unwrap_or(&"$")[..1] == "$" {
                            break;
                        }

                        if let Some(next_line) = lines.next() {
                            if let Ok(size) = next_line.split_once(' ').unwrap().0.parse::<u32>() {
                                dir_sizes
                                    .entry(current_path)
                                    .and_modify(|s| *s += size)
                                    .or_insert(size);
                            }
                        }
                    },
                    err_cmd => panic!("Unrecognised command: {}", err_cmd),
                },
                _ => panic!(),
            };
        };
    }

    while path.len() > 1 {
        let dir_size = *dir_sizes.entry(path.join("/")).or_default();
        path.pop();
        dir_sizes
            .entry(path.join("/"))
            .and_modify(|s| *s += dir_size)
            .or_insert(dir_size);
    }

    dir_sizes
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 95437);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 24933642);
    }
}
