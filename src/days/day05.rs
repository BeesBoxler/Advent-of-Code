use colored::Colorize;
use regex::Regex;
use std::vec;

type Container = Vec<char>;
type Movement = (usize, usize, usize);

pub fn run(input: String) {
    let (containers, movements) = parse_input(&input);
    println!(
        "Top container of each stack is {}",
        part_one(containers.clone(), &movements)
    );

    println!(
        "New top container of each stack is {}",
        part_two(containers.clone(), &movements)
    );
}

fn part_one(mut containers: Vec<Container>, movements: &Vec<Movement>) -> String {
    for container in &mut containers {
        container.reverse();
    }

    for (count, from, to) in movements {
        for _ in 0..*count {
            let value = containers[from - 1].pop().unwrap();
            containers[to - 1].push(value);
        }
    }

    let mut result = Vec::new();
    for container in &containers {
        if container.len() > 0 {
            result.push(container[container.len() - 1] as u8)
        }
    }

    String::from_utf8(result).unwrap()
}

fn part_two(mut containers: Vec<Container>, movements: &Vec<Movement>) -> String {
    for container in &mut containers {
        container.reverse();
    }

    let mut temp = Vec::new();

    for (count, from, to) in movements {
        // draw_crates(
        //     &containers,
        //     format!("Move {:2} from {} to {}", count, from, to),
        //     true,
        // );
        temp.clear();

        for _ in 0..*count {
            temp.push(containers[from - 1].pop().unwrap());
        }

        temp.reverse();
        for value in &temp {
            containers[to - 1].push(*value);
        }
    }
    // draw_crates(&containers, format!("Instructions complete."), true);

    let mut result = Vec::new();
    for container in &containers {
        if container.len() > 0 {
            result.push(container[container.len() - 1] as u8)
        }
    }

    String::from_utf8(result).unwrap()
}

fn draw_crates(containers: &Vec<Container>, instruction: String, highlight_final: bool) {
    print!("{esc}[?25l{esc}[2J{esc}[1;1H", esc = 27 as char);
    for container in containers {
        for i in 0..container.len() {
            if i == container.len() - 1 && highlight_final {
                print!(
                    "{}",
                    format!(
                        "{}{}{}",
                        "[".white(),
                        container[i].to_string().white(),
                        "]".white()
                    )
                    .on_blue()
                );
            } else {
                print!("{}", format!("[{}]", container[i]).on_white());
            }
        }
        println!();
    }
    println!("{}", instruction);
    let mut child = std::process::Command::new("sleep")
        .arg("0.01")
        .spawn()
        .unwrap();
    let _result = child.wait().unwrap();
    print!("{esc}[?25h", esc = 27 as char);
}

fn parse_input(input: &String) -> (Vec<Container>, Vec<Movement>) {
    let data = input.split_once("\n\n").unwrap();
    let move_match = Regex::new(r"move.(\d+).from.(\d).to.(\d)").unwrap();

    let mut containers: Vec<Container> = vec![Vec::new()];
    let mut movements: Vec<Movement> = Vec::new();
    let mut index = 0;
    let mut chars = data.0.chars();

    loop {
        chars.next();
        if let Some(v) = chars.next() {
            if v == '1' {
                break;
            } else {
                if v != ' ' {
                    containers[index].push(v);
                }
            }
        }
        chars.next();
        if let Some(v) = chars.next() {
            if v == '\n' {
                index = 0;
            } else {
                index += 1;
                if index == containers.len() {
                    containers.push(Vec::new());
                }
            }
        } else {
            break;
        }
    }

    for movement in data.1.lines() {
        let matches = move_match.captures_iter(movement).next().unwrap();

        movements.push((
            matches[1].parse().unwrap(),
            matches[2].parse().unwrap(),
            matches[3].parse().unwrap(),
        ))
    }

    (containers, movements)
}

#[cfg(test)]
mod test {
    use super::{parse_input, part_one, part_two};

    static INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn parser_returns_correct_containers() {
        let (containers, _) = parse_input(&INPUT.to_string());
        assert_eq!(
            containers,
            vec!(vec!('N', 'Z'), vec!('D', 'C', 'M'), vec!('P'))
        );
    }

    #[test]
    fn parser_returns_correct_movements() {
        let (_, movements) = parse_input(&INPUT.to_string());
        assert_eq!(movements, vec!((1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)));
    }

    #[test]
    fn part_one_returns_correct_output() {
        let (containers, movements) = parse_input(&INPUT.to_string());
        let result = part_one(containers, &movements);
        assert_eq!(result, "CMZ")
    }

    #[test]
    fn part_two_returns_correct_output() {
        let (containers, movements) = parse_input(&INPUT.to_string());
        let result = part_two(containers, &movements);
        assert_eq!(result, "MCD")
    }
}
