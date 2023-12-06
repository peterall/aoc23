use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{newline, space1, u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, separated_pair};
use nom::IResult;
advent_of_code::solution!(6);

// -- EXAMPLE --
// Time:      7  15   30
// Distance:  9  40  200

fn parse(i: &str) -> IResult<&str, Vec<(u32, u32)>> {
    map(
        separated_pair(
            preceded(pair(tag("Time:"), space1), separated_list1(space1, u32)),
            newline,
            preceded(
                pair(tag("Distance: "), space1),
                separated_list1(space1, u32),
            ),
        ),
        |(a, b)| a.into_iter().zip(b).collect(),
    )(i)
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse(input).unwrap().1;
    Some(
        races
            .iter()
            .map(|(time, distance)| {
                (1..time - 1)
                    .map(|t| t * (time - t))
                    .filter(|d| d > distance)
                    .count() as u32
            })
            .product(),
    )
}

fn parse_2(i: &str) -> IResult<&str, (u64, u64)> {
    map(
        separated_pair(
            preceded(pair(tag("Time:"), space1), take_until("\n")),
            newline,
            preceded(pair(tag("Distance: "), space1), take_until("\n")),
        ),
        |(a, b): (&str, &str)| {
            (
                a.replace(' ', "").parse::<u64>().unwrap(),
                b.replace(' ', "").parse::<u64>().unwrap(),
            )
        },
    )(i)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (time, distance) = parse_2(input).unwrap().1;

    Some(
        (1..time - 1)
            .map(|t| t * (time - t))
            .filter(|d| *d > distance)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
