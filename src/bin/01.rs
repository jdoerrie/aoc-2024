use hashbrown::HashMap;
use itertools::Itertools;

advent_of_code::solution!(1);

fn sum_diffs(vecs: (Vec<u32>, Vec<u32>)) -> u32 {
    vecs.0
        .into_iter()
        .sorted()
        .zip(vecs.1.into_iter().sorted())
        .map(|(x, y)| x.abs_diff(y))
        .sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(sum_diffs(
        input
            .lines()
            .flat_map(|line| {
                line.split_ascii_whitespace()
                    .flat_map(|s| s.parse::<u32>())
                    .collect_tuple()
            })
            .unzip(),
    ))
}

fn build_map(vec: Vec<u32>) -> HashMap<u32, u32> {
    vec.into_iter().fold(HashMap::new(), |mut acc, i| {
        *acc.entry(i).or_insert(0) += 1;
        acc
    })
}

fn sum_sims(vecs: (Vec<u32>, Vec<u32>)) -> u32 {
    let map = build_map(vecs.1);
    vecs.0
        .into_iter()
        .map(|i| i * map.get(&i).unwrap_or(&0))
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(sum_sims(
        input
            .lines()
            .flat_map(|line| {
                line.split_ascii_whitespace()
                    .flat_map(|s| s.parse::<u32>())
                    .collect_tuple()
            })
            .unzip(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
