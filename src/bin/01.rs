advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut current: i32 = 50;
    let mut zero_counter: u32 = 0;

    for item in input.split("\n") {
        let first = &item[0..1];
        let value: &i32 = &item[1..].parse().unwrap();
        match first {
            "L" => current -= value,
            "R" => current += value,
            _ => panic!("impossible"),
        }
        current = current.rem_euclid(100);
        if current == 0 {
            zero_counter += 1;
        }
    }
    Some(zero_counter)
}

fn calc_zero_passes(prev: i32, next: i32) -> u32 {
    if next == prev {
        return 0;
    }
    if next > prev {
        let a = next.div_euclid(100);
        let b = prev.div_euclid(100);
        return (a - b) as u32;
    } else {
        let a = (prev - 1).div_euclid(100);
        let b = (next - 1).div_euclid(100);
        return (a - b) as u32;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut prev: i32 = 50;
    let mut zero_counter: u32 = 0;

    for item in input.split("\n") {
        let first = &item[0..1];
        let value: &i32 = &item[1..].parse().unwrap();
        let next = match first {
            "L" => prev - value,
            "R" => prev + value,
            _ => panic!("impossible"),
        };
        zero_counter += calc_zero_passes(prev, next);
        prev = next;
    }
    Some(zero_counter)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_calc_zero_passes() {
        assert_eq!(calc_zero_passes(0, 150), 1);
        assert_eq!(calc_zero_passes(0, 200), 2);
        assert_eq!(calc_zero_passes(200, 250), 0);
        assert_eq!(calc_zero_passes(200, 300), 1);
        assert_eq!(calc_zero_passes(200, 350), 1);
        assert_eq!(calc_zero_passes(200, 400), 2);
        assert_eq!(calc_zero_passes(400, 200), 2);
    }
}
