advent_of_code::solution!(2);
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines();
    let mut ret = 0;
    let incr: HashSet<i32> = vec![1,2,3].into_iter().collect();
    let decr: HashSet<i32> = vec![-1,-2,-3].into_iter().collect();

    for line in lines {
        let nums: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        let mut diffs: Vec<i32> = Vec::new();
        for i in 0..nums.len()-1 {
            diffs.push(nums[i] - nums[i+1]);
        }
        let incr_bool = diffs.iter().all(|x| incr.contains(&x));
        let decr_bool = diffs.iter().all(|x| decr.contains(&x));
        if incr_bool ^ decr_bool {
            ret += 1;
        }
    }
    Some(ret)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut ret = 0;
    let incr: HashSet<i32> = vec![1,2,3].into_iter().collect();
    let decr: HashSet<i32> = vec![-1,-2,-3].into_iter().collect();

    for line in lines {
        let mut safe = false;
        let orig_nums: Vec<i32> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        for j in 0..orig_nums.len() {
            let orig_iter = orig_nums.iter().enumerate().filter(|(index, _)| *index != j);
            let nums: Vec<&i32> = orig_iter.map(|(_,b)| b).collect();
            let mut diffs: Vec<i32> = Vec::new();
            for i in 0..nums.len()-1 {
                diffs.push(nums[i] - nums[i+1]);
            }

            let incr_bool = diffs.iter().all(|x| incr.contains(&x));
            let decr_bool = diffs.iter().all(|x| decr.contains(&x));
            if incr_bool ^ decr_bool {
                safe = true;
            }
        }
        if safe {
            ret += 1;
        }
    }
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
