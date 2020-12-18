use std::collections::HashSet;

pub type Coordinate = (i32, i32, i32, i32);

struct Map {
    state: HashSet<Coordinate>,
}

impl Map {
    pub fn new(state: HashSet<Coordinate>) -> Self {
        Self { state }
    }

    fn step3d(&mut self) {
        let mut new_map = HashSet::<Coordinate>::new();

        let possible_affected_coords: HashSet<Coordinate> =
            self.state.iter().flat_map(|c| cube_at(*c)).collect();

        for c in possible_affected_coords.iter() {
            if self.state.contains(c) {
                let active_neighbours = cube_at(*c)
                    .filter(|n| self.state.contains(n) && n != c)
                    .count();
                if active_neighbours == 2 || active_neighbours == 3 {
                    new_map.insert(*c);
                }
            } else {
                let active_neighbours = cube_at(*c)
                    .filter(|n| self.state.contains(n) && n != c)
                    .count();
                if active_neighbours == 3 {
                    new_map.insert(*c);
                }
            }
        }

        self.state = new_map;
    }

    fn step4d(&mut self) {
        let mut new_map = HashSet::<Coordinate>::new();

        let possible_affected_coords: HashSet<Coordinate> =
            self.state.iter().flat_map(|c| hypercube_at(*c)).collect();

        for c in possible_affected_coords.iter() {
            if self.state.contains(c) {
                let active_neighbours = hypercube_at(*c)
                    .filter(|n| self.state.contains(n) && n != c)
                    .count();
                if active_neighbours == 2 || active_neighbours == 3 {
                    new_map.insert(*c);
                }
            } else {
                let active_neighbours = hypercube_at(*c)
                    .filter(|n| self.state.contains(n) && n != c)
                    .count();
                if active_neighbours == 3 {
                    new_map.insert(*c);
                }
            }
        }

        self.state = new_map;
    }

    pub fn step(&mut self, dim: usize) {
        match dim {
            3 => self.step3d(),
            4 => self.step4d(),
            _ => unreachable!(),
        }
    }

    pub fn count(&self) -> usize {
        self.state.len()
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> HashSet<Coordinate> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, val)| {
                if val == '#' {
                    Some((x as i32, y as i32, 0, 0))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn square_layer_at((x, y, z, w): Coordinate) -> impl Iterator<Item = Coordinate> {
    (x - 1..=x + 1).flat_map(move |x| (y - 1..=y + 1).map(move |y| (x, y, z, w)))
}

fn cube_at((x, y, z, w): Coordinate) -> impl Iterator<Item = Coordinate> {
    (z - 1..=z + 1).flat_map(move |z| square_layer_at((x, y, z, w)))
}

fn hypercube_at((x, y, z, w): Coordinate) -> impl Iterator<Item = Coordinate> {
    (w - 1..=w + 1).flat_map(move |w| cube_at((x, y, z, w)))
}

#[aoc(day17, part1)]
pub fn part1(map: &HashSet<Coordinate>) -> usize {
    let mut map = Map::new(map.clone());

    for _ in 0..6 {
        map.step(3);
    }

    map.count()
}

#[aoc(day17, part2)]
pub fn part2(map: &HashSet<Coordinate>) -> usize {
    let mut map = Map::new(map.clone());

    for _ in 0..6 {
        map.step(4);
    }

    map.count()
}
