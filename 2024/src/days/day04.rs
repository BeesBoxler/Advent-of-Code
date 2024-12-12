pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect();
    let mut count = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 'X' {
                if i > 2 && data[i - 1][j] == 'M' && data[i - 2][j] == 'A' && data[i - 3][j] == 'S'
                {
                    count += 1
                }
                if i < data.len() - 3
                    && data[i + 1][j] == 'M'
                    && data[i + 2][j] == 'A'
                    && data[i + 3][j] == 'S'
                {
                    count += 1
                }
                if j > 2 && data[i][j - 1] == 'M' && data[i][j - 2] == 'A' && data[i][j - 3] == 'S'
                {
                    count += 1
                }
                if j < data[i].len() - 3
                    && data[i][j + 1] == 'M'
                    && data[i][j + 2] == 'A'
                    && data[i][j + 3] == 'S'
                {
                    count += 1
                }
                if i > 2
                    && j > 2
                    && data[i - 1][j - 1] == 'M'
                    && data[i - 2][j - 2] == 'A'
                    && data[i - 3][j - 3] == 'S'
                {
                    count += 1
                }
                if i > 2
                    && j < data[i].len() - 3
                    && data[i - 1][j + 1] == 'M'
                    && data[i - 2][j + 2] == 'A'
                    && data[i - 3][j + 3] == 'S'
                {
                    count += 1
                }
                if i < data.len() - 3
                    && j > 2
                    && data[i + 1][j - 1] == 'M'
                    && data[i + 2][j - 2] == 'A'
                    && data[i + 3][j - 3] == 'S'
                {
                    count += 1
                }
                if i < data.len() - 3
                    && j < data[i].len() - 3
                    && data[i + 1][j + 1] == 'M'
                    && data[i + 2][j + 2] == 'A'
                    && data[i + 3][j + 3] == 'S'
                {
                    count += 1;
                }
            }
        }
    }

    count
}

fn part_two(input: &str) -> u32 {
    let data: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().map(|c| c).collect())
        .collect();
    let mut count = 0;
    for i in 1..data.len() - 1 {
        for j in 1..data[i].len() - 1 {
            if data[i][j] == 'A' {
                let a = data[i - 1][j - 1] == 'M' && data[i + 1][j + 1] == 'S';
                let b = data[i - 1][j + 1] == 'M' && data[i + 1][j - 1] == 'S';
                let c = data[i + 1][j - 1] == 'M' && data[i - 1][j + 1] == 'S';
                let d = data[i + 1][j + 1] == 'M' && data[i - 1][j - 1] == 'S';

                if a && b || a && c || b && d || c && d {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(&INPUT.to_string()), 18);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(&INPUT.to_string()), 9);
    }
}
