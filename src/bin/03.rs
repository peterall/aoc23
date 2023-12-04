use std::collections::{HashMap, HashSet};
use std::ops::Range;

use nom::bytes::complete::{take, take_while1};
use nom::character::complete::newline;
use nom::{branch::alt, multi::many1};
use nom::{character::complete::digit1, combinator::map};
use nom_locate::LocatedSpan;
type Span<'a> = LocatedSpan<&'a str>;

advent_of_code::solution!(3);

enum Item<'a> {
    Newline(char),
    Dots(Span<'a>),
    Number(Span<'a>),
    Symbol(Span<'a>),
}

fn parse_map(input: Span) -> nom::IResult<Span, Vec<Item>> {
    many1(alt((
        map(take_while1(|n| n == '.'), Item::Dots),
        map(digit1, Item::Number),
        map(newline, Item::Newline),
        map(take(1_usize), Item::Symbol),
    )))(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let items = parse_map(Span::new(input)).unwrap().1;

    let symbols = items
        .iter()
        .flat_map(|i| match i {
            Item::Symbol(s) => Some((s.get_column(), s.location_line())),
            _ => None,
        })
        .collect::<HashSet<_>>();

    let check_line =
        |line: u32, mut columns: Range<usize>| columns.any(|c| symbols.contains(&(c, line)));

    let sum = items
        .iter()
        .map(|i| match i {
            Item::Number(n) => {
                let r = (n.get_column() - 1)..(n.get_column() + n.fragment().len() + 1);
                if check_line(n.location_line(), r.clone())
                    || check_line(n.location_line() + 1, r.clone())
                    || check_line(n.location_line() - 1, r.clone())
                {
                    println!(
                        "{}:{} -> {}",
                        n.location_line(),
                        n.get_column(),
                        n.fragment()
                    );
                    n.fragment().parse::<u32>().unwrap()
                } else {
                    0
                }
            }
            _ => 0,
        })
        .sum::<u32>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let items = parse_map(Span::new(input)).unwrap().1;

    let gears = items
        .iter()
        .flat_map(|i| match i {
            Item::Symbol(s) if s.fragment() == &"*" => Some((s.location_line(), s.get_column())),
            _ => None,
        })
        .collect::<Vec<_>>();

    let ratios = items
        .iter()
        .filter(|n| matches!(n, Item::Number(_)))
        .enumerate()
        .flat_map(|(id, i)| match i {
            Item::Number(n) => {
                let ratio = n.fragment().parse::<u32>().unwrap();
                (n.get_column()..n.get_column() + n.fragment().len())
                    .map(move |c| ((n.location_line(), c), (id, ratio)))
            }
            _ => unreachable!(),
        })
        .collect::<HashMap<_, _>>();

    Some(
        gears
            .into_iter()
            .map(|(line, column)| {
                // positions are all 8 squares adjacent to the gear
                let positions = [
                    (line - 1, column - 1),
                    (line - 1, column),
                    (line - 1, column + 1),
                    (line, column - 1),
                    (line, column + 1),
                    (line + 1, column - 1),
                    (line + 1, column),
                    (line + 1, column + 1),
                ];
                let gears = positions
                    .iter()
                    .flat_map(|p| ratios.get(&p).cloned())
                    .collect::<HashMap<_, _>>()
                    .into_iter()
                    .collect::<Vec<_>>();

                if gears.len() == 2 {
                    gears.first().unwrap().1 * gears.last().unwrap().1
                } else {
                    0
                }
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
