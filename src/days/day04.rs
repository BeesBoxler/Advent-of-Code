use std::collections::HashSet;

type Line = Vec<HashSet<u8>>;

pub fn run(input: String) {
    let formatted_input = format_input(&input);

    part_one(&formatted_input);
    part_two(&formatted_input);
}

fn part_one(input: &Vec<Line>) {
    let mut sum = 0;
    for line in input {
        let union_length = line[0].union(&line[1]).count();
        if union_length == line[0].len() || union_length == line[1].len() {
            sum += 1;
        }
    }

    println!("Number of sets containing another: {}", sum)
}

fn part_two(input: &Vec<Line>) {
    let mut sum = 0;
    for line in input {
        if line[0].intersection(&line[1]).count() > 0 {
            sum += 1;
        }
    }

    println!("Number of overlaps: {}", sum)
}

fn format_input(input: &String) -> Vec<Line>{
    input.split('\n').map(|line| {
        line.split(',').map(|range| {
            let bounds = range.split('-').map(|i| i.parse().unwrap()).collect::<Vec<u8>>();
            (bounds[0]..=bounds[1]).collect()
        }).collect()
    }).collect()
}