use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref EXTRACT_BAGS: Regex = Regex::new(r"(\d?\s?\w{3,} \w+) bag").unwrap();
}

fn parse_rule(rule: &str) -> (String, Vec<(String, u32)>) {
    let mut matches = EXTRACT_BAGS.captures_iter(rule);
    let outer_bag = matches.next().unwrap()[1].trim_start().to_owned();

    let inner_bags = matches
        .map(|group| {
            let (amnt, mut bag) = group[1].split_at(1);
            let amnt = amnt.parse::<u32>().unwrap();
            bag = bag.trim_start();

            (bag.to_owned(), amnt)
        })
        .collect::<Vec<(String, u32)>>();

    (outer_bag, inner_bags)
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> HashMap<String, Vec<(String, u32)>> {
    input.lines().map(parse_rule).collect()
}

fn contains_shiny(
    input: &HashMap<String, Vec<(String, u32)>>,
    memo: &HashSet<&str>,
    bag: &str,
) -> bool {
    if bag == "shiny gold" || memo.contains(bag) {
        true
    } else {
        match input.get(bag) {
            None => false,
            Some(bags) => {
                for (bag, _) in bags.iter() {
                    if contains_shiny(input, memo, &bag) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &HashMap<String, Vec<(String, u32)>>) -> usize {
    let mut shiny_gold_containers: HashSet<&str> = HashSet::new();

    input.iter().fold(0, |acc, (key, values)| {
        for (bag, _) in values {
            if shiny_gold_containers.contains(bag.as_str()) || bag == "shiny gold" {
                shiny_gold_containers.insert(key);
                return acc + 1;
            } else {
                if contains_shiny(input, &shiny_gold_containers, bag) {
                    shiny_gold_containers.insert(bag);
                    shiny_gold_containers.insert(key);

                    return acc + 1;
                }
            }
        }

        acc
    })
}

pub fn count_inner_bags(input: &HashMap<String, Vec<(String, u32)>>, bag: &str) -> usize {
    match input.get(bag) {
        None => 0,
        Some(bags) => {
            if bags.is_empty() {
                return 0;
            } else {
                bags.iter().fold(0, |acc, (bag, amount)| {
                    let amount = *amount as usize;
                    acc + amount + amount * count_inner_bags(input, bag)
                })
            }
        }
    }
}

#[aoc(day7, part2)]
pub fn part2(input: &HashMap<String, Vec<(String, u32)>>) -> usize {
    count_inner_bags(input, "shiny gold")
}

#[test]
pub fn test_example() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    assert_eq!(part1(&input_generator(&input)), 4);
}

#[test]
pub fn test_2_example() {
    let input = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    assert_eq!(part2(&input_generator(&input)), 126);
}
