use std::collections::HashSet;

type Compartment = (HashSet<u8>, HashSet<u8>);

pub fn run(input: String) {
    let input = format_input(&input);
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &Vec<Compartment>) {
    let mut sum: u32 = 0;

    for line in input {
        for value in line.0.intersection(&line.1) {
            sum += *value as u32;
        }
    }

    println!("The total priority is {}", sum);
}

fn part_two(input: &Vec<Compartment>) {
    let mut sum = 0;
    let mut group = Vec::new();

    for line in input {
        group.push(line.0.union(&line.1).map(|x| *x).collect::<HashSet<u8>>());
        if group.len() == 3 {
            let int = group[0]
                .intersection(&group[1])
                .map(|x| *x)
                .collect::<HashSet<u8>>()
                .intersection(&group[2])
                .map(|x| *x)
                .next()
                .unwrap();
            sum += int as u32;
            group = Vec::new();
        }
    }

    println!("Total sum of all groups: {}", sum);
}

fn format_input(input: &String) -> Vec<Compartment> {
    input
        .split('\n')
        .map(|v| {
            let res = v.split_at(v.len() / 2);
            (
                res.0
                    .chars()
                    .map(|char| ascii_to_value(char as u8))
                    .collect(),
                res.1
                    .chars()
                    .map(|char| ascii_to_value(char as u8))
                    .collect(),
            )
        })
        .collect::<Vec<Compartment>>()
}

fn ascii_to_value(code: u8) -> u8 {
    if code > 96 {
        code - 96
    } else {
        code - 38
    }
}
