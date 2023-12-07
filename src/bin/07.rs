use std::{cmp::Ordering, collections::HashSet};

use nom::{
    bytes::complete::take,
    character::complete::{newline, space1, u32},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

advent_of_code::solution!(7);

// example:
// 32T3K 765
// T55J5 684
// KK677 28
// KTJJT 220
// QQQJA 483

// parse example data using nom
fn parse(i: &str) -> IResult<&str, Vec<(Hand, u32)>> {
    separated_list1(
        newline,
        separated_pair(
            map(take(5_usize), |hand: &str| {
                Hand(
                    hand.chars()
                        .map(|c| match c {
                            'A' => 14_u8,
                            'K' => 13_u8,
                            'Q' => 12_u8,
                            'J' => 11_u8,
                            'T' => 10_u8,
                            _ => c.to_digit(10).unwrap() as u8,
                        })
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                )
            }),
            space1,
            u32,
        ),
    )(i)
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand([u8; 5]);

impl Hand {
    fn score(&self) -> u32 {
        let mut sorted = self.0;
        sorted.sort();
        match sorted {
            [a, b, c, d, e] if a == b && b == c && c == d && d == e => 7,
            [a, b, c, d, _] if a == b && b == c && c == d => 6,
            [_, b, c, d, e] if b == c && c == d && d == e => 6,
            [a, b, c, d, e] if a == b && b == c && d == e => 5,
            [a, b, c, d, e] if a == b && c == d && d == e => 5,
            [a, b, c, _, _] if a == b && b == c => 4,
            [_, b, c, d, _] if b == c && c == d => 4,
            [_, _, c, d, e] if c == d && d == e => 4,
            [a, b, c, d, _] if a == b && c == d => 3,
            [a, b, _, d, e] if a == b && d == e => 3,
            [_, b, c, d, e] if b == c && d == e => 3,
            [a, b, _, _, _] if a == b => 2,
            [_, b, c, _, _] if b == c => 2,
            [_, _, c, d, _] if c == d => 2,
            [_, _, _, d, e] if d == e => 2,
            _ => 1,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score()
            .cmp(&other.score())
            .then(self.0[0].cmp(&other.0[0]))
            .then(self.0[1].cmp(&other.0[1]))
            .then(self.0[2].cmp(&other.0[2]))
            .then(self.0[3].cmp(&other.0[3]))
            .then(self.0[4].cmp(&other.0[4]))
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = parse(input).unwrap().1;

    hands.sort_by(|a, b| a.0.cmp(&b.0));

    hands
        .iter()
        .enumerate()
        .map(|(i, (_, bet))| bet * (i as u32 + 1))
        .sum::<u32>()
        .into()
}

fn parse2(i: &str) -> IResult<&str, Vec<(JokerHand, u32)>> {
    separated_list1(
        newline,
        separated_pair(
            map(take(5_usize), |hand: &str| {
                JokerHand::new(
                    hand.chars()
                        .map(|c| match c {
                            'A' => 14_u8,
                            'K' => 13_u8,
                            'Q' => 12_u8,
                            'J' => 1_u8,
                            'T' => 10_u8,
                            _ => c.to_digit(10).unwrap() as u8,
                        })
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap(),
                )
            }),
            space1,
            u32,
        ),
    )(i)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct JokerHand {
    cards: [u8; 5],
    score: u32,
}

impl JokerHand {
    fn get_max_joker_score(cards: [u8; 5], i: usize) -> u32 {
        let mut max_score = Self::score(cards);
        for j in i..5 {
            if cards[j] == 1 {
                for k in 0..5 {
                    if j != k {
                        let mut new_cards = cards;
                        new_cards[j] = new_cards[k];
                        max_score = max_score.max(Self::get_max_joker_score(new_cards, j + 1));
                    }
                }
            }
        }
        max_score
    }
    fn new(cards: [u8; 5]) -> JokerHand {
        JokerHand {
            cards,
            score: Self::get_max_joker_score(cards, 0),
        }
    }
    fn score(mut cards: [u8; 5]) -> u32 {
        cards.sort();
        match cards {
            [a, b, c, d, e] if a == b && b == c && c == d && d == e => 7,
            [a, b, c, d, _] if a == b && b == c && c == d => 6,
            [_, b, c, d, e] if b == c && c == d && d == e => 6,
            [a, b, c, d, e] if a == b && b == c && d == e => 5,
            [a, b, c, d, e] if a == b && c == d && d == e => 5,
            [a, b, c, _, _] if a == b && b == c => 4,
            [_, b, c, d, _] if b == c && c == d => 4,
            [_, _, c, d, e] if c == d && d == e => 4,
            [a, b, c, d, _] if a == b && c == d => 3,
            [a, b, _, d, e] if a == b && d == e => 3,
            [_, b, c, d, e] if b == c && d == e => 3,
            [a, b, _, _, _] if a == b => 2,
            [_, b, c, _, _] if b == c => 2,
            [_, _, c, d, _] if c == d => 2,
            [_, _, _, d, e] if d == e => 2,
            _ => 1,
        }
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score
            .cmp(&other.score)
            .then(self.cards[0].cmp(&other.cards[0]))
            .then(self.cards[1].cmp(&other.cards[1]))
            .then(self.cards[2].cmp(&other.cards[2]))
            .then(self.cards[3].cmp(&other.cards[3]))
            .then(self.cards[4].cmp(&other.cards[4]))
    }
}
impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = parse2(input).unwrap().1;

    hands.sort_by(|a, b| a.0.cmp(&b.0));

    hands
        .iter()
        .enumerate()
        .map(|(i, (cards, bet))| bet * (i as u32 + 1))
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
