use std::cmp::Ordering;

use hashbrown::HashSet;
use itertools::Itertools;
use rayon::prelude::*;
use tuple::Map;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (order, pages) = input.split_once("\n\n").unwrap();
    let graph = order
        .lines()
        .map(|line| {
            line.split_once('|')
                .unwrap()
                .map(|v| v.parse::<u32>().unwrap())
        })
        .collect::<HashSet<_>>();
    Some(
        pages
            .par_lines()
            .map(|page| page.split(',').flat_map(|u| u.parse::<u32>()).collect_vec())
            .filter(|nums| nums.is_sorted_by(|u, v| graph.contains(&(*u, *v))))
            .map(|v| v[v.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (order, pages) = input.split_once("\n\n").unwrap();
    let graph = order
        .lines()
        .map(|line| {
            line.split_once('|')
                .unwrap()
                .map(|v| v.parse::<u32>().unwrap())
        })
        .collect::<HashSet<_>>();
    Some(
        pages
            .par_lines()
            .map(|page| page.split(',').flat_map(|u| u.parse::<u32>()).collect_vec())
            .filter(|nums| !nums.is_sorted_by(|u, v| graph.contains(&(*u, *v))))
            .map(|mut v| {
                let index = v.len() / 2;
                *v.select_nth_unstable_by(index, |u, v| {
                    if graph.contains(&(*u, *v)) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .1
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
