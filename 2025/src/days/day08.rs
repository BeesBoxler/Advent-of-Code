use std::{collections::HashSet, fmt::Display};

pub fn run(input: String) {
    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[derive(Debug, Clone)]
struct Distance {
    to: Point,
    from: Point,
    distance: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x:{}, y:{}, z:{})", self.x, self.y, self.z)
    }
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let mut vals = value.splitn(3, ',');

        Point {
            x: vals.next().unwrap().parse().unwrap(),
            y: vals.next().unwrap().parse().unwrap(),
            z: vals.next().unwrap().parse().unwrap(),
        }
    }
}

impl Point {
    fn distance_to(&self, other: &Self) -> usize {
        (self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2))
        .isqrt()
    }
}

fn create_circuits(points: Vec<Point>, count: usize) -> (Vec<HashSet<Point>>, (Point, Point)) {
    let mut count = count;
    let mut distances = vec![];
    let mut circuits: Vec<HashSet<Point>> =
        points.iter().map(|point| HashSet::from([*point])).collect();

    for (i, point) in points.iter().enumerate() {
        for other_point in &points[i + 1..] {
            let distance = point.distance_to(other_point);

            distances.push(Distance {
                from: point.clone(),
                to: other_point.clone(),
                distance,
            });
        }
    }

    distances.sort_by(|a, b| a.distance.cmp(&b.distance).reverse());

    let mut last = None;

    while count > 0 && distances.len() > 1 {
        let Distance {
            from,
            to,
            distance: _,
        } = distances.pop().unwrap();

        let mut a = 0;
        let mut b = 0;

        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&from) {
                a = i;
            }
            if circuit.contains(&to) {
                b = i;
            }
        }

        if a != b {
            let extends = circuits.remove(a.max(b));
            circuits[a.min(b)].extend(extends);
            last = Some((to, from))
        }
        count -= 1;
    }

    circuits.sort_by(|a, b| a.len().cmp(&b.len()).reverse());

    (circuits, last.unwrap())
}

fn part_one(input: &str) -> usize {
    let points: Vec<Point> = input.lines().map(|line| line.into()).collect();
    let count = if points.len() > 50 { 1000 } else { 10 };

    let (circuits, _) = create_circuits(points, count);

    circuits[0..3].iter().map(|c| c.len()).product()
}

fn part_two(input: &str) -> usize {
    let points: Vec<Point> = input.lines().map(|line| line.into()).collect();
    let count = usize::MAX;

    let (_, last) = create_circuits(points, count);

    last.1.x * last.0.x
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn part_one_returns_correct_output() {
        assert_eq!(part_one(INPUT), 40);
    }

    #[test]
    fn part_two_returns_correct_output() {
        assert_eq!(part_two(INPUT), 25272);
    }
}

