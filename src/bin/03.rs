use itertools::{Itertools, Permutations};

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
    // dbg!(row.iter().combinations(2).unique().collect_vec());
    let row_max = row
        .iter()
        .combinations(size)
        .unique()
        .sorted()
        .last()
        .map(|group| {
            group
                .iter()
                .fold(String::new(), |mut acc, item| {
                    acc.push_str(&item.to_string());
                    acc
                })
                .parse::<u64>()
                .unwrap()
        })
        .unwrap();
    dbg!(row_max);
    row_max
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
