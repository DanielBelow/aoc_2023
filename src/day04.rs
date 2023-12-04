use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct ScratchCard {
    id: usize,
    matches: usize,
}

impl ScratchCard {
    fn count_duplicates(&self, orig: &[Self]) -> usize {
        orig[self.id..self.id + self.matches]
            .iter()
            .fold(1, |acc, dupe| acc + dupe.count_duplicates(orig))
    }

    const fn points(&self) -> usize {
        match self.matches {
            0 => 0,
            n => 2usize.pow(n as u32 - 1),
        }
    }
}

fn parse_number_list(line: &str) -> HashSet<usize> {
    line.split_ascii_whitespace()
        .fold(HashSet::new(), |mut w, winning_num| {
            let num = winning_num.parse::<usize>().expect("input");
            w.insert(num);
            w
        })
}

#[aoc_generator(day04)]
pub fn generate(inp: &str) -> Vec<ScratchCard> {
    inp.lines().enumerate().fold(vec![], |mut acc, (id, line)| {
        let id = id + 1;

        let numbers = line.split_once(": ").expect("input").1;

        let (winning, have) = numbers.split_once(" | ").expect("input");

        let winning_numbers = parse_number_list(winning);
        let have = parse_number_list(have);

        let matches = winning_numbers.intersection(&have).count();

        acc.push(ScratchCard { id, matches });

        acc
    })
}

#[aoc(day04, part1)]
pub fn part1(inp: &[ScratchCard]) -> usize {
    inp.iter().map(ScratchCard::points).sum::<usize>()
}

#[aoc(day04, part2)]
pub fn part2(inp: &[ScratchCard]) -> usize {
    inp.iter()
        .fold(0, |acc, card| acc + card.count_duplicates(inp))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                              Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                              Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                              Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                              Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                              Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 13);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 30);
    }
}
