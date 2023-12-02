use aoc_runner_derive::{aoc, aoc_generator};
use parse_display_derive::{Display, FromStr};

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

#[derive(Copy, Clone, Display, FromStr)]
pub enum Cube {
    #[display("{0} red")]
    Red(usize),

    #[display("{0} green")]
    Green(usize),

    #[display("{0} blue")]
    Blue(usize),
}

#[derive(Debug)]
pub struct Game {
    id: usize,
    biggest_red: usize,
    biggest_green: usize,
    biggest_blue: usize,
}

impl Game {
    const fn is_possible(&self) -> bool {
        self.biggest_red <= MAX_RED
            && self.biggest_green <= MAX_GREEN
            && self.biggest_blue <= MAX_BLUE
    }

    fn points(&self) -> usize {
        self.is_possible().then_some(self.id).unwrap_or_default()
    }

    const fn power(&self) -> usize {
        self.biggest_red * self.biggest_green * self.biggest_blue
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

    for cube in played_game
        .split(';')
        .flat_map(|r| r.split(", "))
        .filter_map(|r| r.trim().parse::<Cube>().ok())
    {
        match cube {
            Cube::Red(n) => red = red.max(n),
            Cube::Green(n) => green = green.max(n),
            Cube::Blue(n) => blue = blue.max(n),
        };
    }

    Game {
        id: id + 1,
        biggest_red: red,
        biggest_green: green,
        biggest_blue: blue,
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
    inp.iter().fold(0, |acc, g| acc + g.points())
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
