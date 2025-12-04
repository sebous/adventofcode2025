advent_of_code::solution!(3);

fn parse(input: &str) -> Vec<Vec<u64>> {
    let mut data = vec![];
    for line in input.split("\n") {
        let mut row = vec![];
        for x in line.chars().map(|ch| ch.to_digit(10).unwrap()) {
            row.push(x as u64);
        }
        data.push(row);
    }
    data
}

fn find_next_highest(row: &Vec<u64>, size: usize) -> u64 {
    let mut combination = vec![];
    let mut last_index = 0;
    for _ in 0..size {
        let mut curr_max = 0;
        let comb_len = combination.len();
        row.iter()
            .enumerate()
            .skip(if comb_len > 0 { last_index + 1 } else { 0 })
            .take_while(|(i, _)| *i <= (row.len() - size + comb_len))
            .for_each(|(i, digit)| {
                if *digit > curr_max {
                    curr_max = *digit;
                    last_index = i;
                }
            });
        combination.push(curr_max);
    }
    combination
        .iter()
        .fold(String::new(), |mut acc, digit| {
            acc.push_str(&digit.to_string());
            acc
        })
        .parse()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .iter()
            .fold(0, |acc, row| acc + find_next_highest(&row, 2)),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        parse(input)
            .iter()
            .fold(0, |acc, row| acc + find_next_highest(&row, 12)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
