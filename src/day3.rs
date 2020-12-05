use std::ops::Index;

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Path,
    Tree,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Cell>,
}

impl Map {
    pub fn new(w: usize, h: usize, data: Vec<Cell>) -> Self {
        Self {
            width: w,
            height: h,
            data,
        }
    }

    pub fn map_index(&self, pos: Pos) -> usize {
        let (x, y) = pos;

        if y >= self.height {
            panic!("Woo, u should have stopped this earlier");
        } else {
            y * self.width + x % self.width
        }
    }
}

type Pos = (usize, usize);

impl Index<Pos> for Map {
    type Output = Cell;

    fn index(&self, index: Pos) -> &Self::Output {
        let i = self.map_index(index);

        &self.data[i]
    }
}

#[derive(Clone, Copy)]
pub struct Line {
    x: usize,
    y: usize,
    x_inc: usize,
    y_inc: usize,
}

impl Line {
    pub fn new(x_inc: usize, y_inc: usize) -> Self {
        Self {
            x: 0,
            y: 0,
            x_inc,
            y_inc,
        }
    }
}

impl Iterator for Line {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        self.x += self.x_inc;
        self.y += self.y_inc;

        Some((self.x, self.y))
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Map {
    let width = input.find('\n').unwrap();
    let mut height: usize = 0;
    let mut data: Vec<Cell> = vec![];

    for line in input.lines() {
        height += 1;

        let mut row: Vec<Cell> = line
            .chars()
            .map(|c| if c == '.' { Cell::Path } else { Cell::Tree })
            .collect();
        data.append(row.as_mut());
    }

    Map::new(width, height, data)
}

fn trees_on_path(map: &Map, path: &Line) -> usize {
    let mut trees: usize = 0;

    for (x, y) in path.into_iter() {
        if y >= map.height {
            break;
        }

        trees += match map[(x, y)] {
            Cell::Tree => 1,
            Cell::Path => 0,
        }
    }

    trees
}

#[aoc(day3, part1)]
pub fn part1(input: &Map) -> usize {
    let path = Line::new(3, 1);

    trees_on_path(input, &path)
}

#[aoc(day3, part2)]
pub fn part2(input: &Map) -> usize {
    let paths = &[
        Line::new(1, 1),
        Line::new(3, 1),
        Line::new(5, 1),
        Line::new(7, 1),
        Line::new(1, 2),
    ];

    paths
        .iter()
        .fold(1, |acc, path| acc * trees_on_path(input, path))
}
