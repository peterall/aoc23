use std::ops::Range;

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{newline, space0, space1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

advent_of_code::solution!(5);

// seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37

fn parse(i: &str) -> IResult<&str, (Vec<u64>, Vec<Vec<(i64, Range<u64>)>>)> {
    pair(
        delimited(tag("seeds: "), separated_list1(space1, u64), tag("\n\n")),
        separated_list1(
            tag("\n\n"),
            preceded(
                pair(take_until("\n"), newline),
                separated_list1(
                    newline,
                    map(
                        tuple((
                            preceded(space0, u64),
                            preceded(space0, u64),
                            preceded(space0, u64),
                        )),
                        |(dest, src, n)| (dest as i64 - src as i64, src..src + n),
                    ),
                ),
            ),
        ),
    )(i)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (seeds, almanac) = parse(input).unwrap().1;

    let min_location = seeds
        .iter()
        .map(|&seed| {
            almanac.iter().fold(seed, |pos, map| {
                map.iter()
                    .find(|(_, range)| range.contains(&pos))
                    .map(|(delta, _)| (pos as i64 + delta) as u64)
                    .unwrap_or(pos)
            })
        })
        .min();

    min_location
}

pub fn part_two(input: &str) -> Option<u64> {
    let (seeds, almanac) = parse(input).unwrap().1;

    let min_location = seeds
        .chunks(2)
        .flat_map(|c| c[0]..c[0] + c[1])
        .map(|seed| {
            almanac.iter().fold(seed, |pos, map| {
                map.iter()
                    .find(|(_, range)| range.contains(&pos))
                    .map(|(delta, _)| (pos as i64 + delta) as u64)
                    .unwrap_or(pos)
            })
        })
        .min();

    min_location
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
