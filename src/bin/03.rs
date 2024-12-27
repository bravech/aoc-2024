advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut res = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        res += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap();
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"don\'t|do|mul\(\d{0,3},\d{0,3}\)").unwrap();
    let re2 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut res = 0;
    let mut enab = true;
    for (m, []) in re.captures_iter(input).map(|c| c.extract()) {
        if m == "do" {
            enab = true;
        } else if m == "don't" {
            enab = false;
        } else {
            if !enab {
                continue;
            }
            for (_, [a, b]) in re2.captures_iter(m).map(|c| c.extract()) {
                res += a.parse::<u64>().unwrap() * b.parse::<u64>().unwrap();
            }
        }
    }

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
