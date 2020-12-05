use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Field {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid,
}

impl FromStr for Field {
    type Err = ();

    fn from_str(input: &str) -> Result<Field, Self::Err> {
        match input {
            "byr" => Ok(Field::Byr),
            "iyr" => Ok(Field::Iyr),
            "eyr" => Ok(Field::Eyr),
            "hgt" => Ok(Field::Hgt),
            "hcl" => Ok(Field::Hcl),
            "ecl" => Ok(Field::Ecl),
            "pid" => Ok(Field::Pid),
            "cid" => Ok(Field::Cid),
            _ => Err(()),
        }
    }
}

fn parse_passport(input: &str) -> HashMap<Field, String> {
    input
        .split_ascii_whitespace()
        .map(|entry| {
            let mut splitted = entry.split(':');
            let key = splitted.next().unwrap();
            let value = splitted.next().unwrap();

            (key, value)
        })
        .map(|(field_str, value)| (Field::from_str(field_str).unwrap(), value.to_owned()))
        .collect()
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<HashMap<Field, String>> {
    input.split("\n\n").map(parse_passport).collect()
}

fn is_valid_passport1(pass: &HashMap<Field, String>) -> bool {
    use Field::*;
    for field in [Byr, Iyr, Eyr, Hgt, Hcl, Ecl, Pid].iter() {
        if pass.get(field).is_none() {
            return false;
        }
    }

    true
}

#[aoc(day4, part1)]
pub fn part1(input: &[HashMap<Field, String>]) -> usize {
    input
        .iter()
        .fold(0, |acc, pass| acc + is_valid_passport1(pass) as usize)
}

fn validate_byr(byr: &str) -> bool {
    if let Ok(value) = byr.parse::<u32>() {
        value >= 1920 && value <= 2002
    } else {
        false
    }
}

fn validate_iyr(iyr: &str) -> bool {
    if let Ok(value) = iyr.parse::<u32>() {
        value >= 2010 && value <= 2020
    } else {
        false
    }
}

fn validate_eyr(eyr: &str) -> bool {
    if let Ok(value) = eyr.parse::<u32>() {
        value >= 2020 && value <= 2030
    } else {
        false
    }
}

fn validate_hgt(hgt: &str) -> bool {
    let (value, unit) = hgt.split_at(hgt.len() - 2);

    match value.parse::<u32>() {
        Ok(x) if unit == "cm" => x >= 150 && x <= 193,
        Ok(x) if unit == "in" => x >= 59 && x <= 76,
        _ => false,
    }
}

fn validate_hcl(hcl: &str) -> bool {
    if hcl.len() == 7 {
        hcl.chars().skip(1).fold(true, |acc, c| match c {
            '0'..='9' | 'a'..='f' => acc & true,
            _ => false,
        })
    } else {
        false
    }
}

fn validate_ecl(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn validate_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.parse::<u32>().is_ok()
}

fn validate_field(field: Field, val: &str) -> bool {
    use Field::*;
    match field {
        Byr => validate_byr(val),
        Iyr => validate_iyr(val),
        Eyr => validate_eyr(val),
        Hgt => validate_hgt(val),
        Hcl => validate_hcl(val),
        Ecl => validate_ecl(val),
        Pid => validate_pid(val),
        Cid => true,
    }
}

fn is_valid_passport2(pass: &HashMap<Field, String>) -> bool {
    use Field::*;
    for field in [Byr, Iyr, Eyr, Hgt, Hcl, Ecl, Pid].iter() {
        if let Some(value) = pass.get(field) {
            if !validate_field(*field, value) {
                println!("{:?}, field: {:?}, value: {}", pass, field, value);
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

#[aoc(day4, part2)]
pub fn part2(input: &[HashMap<Field, String>]) -> usize {
    input
        .iter()
        .fold(0, |acc, pass| acc + is_valid_passport2(pass) as usize)
}
