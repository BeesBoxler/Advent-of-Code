pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn string_to_numbers(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numbers = line
                .bytes()
                .filter(|byte| byte > &47u8 && byte < &58u8)
                .collect::<Vec<u8>>();

            (numbers[0] as u32 - 48) * 10 + (numbers[numbers.len() - 1] as u32 - 48)
        })
        .sum()
}

fn part_one(input: &str) -> u32 {
    string_to_numbers(input)
}

fn part_two(input: &str) -> u32 {
    let input = input
        .lines()
        .map(|line| {
            let mut line = String::from(line);
            let mut i = 1;
            while i < line.len() {
                let len = line.len();
                let curr = *line.as_bytes().get(i - 1).unwrap_or(&0u8) as u32;
                if curr > 47 && curr < 58 {
                    break;
                }
                line.replace_range(
                    0..i,
                    &line[0..i]
                        .replace("one", "1")
                        .replace("two", "2")
                        .replace("three", "3")
                        .replace("four", "4")
                        .replace("five", "5")
                        .replace("six", "6")
                        .replace("seven", "7")
                        .replace("eight", "8")
                        .replace("nine", "9"),
                );
                if line.len() != len {
                    i = 1;
                    continue;
                }

                i += 1;
            }
            i = 0;
            while i < line.len() {
                let len = line.len();

                let j = len - i;
                let curr = *line.as_bytes().get(j).unwrap_or(&0u8) as u32;
                if curr > 47 && curr < 58 {
                    break;
                }
                line.replace_range(
                    j..len,
                    &line[j..len]
                        .replace("one", "1")
                        .replace("two", "2")
                        .replace("three", "3")
                        .replace("four", "4")
                        .replace("five", "5")
                        .replace("six", "6")
                        .replace("seven", "7")
                        .replace("eight", "8")
                        .replace("nine", "9"),
                );
                if line.len() != len {
                    i = 0;
                    continue;
                }
                i += 1;
            }
            line.push('\n');
            line
        })
        .collect::<String>();
    string_to_numbers(&input)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT_DAY_TWO: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 142);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT_DAY_TWO.to_string()), 281);
    }
}
