use std::collections::HashSet;

pub type Input = (Vec<u64>, HashSet<u64>);

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Input {
    let xs: Vec<u64> = input.lines().map(|x| x.parse::<u64>().unwrap()).collect();
    let set: HashSet<u64> = xs.iter().cloned().collect();

    (xs, set)
}

#[aoc(day1, part1)]
pub fn part1(input: &Input) -> u64 {
    let (xs, set) = input;

    for x in xs {
        if set.contains(&(2020 - x)) {
            return x * (2020 - x);
        }
    }

    0
}

#[aoc(day1, part2)]
pub fn part2(input: &Input) -> u64 {
    let (xs, set) = input;

    for (i, x) in xs.iter().enumerate() {
        for y in xs.iter().skip(i) {
            let z = 2020 - x - y;
            if set.contains(&z) {
                return x * y * z;
            }
        } 
    }

    0
}
