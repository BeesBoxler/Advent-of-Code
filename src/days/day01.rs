pub fn run(input: String) {
    let elf_totals = read_input(input);

    part_one(&elf_totals);
    part_two(&elf_totals);
}

fn part_one(elf_totals: &Vec<i32>) {
    let mut max = 0;
    let mut index = 0;
    for i in 0..elf_totals.len() {
        if elf_totals[i] > max {
            max = elf_totals[i];
            index = i;
        }
    }
    println!("Elf number {} is carrying the most, with a weight of {}", index + 1, max)
}

fn part_two(elf_totals: &Vec<i32>) {
    let mut sorted_elves = elf_totals.clone();
    sorted_elves.sort_by(|a,b| b.cmp(a));

    let mut total = 0;

    for i in 0..3 {
        total += sorted_elves[i];
    }

    println!("The top three elves are carrying a total of {} calories.", total);
}

fn read_input(input: String) -> Vec<i32> {
    let mut elf_totals = vec![0];

    input.split("\n").for_each(|v| {

        if let Ok(value) = v.parse::<i32>() {
            let index = elf_totals.len() - 1;
            elf_totals[index] += value;
        } else {
            elf_totals.push(0);
        }
    } );

    return elf_totals;
}