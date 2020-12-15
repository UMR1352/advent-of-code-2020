use std::collections::HashMap;

struct MemoryGame {
    spoken_nums: HashMap<u64, usize>,
    last_num: u64,
    turn: usize,
}

impl MemoryGame {
    pub fn new(input: &[u64]) -> Self {
        let mut spoken_nums: HashMap<u64, usize> = HashMap::new();
        let mut last_num = 0;
        input.iter().enumerate().for_each(|(i, n)| {
            spoken_nums.insert(*n, i + 1);
            last_num = *n;
        });

        Self {
            spoken_nums,
            last_num,
            turn: input.len() + 1,
        }
    }
}

impl Iterator for MemoryGame {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let spoken_on_turn = self.spoken_nums.insert(self.last_num, self.turn - 1);

        match spoken_on_turn {
            None => self.last_num = 0,
            Some(t) => self.last_num = (self.turn - 1 - t) as u64,
        }

        self.turn += 1;

        Some(self.last_num)
    }

    fn nth(&mut self, i: usize) -> Option<Self::Item> {
        let mut res = None;

        while self.turn <= i {
            res = self.next();
        }

        res
    }
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.split(',').map(|n| n.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &Vec<u64>) -> u64 {
    let mut seq = MemoryGame::new(&input);

    seq.nth(2020).unwrap()
}

#[aoc(day15, part2)]
pub fn part2(input: &Vec<u64>) -> u64 {
    let mut seq = MemoryGame::new(&input);

    seq.nth(30_000_000).unwrap()
}
