use aoc_runner_derive::{aoc, aoc_generator};
use parse_display_derive::{Display, FromStr};

#[derive(Copy, Clone, Display, FromStr)]
pub enum Cube {
    #[display("{0} red")]
    Red(usize),

    #[display("{0} green")]
    Green(usize),

    #[display("{0} blue")]
    Blue(usize),
}

#[derive(Copy, Clone, Default, Debug)]
pub struct CubeInfo {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    cube_info: CubeInfo,
}

impl Game {
    const fn is_possible(&self, r: usize, g: usize, b: usize) -> bool {
        self.cube_info.red <= r && self.cube_info.green <= g && self.cube_info.blue <= b
    }

    const fn power(&self) -> usize {
        self.cube_info.red * self.cube_info.green * self.cube_info.blue
    }
}

fn parse_game(id: usize, line: &str) -> Game {
    let played_game = line
        .split_once(':')
        .expect(r"Expected 'Game <id>: ' prefix")
        .1;

    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for played_round in played_game.split(';') {
        for data in played_round.split(", ") {
            match data.trim().parse::<Cube>().expect("Could not parse round") {
                Cube::Red(n) => red = red.max(n),
                Cube::Green(n) => green = green.max(n),
                Cube::Blue(n) => blue = blue.max(n),
            };
        }
    }

    Game {
        id: id + 1,
        cube_info: CubeInfo { red, green, blue },
    }
}

#[aoc_generator(day02)]
pub fn generate(inp: &str) -> Vec<Game> {
    inp.lines().enumerate().fold(vec![], |mut acc, (id, line)| {
        acc.push(parse_game(id, line));
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
