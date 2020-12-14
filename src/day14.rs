use std::collections::HashMap;

pub enum Command {
    MemSet(usize, u64),
    MaskSet(Vec<Option<bool>>),
}

fn parse_input(input: &str) -> impl Iterator<Item = Command> + '_ {
    input.lines().map(|l| {
        let mut splitted = l.split(" = ");
        let command = splitted.next().unwrap();
        let value = splitted.next().unwrap();

        if command == "mask" {
            let value = value
                .chars()
                .map(|c| match c {
                    '0' => Some(false),
                    '1' => Some(true),
                    'X' => None,
                    _ => unreachable!(),
                })
                .collect();

            Command::MaskSet(value)
        } else {
            let address = command
                .split_at(4)
                .1
                .strip_suffix(']')
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let value = value.parse::<u64>().unwrap();

            Command::MemSet(address, value)
        }
    })
}

struct Bits {
    val: u64,
    i: usize,
}

impl Bits {
    pub fn new(val: u64) -> Self {
        Self { val, i: 36 }
    }
}

impl Iterator for Bits {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == 0 {
            None
        } else {
            let bit = (self.val >> (self.i - 1)) & 1 != 0;
            self.i -= 1;

            Some(bit)
        }
    }
}

trait ToBits {
    fn bits(self) -> Bits;
}

impl ToBits for u64 {
    fn bits(self) -> Bits {
        Bits::new(self)
    }
}

fn mask_value(mask: &[Option<bool>], value: u64) -> u64 {
    value
        .bits()
        .zip(mask)
        .map(|(v, m)| m.unwrap_or(v))
        .fold(0, |acc, b| acc * 2 + b as u64)
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u64 {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut mask: Vec<Option<bool>> = vec![];

    let commands = parse_input(input);
    for command in commands {
        match command {
            Command::MaskSet(m) => mask = m,
            Command::MemSet(address, value) => {
                mem.insert(address, mask_value(&mask, value));
            }
        }
    }

    mem.values().sum()
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> u64 {
    let mut mem: HashMap<usize, u64> = HashMap::new();
    let mut mask: Vec<Option<bool>> = vec![];
    let mut num_x: usize = 0;

    let commands = parse_input(input);
    for command in commands {
        match command {
            Command::MaskSet(m) => {
                mask = m;
                num_x = mask.iter().filter(|x| x.is_none()).count();
            }
            Command::MemSet(address, value) => {
                let n_bits = 2_usize.pow(num_x as u32);
                for i in 0..n_bits {
                    let mut floating_bits = (i as u64).bits().skip(36 - num_x);
                    let floating_address =
                        mask.iter()
                            .zip((address as u64).bits())
                            .map(|(m, a)| {
                                m.map_or_else(
                                    || floating_bits.next().unwrap(),
                                    |x| if !x { a } else { x },
                                )
                            })
                            .fold(0, |acc, b| acc * 2 + b as u64) as usize;

                    mem.insert(floating_address, value);
                }
            }
        }
    }

    mem.values().sum()
}
