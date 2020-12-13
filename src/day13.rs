use ring_algorithm::chinese_remainder_theorem;

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> (u64, Vec<Option<u64>>) {
    let (time, busses) = input.split_at(input.find('\n').unwrap());
    let time = time.parse().unwrap();
    let busses = busses
        .trim_start()
        .split(',')
        .map(|val| val.parse::<u64>().ok())
        .collect();

    (time, busses)
}

#[aoc(day13, part1)]
pub fn part1(input: &(u64, Vec<Option<u64>>)) -> u64 {
    let (time, busses) = input;

    let (waiting_time, next_bus) = busses
        .iter()
        .filter_map(|x| *x)
        .map(|bus| (bus - time % bus, bus))
        .min_by_key(|(wait, _)| *wait)
        .unwrap();

    waiting_time * next_bus
}

#[aoc(day13, part2)]
pub fn part2(input: &(u64, Vec<Option<u64>>)) -> u64 {
    let (_, busses) = input;
    let (rhs, mods): (Vec<_>, Vec<_>) = busses
        .iter()
        .enumerate()
        .filter_map(|(i, x)| x.map_or(None, |x| Some((-(i as isize), x as isize))))
        .unzip();

    chinese_remainder_theorem(&rhs, &mods).unwrap() as u64
}

#[test]
pub fn part2_example() {
    let busses: &[(usize, u64)] = &[(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)];
    let time_stamp = 1068781;

    assert!(busses
        .iter()
        .all(|(i, x)| (time_stamp + *i as u64) % x == 0))
}
