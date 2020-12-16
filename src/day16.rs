use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref RE_FIELD: Regex = Regex::new(r"(\w+ ?\w*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

pub type NonContinuousRange = (u64, u64, u64, u64);

fn parse_ticket(ticket: &str) -> Vec<u64> {
    ticket
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

#[aoc_generator(day16)]
pub fn input_generator(
    input: &str,
) -> (HashMap<String, NonContinuousRange>, Vec<u64>, Vec<Vec<u64>>) {
    let mut paragraphs = input.split("\n\n");
    // parse ticket fields
    let mut fields: HashMap<String, NonContinuousRange> = HashMap::new();
    for capture in RE_FIELD.captures_iter(paragraphs.next().unwrap()) {
        fields.insert(
            capture[1].to_owned(),
            (
                capture[2].parse().unwrap(),
                capture[3].parse().unwrap(),
                capture[4].parse().unwrap(),
                capture[5].parse().unwrap(),
            ),
        );
    }
    // parse my ticket
    let my_ticket = parse_ticket(paragraphs.next().unwrap().lines().skip(1).next().unwrap());

    //parse other tickets
    let other_tickets = paragraphs
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|t| parse_ticket(t))
        .collect();

    (fields, my_ticket, other_tickets)
}

fn validate_field(field: u64, (a, b, c, d): (u64, u64, u64, u64)) -> bool {
    (a..=b).contains(&field) || (c..=d).contains(&field)
}
fn validate_ticket(
    ticket: &[u64],
    constraints: &HashMap<String, NonContinuousRange>,
) -> Option<u64> {
    ticket
        .iter()
        .find(|x| !constraints.values().any(|c| validate_field(**x, *c)))
        .cloned()
}

#[aoc(day16, part1)]
pub fn part1(
    (fields, _, tickets): &(HashMap<String, NonContinuousRange>, Vec<u64>, Vec<Vec<u64>>),
) -> u64 {
    tickets.iter().fold(0, |cnt, ticket| {
        if let Some(x) = validate_ticket(ticket, fields) {
            cnt + x
        } else {
            cnt
        }
    })
}

#[aoc(day16, part2)]
pub fn part2(
    (fields, my_ticket, tickets): &(HashMap<String, NonContinuousRange>, Vec<u64>, Vec<Vec<u64>>),
) -> u64 {
    let tickets: Vec<&Vec<u64>> = tickets
        .into_iter()
        .filter(|t| validate_ticket(t, fields).is_none())
        .collect();
    let mut valid_col_field: Vec<HashSet<&str>> = vec![HashSet::new(); my_ticket.len()];

    for i in 0..my_ticket.len() {
        fields
            .iter()
            .filter(|(_, c)| tickets.iter().map(|t| t[i]).all(|f| validate_field(f, **c)))
            .for_each(|(f, _)| {
                valid_col_field[i].insert(f);
            })
    }

    let mut field_pos: HashMap<&str, usize> = HashMap::new();

    loop {
        if field_pos.len() == my_ticket.len() {
            break;
        }

        valid_col_field
            .iter()
            .enumerate()
            .filter(|(_, f)| f.len() == 1)
            .for_each(|(i, f)| {
                field_pos.insert(f.iter().next().unwrap(), i);
            });
        for i in 0..valid_col_field.len() {
            for field in field_pos.keys() {
                valid_col_field[i].remove(field);
            }
        }
    }

    field_pos
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .fold(1, |acc, (_, i)| acc * my_ticket[*i])
}
