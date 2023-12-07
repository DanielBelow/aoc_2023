use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;

fn hand_type(s: &str) -> u8 {
    let freqs = s.chars().counts().values().copied().sorted().collect_vec();
    match freqs.as_slice() {
        [5] => 6,
        [1, 4] => 5,
        [2, 3] => 4,
        [1, 1, 3] => 3,
        [1, 2, 2] => 2,
        [1, 1, 1, 2] => 1,
        _ => 0,
    }
}

fn compare_card_strength(lhs: &str, rhs: &str, joker_is_zero: bool) -> Ordering {
    lhs.chars()
        .zip(rhs.chars())
        .find(|&(l, r)| l != r)
        .map_or(Ordering::Equal, |(l, r)| {
            card_strength(l, joker_is_zero).cmp(&card_strength(r, joker_is_zero))
        })
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Hand {
    hand: String,
    bid: usize,
    hand_type: u8,
    strongest_possible: Option<String>,
}

impl Hand {
    fn find_strongest_hand(&self) -> String {
        let freqs = self.hand.chars().counts();

        let replaced_joker = freqs
            .iter()
            .filter(|&(c, _)| *c != 'J')
            .max_by_key(|&(_, a)| *a)
            .map_or('A', |(c, _)| *c)
            .to_string();

        self.hand.replace('J', &replaced_joker)
    }

    fn calc_strongest_possible(&mut self) {
        if !self.hand.contains('J') {
            return;
        }

        self.strongest_possible = Some(self.find_strongest_hand());
    }
}

fn card_strength(card: char, joker_is_zero: bool) -> usize {
    match card {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => {
            if joker_is_zero {
                0
            } else {
                10
            }
        }
        'T' => 9,
        _ => card.to_digit(10).expect("unknown card") as usize - 1,
    }
}

const fn total_winnings(sum: usize, (bid, rank): (usize, usize)) -> usize {
    sum + rank * bid
}

#[aoc_generator(day07)]
pub fn generate(inp: &str) -> Vec<Hand> {
    inp.lines()
        .map(|it| {
            let (h, b) = it.split_once(' ').expect("input");
            let hand_type = hand_type(h);
            Hand {
                hand: h.to_string(),
                bid: b.parse::<usize>().expect("bid"),
                hand_type,
                strongest_possible: None,
            }
        })
        .collect_vec()
}

#[aoc(day07, part1)]
pub fn part1(inp: &[Hand]) -> usize {
    inp.iter()
        .sorted_by(|lhs, rhs| {
            lhs.hand_type
                .cmp(&rhs.hand_type)
                .then_with(|| compare_card_strength(&lhs.hand, &rhs.hand, false))
        })
        .map(|it| it.bid)
        .zip(1usize..)
        .fold(0, total_winnings)
}

#[aoc(day07, part2)]
pub fn part2(inp: &[Hand]) -> usize {
    let strongest_hand_type = |hand: &Hand| {
        hand.strongest_possible
            .as_ref()
            .map_or(hand.hand_type, |it| hand_type(it))
    };

    inp.to_owned()
        .iter_mut()
        .update(|it| {
            it.calc_strongest_possible();
        })
        .sorted_by(|lhs, rhs| {
            let lhs_strongest = strongest_hand_type(lhs);
            let rhs_strongest = strongest_hand_type(rhs);
            lhs_strongest
                .cmp(&rhs_strongest)
                .then_with(|| compare_card_strength(&lhs.hand, &rhs.hand, true))
        })
        .map(|it| it.bid)
        .zip(1usize..)
        .fold(0, total_winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "32T3K 765\n\
                              T55J5 684\n\
                              KK677 28\n\
                              KTJJT 220\n\
                              QQQJA 483";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 6440);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 5905);
    }
}
