use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(7);

fn solve_one(goal: u64, curr: u64, nums: &[u64]) -> bool {
    if curr > goal {
        return false;
    }

    if let Some((num, rest)) = nums.split_first() {
        return solve_one(goal, curr + num, rest) || solve_one(goal, curr * num, rest);
    } else {
        return goal == curr;
    }
}

fn solve_two(goal: u64, curr: u64, nums: &[u64]) -> bool {
    if curr > goal {
        return false;
    }

    if let Some((num, rest)) = nums.split_first() {
        let mut rounded = 1;
        while rounded <= *num {
            rounded *= 10;
        }

        return solve_two(goal, curr + num, rest)
            || solve_two(goal, curr * num, rest)
            || solve_two(goal, curr * rounded + num, rest);
    } else {
        return goal == curr;
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .par_lines()
            .filter_map(|line| line.split_once(": "))
            .map(|(goal, nums)| {
                (
                    goal.parse::<u64>().unwrap(),
                    nums.split_ascii_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect_vec(),
                )
            })
            .filter_map(|(goal, nums)| {
                let (curr, nums) = nums.split_first().unwrap();
                if solve_one(goal, *curr, nums) {
                    Some(goal)
                } else {
                    None
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .filter_map(|line| line.split_once(": "))
            .map(|(goal, nums)| {
                (
                    goal.parse::<u64>().unwrap(),
                    nums.split_ascii_whitespace()
                        .map(|num| num.parse::<u64>().unwrap())
                        .collect_vec(),
                )
            })
            .filter_map(|(goal, nums)| {
                let (curr, nums) = nums.split_first().unwrap();
                if solve_two(goal, *curr, nums) {
                    Some(goal)
                } else {
                    None
                }
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
