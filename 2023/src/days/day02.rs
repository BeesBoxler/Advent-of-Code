pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    let r_max = 12;
    let g_max = 13;
    let b_max = 14;
    lines.for_each(|line| {
        let mut valid = true;

        let sections: Vec<&str> = line.split(':').collect();
        let mut game = sections[0].split_whitespace();
        game.next();
        let id: u32 = game.next().unwrap().parse().unwrap();
        let draws = sections[1].split(';');
        draws.for_each(|draw| {
            let mut r_sum = 0;
            let mut g_sum = 0;
            let mut b_sum = 0;

            let words: Vec<&str> = draw.split_whitespace().collect();
            for i in 0..words.len() {
                match words[i] {
                    "red" | "red," => {
                        let count: u8 = words[i - 1].parse().unwrap();
                        r_sum += count;
                    }
                    "green" | "green," => {
                        let count: u8 = words[i - 1].parse().unwrap();
                        g_sum += count
                    }
                    "blue" | "blue," => {
                        let count: u8 = words[i - 1].parse().unwrap();
                        b_sum += count;
                    }
                    _ => {}
                }

                if r_sum.gt(&r_max) || g_sum.gt(&g_max) || b_sum.gt(&b_max) {
                    valid = false;
                    break;
                }
            }
        });

        if valid {
            sum += id
        };
    });
    sum
}

fn part_two(input: &str) -> u32 {
    let lines = input.lines();
    let mut sum = 0;
    lines.for_each(|line| {
        let mut r_max = 0;
        let mut g_max = 0;
        let mut b_max = 0;

        let mut draws = line.split(':');
        draws.next();
        let draws = draws.next().unwrap().split(';');
        draws.for_each(|draw| {
            let mut r_sum: u32 = 0;
            let mut g_sum: u32 = 0;
            let mut b_sum: u32 = 0;

            let words: Vec<&str> = draw.split_whitespace().collect();
            for i in 0..words.len() {
                match words[i] {
                    "red" | "red," => {
                        let count: u32 = words[i - 1].parse().unwrap();
                        r_sum += count;
                    }
                    "green" | "green," => {
                        let count: u32 = words[i - 1].parse().unwrap();
                        g_sum += count
                    }
                    "blue" | "blue," => {
                        let count: u32 = words[i - 1].parse().unwrap();
                        b_sum += count;
                    }
                    _ => {}
                }
                r_max = u32::max(r_sum, r_max);
                g_max = u32::max(g_sum, g_max);
                b_max = u32::max(b_sum, b_max);
            }
        });

        sum += r_max * g_max * b_max;
    });
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 8);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 2286);
    }
}
