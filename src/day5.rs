use std::collections::BTreeSet;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|seat| {
            seat.chars()
                .map(|c| match c {
                    'F' => '0',
                    'B' => '1',
                    'L' => '0',
                    'R' => '1',
                    _ => unreachable!(),
                })
                .collect::<String>()
        })
        .map(|id_str| usize::from_str_radix(&id_str, 2).unwrap())
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[usize]) -> usize {
    input.iter().max().unwrap().to_owned()
}

#[aoc(day5, part2)]
pub fn part2(input: &[usize]) -> usize {
    let occupied_seats: BTreeSet<usize> = input.iter().cloned().collect();

    let mut occupied_seats = occupied_seats.into_iter();
    let mut last_seat = occupied_seats.next().unwrap();

    for seat_id in occupied_seats {
        if seat_id != last_seat + 1 {
            return seat_id - 1;
        }

        last_seat = seat_id;
    }

    panic!("Oh boi, this shouldn't have happend!");
}
