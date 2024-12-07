use hashbrown::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(4);

fn matches(grid: &[Vec<char>], x: isize, y: isize, dx: isize, dy: isize, needle: &[char]) -> bool {
    (0..needle.len()).all(|i| {
        let x = x + i as isize * dx;
        let y = y + i as isize * dy;
        if let Some(row) = grid.get(x as usize) {
            row.get(y as usize) == Some(&needle[i])
        } else {
            false
        }
    })
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let n = grid.len();
    let m = grid[0].len();
    let dirs = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let work: Vec<_> = (0..n)
        .cartesian_product(0..m)
        .cartesian_product(dirs)
        .collect();
    let needle = "XMAS".chars().collect_vec();
    Some(
        work.par_iter()
            .filter(|((x, y), (dx, dy))| {
                matches(&grid, *x as isize, *y as isize, *dx, *dy, &needle)
            })
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let n = grid.len();
    let m = grid[0].len();
    let dirs = [(1, 1), (1, -1), (-1, 1), (-1, -1)];

    let work: Vec<_> = (0..n)
        .cartesian_product(0..m)
        .cartesian_product(dirs)
        .collect();

    let needle = "MAS".chars().collect_vec();
    let mases: HashSet<_> = work
        .par_iter()
        .filter(|((x, y), (dx, dy))| matches(&grid, *x as isize, *y as isize, *dx, *dy, &needle))
        .collect();
    Some(
        mases
            .par_iter()
            .filter(|x| match x {
                ((x, y), (1, -1)) => {
                    mases.contains(&((x + 2, *y), (-1, -1)))
                        || mases.contains(&((*x, y - 2), (1, 1)))
                }
                ((x, y), (1, 1)) => {
                    mases.contains(&((x + 2, *y), (-1, 1)))
                        || mases.contains(&((*x, y + 2), (1, -1)))
                }
                ((x, y), (-1, -1)) => {
                    mases.contains(&((x - 2, *y), (1, -1)))
                        || mases.contains(&((*x, y - 2), (-1, 1)))
                }
                ((x, y), (-1, 1)) => {
                    mases.contains(&((x - 2, *y), (1, 1)))
                        || mases.contains(&((*x, y + 2), (-1, -1)))
                }
                _ => false,
            })
            .count()
            / 2,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
