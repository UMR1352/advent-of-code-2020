use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Floor,
    EmptySeat,
    TakenSeat,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match *self {
            Cell::Floor => '.',
            Cell::EmptySeat => 'L',
            Cell::TakenSeat => '#',
        };

        write!(f, "{}", c)
    }
}

impl From<char> for Cell {
    fn from(x: char) -> Self {
        match x {
            '.' => Cell::Floor,
            'L' => Cell::EmptySeat,
            '#' => Cell::TakenSeat,
            _ => unreachable!(),
        }
    }
}

type Pos = (i32, i32);
const ADJACENT_POS: [Pos; 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

#[derive(Clone)]
pub struct Map {
    width: usize,
    pub data: Vec<Cell>,
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.data.iter().enumerate().for_each(|(i, x)| {
            if i % self.width == 0 {
                write!(f, "\n").unwrap();
            }

            write!(f, "{}", x).unwrap();
        });

        Ok(())
    }
}
impl Map {
    fn index_of(&self, pos: Pos) -> usize {
        let (x, y) = pos;

        y as usize * self.width + x as usize
    }

    fn pos_of(&self, idx: usize) -> Pos {
        let x = idx % self.width;
        let y = idx / self.width;

        (x as i32, y as i32)
    }

    fn count_visible_taken_seats(&self, idx: usize, max_depth: usize) -> usize {
        let (x, y) = self.pos_of(idx);

        ADJACENT_POS.iter().fold(0, |cnt, (x_off, y_off)| {
            let mut depth = 0;
            let mut x = x;
            let mut y = y;

            loop {
                depth += 1;
                x += x_off;
                y += y_off;

                if depth > max_depth
                    || x < 0
                    || y < 0
                    || x >= self.width as i32
                    || y >= (self.data.len() / self.width) as i32
                {
                    break cnt;
                }

                unsafe {
                    match self.data.get_unchecked(self.index_of((x, y))) {
                        Cell::Floor => continue,
                        Cell::EmptySeat => break cnt,
                        Cell::TakenSeat => break cnt + 1,
                    }
                }
            }
        })
    }

    fn rule1(&self, idx: usize, depth: usize) -> Cell {
        if self.count_visible_taken_seats(idx, depth) > 0 {
            Cell::EmptySeat
        } else {
            Cell::TakenSeat
        }
    }

    fn rule2(&self, idx: usize, tollerance: usize, depth: usize) -> Cell {
        if self.count_visible_taken_seats(idx, depth) >= tollerance {
            Cell::EmptySeat
        } else {
            Cell::TakenSeat
        }
    }

    pub fn step(&mut self, depth: usize, tollerance: usize) -> (usize, bool) {
        let mut new_data = self.data.clone();

        let result =
            new_data
                .iter_mut()
                .enumerate()
                .fold((0, false), |(taken, changes), (idx, seat)| match seat {
                    Cell::Floor => (taken, changes),
                    Cell::EmptySeat => {
                        let new_seat = self.rule1(idx, depth);
                        let changed = !matches!(new_seat, Cell::EmptySeat);

                        *seat = new_seat;
                        (taken + changed as usize, changes || changed)
                    }
                    Cell::TakenSeat => {
                        let new_seat = self.rule2(idx, tollerance, depth);
                        let changed = !matches!(new_seat, Cell::TakenSeat);

                        *seat = new_seat;
                        (taken + !changed as usize, changes || changed)
                    }
                });

        self.data = new_data;

        result
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Map {
    let width = input.find('\n').unwrap();
    let data = input
        .lines()
        .flat_map(|l| l.chars())
        .map(Cell::from)
        .collect();

    Map { width, data }
}

#[aoc(day11, part1)]
pub fn part1(input: &Map) -> usize {
    let mut map = input.clone();

    loop {
        let (taken_seats, changed) = map.step(1, 4);

        if !changed {
            break taken_seats;
        }
    }
}

#[aoc(day11, part2)]
pub fn part2(input: &Map) -> usize {
    let mut map = input.clone();

    loop {
        let (taken_seats, changed) = map.step(usize::MAX, 5);

        if !changed {
            break taken_seats;
        }
    }
}
