use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug)]
pub struct Round {
    reds: usize,
    greens: usize,
    blues: usize,
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl Game {
    pub fn is_possible(&self, r: usize, g: usize, b: usize) -> bool {
        self.rounds
            .iter()
            .all(|round| round.reds <= r && round.greens <= g && round.blues <= b)
    }

    pub fn power(&self) -> usize {
        let mut req_r = 0;
        let mut req_g = 0;
        let mut req_b = 0;

        for r in &self.rounds {
            req_r = req_r.max(r.reds);
            req_g = req_g.max(r.greens);
            req_b = req_b.max(r.blues);
        }

        req_r * req_g * req_b
    }
}

fn parse_round(round_data: &str) -> Round {
    let round = round_data
        .split(", ")
        .flat_map(|it| it.split(' ').collect_vec())
        .filter(|it| !it.is_empty())
        .collect_vec();

    let mut reds = 0;
    let mut blues = 0;
    let mut greens = 0;

    for chunk in round.chunks_exact(2) {
        match chunk {
            [amount, color] => {
                let amount = amount.parse::<usize>().expect("Could not parse amount");
                match *color {
                    "red" => reds = amount,
                    "green" => greens = amount,
                    "blue" => blues = amount,
                    _ => unreachable!("Got unknown color: {color}"),
                };
            }
            _ => unreachable!("Expected to have chunks of size 2"),
        };
    }

    Round {
        reds,
        greens,
        blues,
    }
}

#[aoc_generator(day02)]
pub fn generate(inp: &str) -> Vec<Game> {
    inp.lines().enumerate().fold(vec![], |mut acc, (id, line)| {
        let played_game = line
            .split_once(':')
            .expect(r"Expected 'Game <id>: ' prefix")
            .1;
        let rounds = played_game.split(';').fold(vec![], |mut acc, round_data| {
            let round = parse_round(round_data);
            acc.push(round);
            acc
        });

        acc.push(Game { id: id + 1, rounds });

        acc
    })
}

#[aoc(day02, part1)]
pub fn part1(inp: &[Game]) -> usize {
    const MAX_RED: usize = 12;
    const MAX_GREEN: usize = 13;
    const MAX_BLUE: usize = 14;

    inp.iter().fold(0, |acc, g| {
        acc + if g.is_possible(MAX_RED, MAX_GREEN, MAX_BLUE) {
            g.id
        } else {
            0
        }
    })
}

#[aoc(day02, part2)]
pub fn part2(inp: &[Game]) -> usize {
    inp.iter().fold(0, |acc, g| acc + g.power())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                              Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                              Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                              Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                              Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 8);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 2286);
    }
}
