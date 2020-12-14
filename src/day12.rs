use std::convert::TryInto;

pub type Pos = (isize, isize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward,
}

impl TryInto<Pos> for Direction {
    type Error = ();

    fn try_into(self) -> Result<Pos, Self::Error> {
        match self {
            Self::North => Ok((0, 1)),
            Self::East => Ok((1, 0)),
            Self::South => Ok((0, -1)),
            Self::West => Ok((-1, 0)),
            _ => Err(()),
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Direction {
        match c {
            'N' => Direction::North,
            'E' => Direction::East,
            'S' => Direction::South,
            'W' => Direction::West,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'F' => Direction::Forward,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Action {
    pub dir: Direction,
    pub amount: usize,
}

impl Action {
    pub fn new(dir: Direction, amount: usize) -> Self {
        match dir {
            Direction::Left | Direction::Right => Self {
                amount: amount / 90,
                dir,
            },
            _ => Self { amount, dir },
        }
    }
}

#[derive(Debug)]
pub struct WayPoint {
    pub x_off: isize,
    pub y_off: isize,
}

#[derive(Debug)]
pub struct Ferry {
    pub x: isize,
    pub y: isize,
    pub waypoint: WayPoint,
}

impl Default for Ferry {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            waypoint: WayPoint {
                x_off: 10,
                y_off: 1,
            },
        }
    }
}

impl Ferry {
    fn move_ferry(&mut self, dir: Pos, amt: usize) {
        let (dx, dy) = dir;

        self.x += dx * amt as isize;
        self.y += dy * amt as isize;
    }

    fn move_waypoint(&mut self, dir: Pos, amt: usize) {
        let (dx, dy) = dir;

        self.waypoint.x_off += dx * amt as isize;
        self.waypoint.y_off += dy * amt as isize;
    }

    fn forward(&mut self, amt: usize) {
        let dx = self.waypoint.x_off;
        let dy = self.waypoint.y_off;

        self.x += dx * amt as isize;
        self.y += dy * amt as isize;
    }

    fn rotate_waypoint(&mut self, dir: Direction, amt: usize) {
        let amt = match dir {
            Direction::Left => amt,
            Direction::Right => 4 - amt,
            _ => unreachable!(),
        };

        for _ in 0..amt {
            let x = self.waypoint.x_off;

            self.waypoint.x_off = -self.waypoint.y_off;
            self.waypoint.y_off = x;
        }
    }

    pub fn step(&mut self, action: Action, part1: bool) {
        let Action { dir, amount: amt } = action;
        match dir {
            Direction::Left | Direction::Right => self.rotate_waypoint(dir, amt),
            Direction::Forward => self.forward(amt),
            _ if !part1 => self.move_waypoint(dir.try_into().unwrap(), amt),
            _ => self.move_ferry(dir.try_into().unwrap(), amt),
        };
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|l| {
            let (dir, amount) = l.split_at(1);
            let dir = Direction::from(dir.chars().nth(0).unwrap());
            let amount = amount.parse().unwrap();

            Action::new(dir, amount)
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn part1(input: &[Action]) -> usize {
    let mut ferry = Ferry::default();
    ferry.waypoint = WayPoint { x_off: 1, y_off: 0 };

    for action in input {
        ferry.step(*action, true);
    }

    ferry.x.abs() as usize + ferry.y.abs() as usize
}

#[aoc(day12, part2)]
pub fn part2(input: &[Action]) -> usize {
    let mut ferry = Ferry::default();

    for action in input {
        ferry.step(*action, false);
    }

    ferry.x.abs() as usize + ferry.y.abs() as usize
}
