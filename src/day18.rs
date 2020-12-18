fn solve_simple_expr(expr: &str, with_precedence: bool) -> u64 {
    let operators = expr
        .chars()
        .filter(|c| *c == '+' || *c == '*')
        .collect::<Vec<char>>();
    let mut operands = expr
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    if !with_precedence {
        let first_operand = operands[0];
        operators
            .into_iter()
            .zip(operands[1..].into_iter())
            .fold(first_operand, |acc, (op, x)| match op {
                '+' => acc + x,
                '*' => acc * x,
                _ => unreachable!(),
            })
    } else {
        operators
            .into_iter()
            .enumerate()
            .rev()
            .filter(|(_, c)| *c == '+')
            .for_each(|(i, _)| {
                let res = operands[i] + operands[i + 1];
                operands[i] = res;
                operands.swap_remove(i + 1);
            });

        operands.into_iter().product()
    }
}

fn remove_sub_expr(expr: &String, precedence: bool) -> String {
    let mut expr = expr.clone();
    let mut parentheses = Vec::<usize>::new();

    let mut i = 0;
    loop {
        if i >= expr.len() {
            break;
        }

        match expr[i..].chars().next().unwrap() {
            '(' => parentheses.push(i),
            ')' => {
                let left = parentheses.pop().unwrap();
                let res = solve_simple_expr(&expr[left + 1..i], precedence);

                expr.replace_range(left..=i, &res.to_string());
                i = left - 1;
            }
            _ => (),
        }
        i += 1;
    }

    expr
}

fn solve(expr: &String, precedence: bool) -> u64 {
    solve_simple_expr(remove_sub_expr(expr, precedence).as_str(), precedence)
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(str::to_owned).collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[String]) -> u64 {
    input.into_iter().map(|e| solve(e, false)).sum()
}

#[aoc(day18, part2)]
pub fn part2(input: &[String]) -> u64 {
    input.into_iter().map(|e| solve(e, true)).sum()
}
