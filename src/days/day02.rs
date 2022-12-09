pub fn run(input: String) {
    part_one(&input);
    part_two(&input);
}

fn part_one(input: &String) {
    let mut total = 0;
    input
        .split('\n')
        .for_each(|round| match round {
            "A X" => total += 4,
            "B Y" => total += 5,
            "C Z" => total += 6,

            "C X" => total += 7,
            "A Y" => total += 8,
            "B Z" => total += 9,

            "B X" => total += 1,
            "C Y" => total += 2,
            "A Z" => total += 3,

            _ => {}
        });

    println!("Our incorrect score was {}", total);
}

fn part_two(input: &String) {
    let mut total = 0;
    input
        .split('\n')
        .for_each(|round| match round {
            "A X" => total += 3,
            "A Y" => total += 4,
            "A Z" => total += 8,

            "B X" => total += 1,
            "B Y" => total += 5,
            "B Z" => total += 9,

            "C X" => total += 2,
            "C Y" => total += 6,
            "C Z" => total += 7,

            _ => {}
        });

    println!("The correct score was {}", total);
}

/*
A Y
B X
C Z
*/
