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
    let mut on = true;
    Some( Regex::new(r"(?<mul>mul\((?<lhs>[0-9]{1,3}),(?<rhs>[0-9]{1,3})\))|(?<do>do\(\))|(?<dont>don't\(\))") .unwrap() .captures_iter(input) .map(|caps| { if caps.name("do").is_some() { on = true; 0 } else if caps.name("dont").is_some() { on = false; 0 } else if on { caps.name("lhs").unwrap().as_str().parse::<u32>().unwrap() * caps.name("rhs").unwrap().as_str().parse::<u32>().unwrap() } else { 0} }) .sum(),)
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
