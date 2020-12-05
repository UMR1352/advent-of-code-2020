use std::collections::BTreeSet;
use std::convert::TryFrom;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Front,
    Back,
    Right,
    Left,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'F' => Ok(Direction::Front),
            'B' => Ok(Direction::Back),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SeatSpace {
    pub left: usize,
    pub right: usize,
    pub top: usize,
    pub bottom: usize,
    current_pos: (usize, usize),
}

impl Default for SeatSpace {
    fn default() -> Self {
        Self {
            left: 0,
            right: 8,
            top: 0,
            bottom: 128,
            current_pos: (0, 0),
        }
    }
}

impl SeatSpace {
    pub fn new(top: usize, bottom: usize, left: usize, right: usize) -> Self {
        Self {
            top,
            bottom,
            left,
            right,
            current_pos: (left, top),
        }
    }
    pub fn get_partition(&self, dir: Direction) -> Self {
        let mut partition = self.clone();
        let h_mid = (self.left + self.right) / 2;
        let v_mid = (self.top + self.bottom) / 2;

        match dir {
            Direction::Front => partition.bottom = v_mid,
            Direction::Back => partition.top = v_mid,
            Direction::Left => partition.right = h_mid,
            Direction::Right => partition.left = h_mid,
        }

        partition
    }

    pub fn get_id(&self) -> usize {
        assert!(self.left == self.right - 1);
        assert!(self.top == self.bottom - 1);

        self.top * 8 + self.left
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|seat| {
            seat.chars()
                .map(|c| Direction::try_from(c).unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[Vec<Direction>]) -> usize {
    input
        .iter()
        .map(|seat| {
            seat.iter()
                .fold(SeatSpace::default(), |acc, d| acc.get_partition(*d))
                .get_id()
        })
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &[Vec<Direction>]) -> usize {
    let occupied_seats: BTreeSet<usize> = input
        .iter()
        .map(|seat| {
            seat.iter()
                .fold(SeatSpace::default(), |acc, d| acc.get_partition(*d))
                .get_id()
        })
        .collect();

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
