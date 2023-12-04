use std::collections::HashSet;

use nom::character::complete::{newline, space1, u8};
use nom::sequence::{preceded, tuple};
use nom::IResult;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map, multi::separated_list1,
    sequence::separated_pair,
};
advent_of_code::solution!(4);

pub fn parse_cards(input: &str) -> IResult<&str, Vec<(HashSet<u8>, Vec<u8>)>> {
    separated_list1(
        newline,
        preceded(
            tuple((tag("Card"), space1, digit1, tag(":"))),
            separated_pair(
                preceded(space1, map(separated_list1(space1, u8), HashSet::from_iter)),
                tag(" |"),
                preceded(space1, separated_list1(space1, u8)),
            ),
        ),
    )(input)
}

pub fn part_one(input: &str) -> Option<u32> {
    let games: Vec<(HashSet<u8>, Vec<u8>)> = parse_cards(input).unwrap().1;
    Some(
        games
            .into_iter()
            .map(|(winners, cards)| {
                cards.into_iter().fold(0, |acc, card| {
                    if winners.contains(&card) {
                        if acc == 0 {
                            1
                        } else {
                            acc + acc
                        }
                    } else {
                        acc
                    }
                })
            })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<(HashSet<u8>, Vec<u8>)> = parse_cards(input).unwrap().1;

    let mut cards_on_hand = (0..cards.len()).collect::<Vec<usize>>();
    let mut i = 0_usize;
    while i < cards_on_hand.len() {
        let card = cards_on_hand[i];
        let new_cards = cards[card]
            .1
            .iter()
            .filter(|c| cards[card].0.contains(c))
            .count();
        for new_card in card + 1..card + new_cards + 1 {
            cards_on_hand.push(new_card);
        }
        i += 1;
    }
    Some(cards_on_hand.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
