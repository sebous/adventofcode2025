use itertools::Itertools;

advent_of_code::solution!(6);

fn parse(input: &str) -> (Vec<Vec<&str>>, usize, usize) {
    let lines = input
        .split("\n")
        .map(|row| row.split_whitespace().collect_vec())
        .collect_vec()
        .iter()
        .rev()
        .cloned()
        .collect_vec();
    let col_size = lines.iter().count();
    let row_size = input
        .split("\n")
        .into_iter()
        .next()
        .unwrap()
        .split_whitespace()
        .count();

    (lines, row_size, col_size)
}

enum Op {
    Add,
    Multiply,
}

pub fn part_one(input: &str) -> Option<u64> {
    let (data, row_size, col_size) = parse(input);
    let mut total = 0;

    for x in 0..row_size {
        let mut row_total = 0;
        let mut op = None;
        for y in 0..col_size {
            if y == 0 {
                op = Some(match data[y][x] {
                    "*" => Op::Multiply,
                    "+" => Op::Add,
                    _ => unreachable!(),
                });
                continue;
            }

            let val = data[y][x].parse::<u64>().unwrap();

            if y == 1 {
                row_total = val;
                continue;
            }

            match op {
                Some(Op::Add) => {
                    row_total += val;
                }
                Some(Op::Multiply) => {
                    row_total *= val;
                }
                _ => unreachable!(),
            }
        }
        total += row_total;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data = input
        .lines()
        .map(|ln| ln.chars().collect_vec())
        .collect_vec();
    let col_size = data.len();
    let row_size = data[0].len();
    let mut total = 0;

    let mut stack = vec![];
    for x in (0..row_size).rev() {
        let mut number_stack = String::new();
        for y in 0..col_size - 1 {
            if data[y][x].is_digit(10) {
                number_stack.push(data[y][x]);
            }
        }
        if number_stack.len() > 0 {
            stack.push(number_stack.parse::<u64>().unwrap());
        }

        match data[col_size - 1][x] {
            '*' => {
                total += stack.iter().fold(1, |acc, num| acc * num);
                stack.clear();
            }
            '+' => {
                total += stack.iter().fold(0, |acc, num| acc + num);
                stack.clear();
            }
            _ => continue,
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
