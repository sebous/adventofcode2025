use itertools::Itertools;

advent_of_code::solution!(2);

fn parse(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|range| {
            range
                .split_once("-")
                .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
                .unwrap()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse(input);
    let mut total = 0;
    for (min, max) in ranges {
        for id_num in min..=max {
            let id_str = id_num.to_string();
            if id_str.len() % 2 != 0 {
                continue;
            }
            let (first, second) = id_str.split_at(id_str.len() / 2);
            if first == second {
                total += id_num;
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse(input);
    let mut total = 0;
    for (min, max) in ranges {
        dbg!(max - min);
        for id_num in min..=max {
            let id_str = id_num.to_string();
            'chunk_iter: for chunk_size in 1..=id_str.len() / 2 {
                let valid_id = id_str
                    .chars()
                    .chunks(chunk_size)
                    .into_iter()
                    .map(|chars| chars.collect::<String>())
                    .all_equal();
                if valid_id {
                    total += id_num;
                    break 'chunk_iter;
                }
            }
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
