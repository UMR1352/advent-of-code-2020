use std::collections::BTreeSet;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> BTreeSet<u64> {
    input.lines().map(|x| x.parse::<u64>().unwrap()).collect()
}

fn differences(xs: &BTreeSet<u64>) -> impl Iterator<Item = u64> + '_ {
    xs.iter().scan(0, |acc, x| {
        let diff = *x - *acc;
        *acc = *x;

        Some(diff)
    })
}

#[aoc(day10, part1)]
pub fn part1(input: &BTreeSet<u64>) -> usize {
    let mut diffs = vec![0, 0, 1];

    differences(input).for_each(|d| diffs[d as usize - 1] += 1);

    diffs[0] * diffs[2]
}

#[aoc(day10, part2)]
pub fn part2(input: &BTreeSet<u64>) -> u64 {
    let perms = |cnt| 2_u64.pow(cnt as u32 - 1).min(7);
    let result = differences(input).fold((1, 0), |(tot, cnt), x| match x {
        1 => (tot, cnt + 1),
        _ if cnt > 1 => (tot * perms(cnt), 0),
        _ => (tot, 0),
    });

    result.0 * perms(result.1)
}
