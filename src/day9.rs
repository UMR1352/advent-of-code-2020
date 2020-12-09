use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &Vec<u64>) -> u64 {
    let mut prev_nums: HashSet<u64> = HashSet::new();
    input[0..25].iter().for_each(|n| {
        prev_nums.insert(*n);
    });

    let mut i = 25;
    loop {
        let x = input[i];
        // Is x = y + z where y and z are in `prev_nums`
        let z = input[i - 25..i].iter().find(|y| {
            let z = x - **y;
            prev_nums.contains(&z)
        });

        match z {
            Some(_) => {
                prev_nums.insert(x);
                prev_nums.remove(&input[i - 25]);
                i += 1;
            }
            None => break x,
        }
    }
}

#[aoc(day9, part2)]
pub fn part2(input: &Vec<u64>) -> u64 {
    let x = part1(input);

    let mut left = 0;

    loop {
        let mut sum = input[left];
        let mut right = left;

        while sum < x {
            right += 1;
            sum += input[right];
        }

        if sum == x {
            let mut window = vec![0; right - left + 1];
            window.copy_from_slice(&input[left..=right]);
            window.sort();

            break window.first().unwrap() + window.last().unwrap();
        }

        left += 1;
    }
}
