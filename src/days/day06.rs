use std::collections::HashSet;

pub fn run(input: String) {
    println!(
        "We discover the marker after {} characters",
        part_one(&input)
    );
    println!(
        "We discover the message marker after {} characters",
        part_two(&input)
    );
}

fn part_one(input: &String) -> usize {
    const WINDOW_SIZE: usize = 4;
    get_end_of_unique_windowun(input, WINDOW_SIZE)
}

fn part_two(input: &String) -> usize {
    const WINDOW_SIZE: usize = 14;
    get_end_of_unique_windowun(input, WINDOW_SIZE)
}

fn get_end_of_unique_windowun(input: &String, window_size: usize) -> usize {
    let mut window = Vec::with_capacity(window_size);
    for _ in 0..window_size {
        window.push(0);
    }
    let mut unique_window = HashSet::new();
    let chars: Vec<char> = input.chars().collect();
    for i in 0..chars.len() {
        window[i % window_size] = chars[i] as usize;

        if i < window_size {
            continue;
        }

        unique_window.clear();
        for c in &window {
            unique_window.insert(*c);
        }

        if unique_window.len() == window_size {
            return i + 1;
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: [(&str, usize, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn part_one_returns_correct_output() {
        for (input, result, _) in INPUT {
            assert_eq!(part_one(&input.to_string()), result);
        }
    }

    #[test]
    fn part_two_returns_correct_output() {
        for (input, _, result) in INPUT {
            assert_eq!(part_two(&input.to_string()), result);
        }
    }
}
