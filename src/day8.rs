use std::collections::HashSet;

enum Instruction {
    Nop(i16),
    Jmp(i16),
    Acc(i16),
}

enum PcResult {
    Next,
    Relative(i16),
}

enum ExecutionState {
    Running,
    Terminated,
}

struct Console {
    pub pc: usize,
    pub acc: i64,
    pub tape: Vec<Instruction>,
}

impl Console {
    fn parse_tape(input: &str) -> Vec<Instruction> {
        input
            .lines()
            .map(|line| {
                let (opcode, arg) = line.split_at(3);
                let arg = arg.trim_start().parse::<i16>().unwrap();

                match opcode {
                    "nop" => Instruction::Nop(arg),
                    "jmp" => Instruction::Jmp(arg),
                    "acc" => Instruction::Acc(arg),
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    pub fn new(input: &str) -> Self {
        Self {
            pc: 0,
            acc: 0,
            tape: Self::parse_tape(input),
        }
    }

    pub fn step(&mut self) -> ExecutionState {
        let result = match self.tape[self.pc] {
            Instruction::Nop(_) => PcResult::Next,
            Instruction::Acc(arg) => {
                self.acc += arg as i64;
                PcResult::Next
            }
            Instruction::Jmp(arg) => PcResult::Relative(arg),
        };

        match result {
            PcResult::Next => self.pc += 1,
            PcResult::Relative(offset) => self.pc = (self.pc as i16 + offset) as usize,
        }

        if self.pc >= self.tape.len() {
            ExecutionState::Terminated
        } else {
            ExecutionState::Running
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> i64 {
    let mut visited_addresses: HashSet<usize> = HashSet::new();
    visited_addresses.insert(0);

    let mut console = Console::new(input);

    while let ExecutionState::Running = console.step() {
        if visited_addresses.contains(&console.pc) {
            break;
        }

        visited_addresses.insert(console.pc);
    }

    console.acc
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> i64 {
    let mut console = Console::new(input);
    let mut trace = vec![false; console.tape.len()];

    trace[0] = true;
    while let ExecutionState::Running = console.step() {
        if trace[console.pc] {
            break;
        }

        trace[console.pc] = true;
    }

    let mut landing_spots = vec![false; trace.len() + 1];
    // Every addr after the last negative jump is a valid spot to land
    let mut i = console.tape.len();
    loop {
        landing_spots[i] = true;
        i -= 1;

        if matches!(console.tape[i], Instruction::Jmp(x) if x < 0) {
            break;
        }
    }

    let start = i;
    // If this last jump is visited than removing it solves the issue
    let swap_adr = if trace[i] {
        i
    } else {
        loop {
            i -= 1;

            if landing_spots[i] {
                continue;
            } else if let Instruction::Nop(x) = console.tape[i] {
                if trace[i] && landing_spots[i + x as usize] {
                    break i;
                }
            } else if let Instruction::Jmp(x) = console.tape[i] {
                if !trace[i] && landing_spots[i + x as usize] && !landing_spots[i] {
                    let mut j = i - 1;
                    loop {
                        if matches!(console.tape[j], Instruction::Jmp(_)) {
                            break;
                        }
                        j -= 1;
                    }

                    if trace[j] {
                        break j;
                    } else {
                        landing_spots[j + 1..=i].iter_mut().for_each(|a| *a = true);
                        i = start;
                    }
                }
            }
        }
    };

    console.tape[swap_adr] = match console.tape[swap_adr] {
        Instruction::Acc(_) => unreachable!(),
        Instruction::Jmp(x) => Instruction::Nop(x),
        Instruction::Nop(x) => Instruction::Jmp(x),
    };

    console.acc = 0;
    console.pc = 0;
    loop {
        if matches!(console.step(), ExecutionState::Terminated) {
            break;
        }
    }

    console.acc
}
