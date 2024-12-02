use itertools::Itertools;
use num_traits::signum;

advent_of_code::solution!(2);

fn is_valid(nums: &[i64]) -> bool {
    nums.iter()
        .tuple_windows()
        .map(|(x, y)| x - y)
        .fold(None, |acc, e| {
            if !(1 <= e.abs() && e.abs() <= 3) {
                Some(0)
            } else {
                match acc {
                    None => Some(signum(e)),
                    Some(x) => {
                        if signum(x) == signum(e) {
                            Some(signum(e))
                        } else {
                            Some(0)
                        }
                    }
                }
            }
        })
        .is_some_and(|x| x != 0)
}

fn is_valid_two(nums: &[i64]) -> bool {
    is_valid(nums) || (0..nums.len()).any(|i| is_valid(&[&nums[..i], &nums[i + 1..]].concat()))
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| {
                is_valid(
                    &line
                        .split_ascii_whitespace()
                        .flat_map(|n| n.parse::<i64>())
                        .collect_vec(),
                )
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .filter(|line| {
                is_valid_two(
                    &line
                        .split_ascii_whitespace()
                        .flat_map(|n| n.parse::<i64>())
                        .collect_vec(),
                )
            })
            .count(),
    )
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
