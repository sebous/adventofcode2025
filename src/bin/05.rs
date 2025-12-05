use std::collections::HashSet;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut ranges = vec![];
    let mut ranges_ended = false;
    let mut result = 0;
    for line in input.lines() {
        if line == "" {
            ranges_ended = true;
        } else if !ranges_ended {
            let (a, b) = line.split_once("-").unwrap();
            ranges.push((a.parse().unwrap(), b.parse().unwrap()));
        } else {
            let num: u64 = line.parse().unwrap();
            for (a, b) in &ranges {
                if num >= *a && num <= *b {
                    result += 1;
                    break;
                }
            }
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut result = 0;
    for line in input.lines() {
        if line == "" {
            break;
        } else {
            let (a, b) = line.split_once("-").unwrap();
            ranges.push((a.parse().unwrap(), b.parse().unwrap()));
        }
    }
    ranges.sort();

    let mut skip_indexes = HashSet::new();
    for i in 0..ranges.len() {
        if skip_indexes.get(&i).is_some() {
            continue;
        }
        let (a1, b1) = ranges[i];
        let mut end = b1;
        for j in (i + 1)..ranges.len() {
            let (a2, b2) = ranges[j];
            if a2 <= end {
                end = end.max(b2);
                skip_indexes.insert(j);
            }
        }
        result += end - a1 + 1;
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
