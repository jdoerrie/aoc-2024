use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        Regex::new(r"mul\((?<lhs>[0-9]{1,3}),(?<rhs>[0-9]{1,3})\)")
            .unwrap()
            .captures_iter(input)
            .map(|caps| {
                caps.name("lhs").unwrap().as_str().parse::<u32>().unwrap()
                    * caps.name("rhs").unwrap().as_str().parse::<u32>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let re =
        r"(?<mul>mul\((?<lhs>[0-9]{1,3}),(?<rhs>[0-9]{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))";
    let mut on = true;
    Some(
        Regex::new(re)
            .unwrap()
            .captures_iter(input)
            .flat_map(|caps| {
                on = (on || caps.name("do").is_some()) && (caps.name("dont").is_none());
                if on && caps.name("mul").is_some() {
                    Some(
                        ["lhs", "rhs"]
                            .into_iter()
                            .flat_map(|name| caps.name(name).unwrap().as_str().parse::<u32>())
                            .product::<u32>(),
                    )
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
