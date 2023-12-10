use std::{collections::HashMap, iter};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, separated_pair},
    IResult,
};

advent_of_code::solution!(8);

fn parse(i: &str) -> IResult<&str, (String, HashMap<String, (String, String)>)> {
    separated_pair(
        map(alpha1, |s: &str| s.to_owned()),
        pair(newline, newline),
        map(
            separated_list1(
                newline,
                separated_pair(
                    map(alphanumeric1, |s: &str| s.to_owned()),
                    tag(" = "),
                    delimited(
                        tag("("),
                        separated_pair(
                            map(alphanumeric1, |s: &str| s.to_owned()),
                            tag(", "),
                            map(alphanumeric1, |s: &str| s.to_owned()),
                        ),
                        tag(")"),
                    ),
                ),
            ),
            |v| v.into_iter().collect(),
        ),
    )(i)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (path, map) = parse(input).unwrap().1;
    let mut current = "AAA";
    for (i, c) in iter::repeat_with(|| path.chars()).flatten().enumerate() {
        if current == "ZZZ" {
            return Some(i as u32);
        }
        current = match c {
            'L' => map[current].0.as_str(),
            'R' => map[current].1.as_str(),
            _ => unreachable!(),
        };
    }
    unreachable!()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (path, map) = parse(input).unwrap().1;
    let mut currents = map
        .keys()
        .filter(|k| k.chars().nth(2).unwrap() == 'A')
        .map(|k| k.as_str())
        .collect::<Vec<_>>();

    for (i, c) in iter::repeat_with(|| path.chars()).flatten().enumerate() {
        if currents.iter().all(|c| c.chars().nth(2).unwrap() == 'Z') {
            return Some(i as u32);
        }
        for current in currents.iter_mut() {
            *current = match c {
                'L' => map[*current].0.as_str(),
                'R' => map[*current].1.as_str(),
                _ => unreachable!(),
            };
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
