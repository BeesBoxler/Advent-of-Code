pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    let mut max = 0;
    let points = parse_input(input);

    for (i, (x1, y1)) in points.iter().enumerate() {
        for (x2, y2) in &points[i + 1..] {
            max = max.max((x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1));
        }
    }

    max
}

fn part_two(input: &str) -> usize {
    let points = parse_input(input);
    let mut max = 0;
    let mut aux_points = vec![];

    for (x1, y1) in &points {
        for (x2, y2) in &points {
            if x1 == x2 {
                for i in y1 + 1..*y2 {
                    aux_points.push((*x1, i));
                }
            }
            if y1 == y2 {
                for i in x1 + 1..*x2 {
                    aux_points.push((i, *y1))
                }
            }

            // if (x1 < x2 && y1 > y2) || (x1 > x2 && y1 < y2) {
            for i in x1 + 1..*x2 {
                for j in y1 + 1..*y2 {
                    aux_points.push((i, j))
                    // }
                }
            }
        }
    }

    // for i in 0..15 {
    //     for j in 0..15 {
    //         if points.contains(&(j, i)) {
    //             print!("#")
    //         } else if aux_points.contains(&(j, i)) {
    //             print!("X")
    //         } else {
    //             print!(".")
    //         }
    //     }
    //     println!();
    // }

    for (x1, y1) in &points {
        for (x2, y2) in &points {
            let a1 = (*x1, *y2);
            let a2 = (*x2, *y1);

            if (aux_points.contains(&a1) || points.contains(&a1))
                && (points.contains(&a2) || aux_points.contains(&a2))
            {
                max = max.max((x1.abs_diff(*x2) + 1) * (y1.abs_diff(*y2) + 1));
            }
        }
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 50);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 24);
    }
}

