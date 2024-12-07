use std::collections::HashSet;

use itertools::Itertools;
use rayon::prelude::*;

advent_of_code::solution!(6);

fn guard_pos(
    (mut x, mut y): (isize, isize),
    (max_x, max_y): (isize, isize),
    trees: &HashSet<(isize, isize)>,
) -> HashSet<(isize, isize)> {
    let mut all_pos = HashSet::new();
    let (mut dx, mut dy) = (-1, 0);
    loop {
        all_pos.insert((x, y));
        x += dx;
        y += dy;
        if trees.contains(&(x, y)) {
            x -= dx;
            y -= dy;
            (dx, dy) = match (dx, dy) {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            };
        }

        if x < 0 || x > max_x || y < 0 || y > max_y {
            return all_pos;
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut pos = (0, 0);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut trees = HashSet::new();

    for (x, line) in input.lines().enumerate() {
        max_x = max_x.max(x as isize);
        for (y, c) in line.char_indices() {
            max_y = max_y.max(y as isize);
            match c {
                '^' => pos = (x as isize, y as isize),
                '#' => {
                    trees.insert((x as isize, y as isize));
                }
                _ => {}
            }
        }
    }

    Some(guard_pos(pos, (max_x, max_y), &trees).len())
}

fn is_infinite_loop(
    (mut x, mut y): (isize, isize),
    (max_x, max_y): (isize, isize),
    trees: &HashSet<(isize, isize)>,
    obs: (isize, isize),
) -> bool {
    let mut all_pos = HashSet::new();
    let (mut dx, mut dy) = (-1, 0);
    loop {
        if !all_pos.insert((x, y, dx, dy)) {
            return true;
        }
        x += dx;
        y += dy;
        if trees.contains(&(x, y)) || obs == (x, y) {
            x -= dx;
            y -= dy;
            (dx, dy) = match (dx, dy) {
                (-1, 0) => (0, 1),
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                _ => unreachable!(),
            };
        }

        if x < 0 || x > max_x || y < 0 || y > max_y {
            return false;
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut pos = (0, 0);
    let mut max_x = 0;
    let mut max_y = 0;
    let mut trees = HashSet::new();

    for (x, line) in input.lines().enumerate() {
        max_x = max_x.max(x as isize);
        for (y, c) in line.char_indices() {
            max_y = max_y.max(y as isize);
            match c {
                '^' => pos = (x as isize, y as isize),
                '#' => {
                    trees.insert((x as isize, y as isize));
                }
                _ => {}
            }
        }
    }

    let work = guard_pos(pos, (max_x, max_y), &trees)
        .into_iter()
        .filter(|xy| *xy != pos)
        .collect_vec();
    Some(
        work.into_par_iter()
            .filter(|obs| is_infinite_loop(pos, (max_x, max_y), &trees, *obs))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
