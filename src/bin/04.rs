use std::collections::HashMap;

use advent_of_code::grid::Grid;
use itertools::Itertools;

advent_of_code::solution!(4);

fn parse(input: &str) -> Grid<bool> {
    let mut map = HashMap::new();
    let mut width = 0;
    let height = input.split("\n").collect_vec().len();
    for (y, line) in input.split("\n").enumerate() {
        for (x, pt) in line.chars().enumerate() {
            match pt {
                '@' => {
                    map.insert((x, y), true);
                }
                _ => continue,
            }
        }
        if y == 0 {
            width = line.len();
        }
    }
    Grid { map, width, height }
}

fn removal_pass(grid: &mut Grid<bool>) -> u64 {
    let mut total = 0;
    let mut removed = vec![];
    for coord in grid
        .iter_coords()
        .filter(|c| grid.map.get(&c).is_some_and(|x| *x))
    {
        if grid
            .get_adjacent_coords(&coord, true)
            .filter_map(|c| c.and_then(|c| grid.map.get(&c).and_then(|x| x.then(|| x))))
            .count()
            < 4
        {
            total += 1;
            removed.push(coord);
        }
    }

    for coord in removed {
        grid.map.insert(coord, false);
    }
    total
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = parse(&input);
    Some(removal_pass(&mut grid))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse(input);
    let mut total = 0;
    loop {
        match removal_pass(&mut grid) {
            0 => break,
            x => total += x,
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
