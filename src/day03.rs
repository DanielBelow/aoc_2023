use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
pub struct ParsedNumber {
    row: usize,
    start: usize,
    end: usize,
    value: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct Symbol {
    row: usize,
    col: usize,
    sym: char,
}

impl ParsedNumber {
    const fn is_adjacent(&self, y: usize, x: usize) -> bool {
        self.row.abs_diff(y) <= 1 && x <= self.end && x >= self.start.saturating_sub(1)
    }

    const fn touches_symbol(&self, sym: &Symbol) -> bool {
        self.is_adjacent(sym.row, sym.col)
    }

    fn touches_any_symbol(&self, syms: &[Symbol]) -> bool {
        syms.iter().any(|s| self.touches_symbol(s))
    }
}

#[derive(Clone, Debug)]
pub struct ParsedInput {
    symbols: Vec<Symbol>,
    numbers: Vec<ParsedNumber>,
}

impl ParsedInput {
    fn gears(&self) -> impl Iterator<Item = &Symbol> {
        self.symbols.iter().filter(|it| it.sym == '*')
    }
}

fn collect_numbers(inp: &str) -> Vec<ParsedNumber> {
    let re = Regex::new(r"\d+").expect("valid regex");

    inp.lines()
        .enumerate()
        .fold(vec![], |mut acc, (row_idx, row)| {
            for match_result in re.find_iter(row) {
                acc.push(ParsedNumber {
                    row: row_idx,
                    start: match_result.start(),
                    end: match_result.end(),
                    value: match_result
                        .as_str()
                        .parse::<usize>()
                        .expect("matched regex"),
                });
            }

            acc
        })
}

fn collect_symbols(inp: &str) -> Vec<Symbol> {
    inp.lines()
        .enumerate()
        .fold(vec![], |acc, (row_idx, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| is_symbol(c))
                .fold(acc, |mut acc, (col_idx, c)| {
                    acc.push(Symbol {
                        row: row_idx,
                        col: col_idx,
                        sym: c,
                    });

                    acc
                })
        })
}

#[aoc_generator(day03)]
pub fn generate(inp: &str) -> ParsedInput {
    let numbers = collect_numbers(inp);
    let symbols = collect_symbols(inp);

    ParsedInput { symbols, numbers }
}

const fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

#[aoc(day03, part1)]
pub fn part1(inp: &ParsedInput) -> usize {
    inp.numbers
        .iter()
        .filter(|&it| it.touches_any_symbol(&inp.symbols))
        .fold(0, |acc, it| acc + it.value)
}

#[aoc(day03, part2)]
pub fn part2(inp: &ParsedInput) -> usize {
    inp.gears()
        .map(|it| {
            inp.numbers
                .iter()
                .filter(|num| num.touches_symbol(it))
                .map(|num| num.value)
                .collect_vec()
        })
        .filter(|it| it.len() == 2)
        .fold(0, |acc, ratios| acc + ratios[0] * ratios[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "467..114..\n\
                              ...*......\n\
                              ..35..633.\n\
                              ......#...\n\
                              617*......\n\
                              .....+.58.\n\
                              ..592.....\n\
                              ......755.\n\
                              ...$.*....\n\
                              .664.598..";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 4361);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 467_835);
    }
}
