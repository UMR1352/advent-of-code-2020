use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|member| member.to_owned())
                .collect::<Vec<String>>()
        })
        .collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|member| member.chars())
                .flatten()
                .collect::<HashSet<char>>()
                .len()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &[Vec<String>]) -> usize {
    input
        .iter()
        .map(|group| {
            let mut answers = group
                .iter()
                .map(|member| member.chars().collect::<HashSet<char>>());

            let first_answer = answers.next().unwrap();
            answers
                .fold(first_answer, |acc, x| {
                    acc.intersection(&x).cloned().collect()
                })
                .len()
        })
        .sum()
}
