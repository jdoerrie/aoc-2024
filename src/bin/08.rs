use hashbrown::HashSet;
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .map(move |(j, c)| (i as isize, j as isize, c))
        })
        .collect_vec();
    let (max_i, max_j, _) = *input.last().unwrap();
    let antennas = input
        .into_iter()
        .filter_map(|(x, y, c)| if c == '.' { None } else { Some((c, (x, y))) })
        .into_group_map();
    Some(
        antennas
            .into_iter()
            .flat_map(|(_, vs)| {
                vs.into_iter()
                    .tuple_combinations()
                    .flat_map(|((x_i, x_j), (y_i, y_j))| {
                        let d_i = y_i - x_i;
                        let d_j = y_j - x_j;

                        [(y_i + d_i, y_j + d_j), (x_i - d_i, x_j - d_j)]
                            .into_iter()
                            .filter(|&(i, j)| 0 <= i && i <= max_i && 0 <= j && j <= max_j)
                    })
            })
            .collect::<HashSet<_>>()
            .len(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.char_indices()
                .map(move |(j, c)| (i as isize, j as isize, c))
        })
        .collect_vec();
    let (max_i, max_j, _) = *input.last().unwrap();
    let antennas = input
        .into_iter()
        .filter_map(|(x, y, c)| if c == '.' { None } else { Some((c, (x, y))) })
        .into_group_map();
    Some(
        antennas
            .into_iter()
            .flat_map(|(_, vs)| {
                vs.into_iter()
                    .tuple_combinations()
                    .flat_map(|((x_i, x_j), (y_i, y_j))| {
                        let d_i = y_i - x_i;
                        let d_j = y_j - x_j;

                        let mut posis = Vec::new();
                        let (mut up_i, mut up_j) = (y_i, y_j);
                        while 0 <= up_i && up_i <= max_i && 0 <= up_j && up_j <= max_j {
                            posis.push((up_i, up_j));
                            up_i += d_i;
                            up_j += d_j;
                        }

                        let (mut dn_i, mut dn_j) = (x_i, x_j);
                        while 0 <= dn_i && dn_i <= max_i && 0 <= dn_j && dn_j <= max_j {
                            posis.push((dn_i, dn_j));
                            dn_i -= d_i;
                            dn_j -= d_j;
                        }

                        posis.into_iter()
                    })
            })
            .collect::<HashSet<_>>()
            .len(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
