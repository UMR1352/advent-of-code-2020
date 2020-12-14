pub struct Policy(usize, usize, char);

impl Policy {
    pub fn new(s: &str) -> Self {
        let mut policy = Policy(0, 0, ' ');

        let mut values = s.split(|c| c == '-' || c == ' ');

        policy.0 = values.next().unwrap().parse().unwrap();
        policy.1 = values.next().unwrap().parse().unwrap();
        policy.2 = values.next().unwrap().chars().nth(0).unwrap();

        policy
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Policy, String)> {
    input
        .lines()
        .map(|line| {
            let mut line_iter = line.split(':');
            let policy = line_iter.next().unwrap();
            let pwd = line_iter.next().unwrap().trim_start();
            let policy = Policy::new(policy);

            (policy, pwd.to_owned())
        })
        .collect()
}

fn respects_policy_part1(pol: &Policy, pwd: &str) -> bool {
    let Policy(lower_limit, upper_limit, c) = *pol;

    let count = pwd.chars().filter(|x| *x == c).count();

    count >= lower_limit && count <= upper_limit
}

#[aoc(day2, part1)]
pub fn part1(input: &[(Policy, String)]) -> usize {
    input
        .iter()
        .fold(0, |acc, x| acc + respects_policy_part1(&x.0, &x.1) as usize)
}

fn respects_policy_part2(pol: &Policy, pwd: &str) -> bool {
    let Policy(i, j, c) = *pol;

    (pwd.chars().nth(i - 1).unwrap() == c) ^ (pwd.chars().nth(j - 1).unwrap() == c)
}

#[aoc(day2, part2)]
pub fn part2(input: &[(Policy, String)]) -> usize {
    input
        .iter()
        .fold(0, |acc, x| acc + respects_policy_part2(&x.0, &x.1) as usize)
}

#[test]
pub fn test1() {
    let pol = Policy(1, 3, 'a');
    let pwd = "abcde";

    assert!(respects_policy_part2(&pol, &pwd));
}
#[test]
pub fn test2() {
    let pol = Policy(1, 3, 'b');
    let pwd = "cdefg";

    assert!(!respects_policy_part2(&pol, &pwd));
}
#[test]
pub fn test3() {
    let pol = Policy(2, 9, 'c');
    let pwd = "ccccccccc";

    assert!(!respects_policy_part2(&pol, &pwd));
}
