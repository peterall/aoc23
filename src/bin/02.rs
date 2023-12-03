use std::cmp::max;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha0, char, space1, u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{pair, preceded, tuple};
use nom::IResult;

advent_of_code::solution!(2);

struct Hand {
    blue: u32,
    red: u32,
    green: u32,
}

fn parse_game(i: &str) -> IResult<&str, Vec<Hand>> {
    preceded(
        tuple((tag("Game "), u32, char(':'))),
        separated_list1(
            char(';'),
            map(
                separated_list1(
                    char(','),
                    pair(preceded(space1, u32), preceded(space1, alpha0)),
                ),
                |l| {
                    l.into_iter().fold::<Hand, _>(
                        Hand {
                            blue: 0,
                            red: 0,
                            green: 0,
                        },
                        |hand, (n, c)| match c {
                            "blue" => Hand { blue: n, ..hand },
                            "red" => Hand { red: n, ..hand },
                            "green" => Hand { green: n, ..hand },
                            _ => hand,
                        },
                    )
                },
            ),
        ),
    )(i)
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = input
        .lines()
        .map(|l| parse_game(l).unwrap().1)
        .collect::<Vec<_>>();

    let limit = Hand {
        blue: 14,
        red: 12,
        green: 13,
    };

    let sum = games
        .iter()
        .enumerate()
        .filter(|game| {
            game.1
                .iter()
                .all(|h| h.blue <= limit.blue && h.red <= limit.red && h.green <= limit.green)
        })
        .map(|h| (h.0 + 1) as u32)
        .sum::<u32>();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = input
        .lines()
        .map(|l| parse_game(l).unwrap().1)
        .collect::<Vec<_>>();

    let total_power = games
        .into_iter()
        .map(|game| {
            let max = game
                .into_iter()
                .reduce(|a, b| Hand {
                    blue: max(a.blue, b.blue),
                    red: max(a.red, b.red),
                    green: max(a.green, b.green),
                })
                .unwrap();
            max.blue * max.red * max.green
        })
        .sum::<u32>();
    Some(total_power)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
