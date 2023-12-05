use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Copy, Clone, Default, Debug)]
pub struct NumberRange {
    dest_start: usize,
    source_start: usize,
    length: usize,
}

impl NumberRange {
    const fn end(&self) -> usize {
        self.source_start + self.length - 1
    }

    const fn contains(&self, value: usize) -> bool {
        value >= self.source_start && value <= self.end()
    }

    const fn map_number(&self, value: usize) -> usize {
        assert!(self.contains(value));
        value - self.source_start + self.dest_start
    }
}

#[derive(Clone, Debug)]
pub struct Layer {
    maps: Vec<NumberRange>,
}

impl Layer {
    fn transform(&self, value: usize) -> usize {
        self.maps
            .iter()
            .find_map(|m| {
                if m.contains(value) {
                    Some(m.map_number(value))
                } else {
                    None
                }
            })
            .unwrap_or(value)
    }
}

#[derive(Clone, Debug)]
pub struct ParsedInput {
    seeds: Vec<usize>,
    mapping: Vec<Layer>,
}

fn parse_seeds(inp: &str) -> Vec<usize> {
    let nums = inp.split(": ").nth(1).expect("seeds: prefix");

    nums.split_ascii_whitespace()
        .filter_map(|it| it.parse::<usize>().ok())
        .collect_vec()
}

#[aoc_generator(day05)]
pub fn generate(inp: &str) -> ParsedInput {
    let blocks = inp.split_terminator("\n\n").collect_vec();
    let seeds = parse_seeds(blocks[0]);

    let mut mapping = Vec::new();

    for block in blocks.iter().skip(1) {
        let maps = block.lines().skip(1).fold(vec![], |mut acc, nums| {
            let parsed_nums = nums
                .split_ascii_whitespace()
                .map(|it| it.parse::<usize>().expect("number"))
                .collect_vec();
            acc.push(NumberRange {
                dest_start: parsed_nums[0],
                source_start: parsed_nums[1],
                length: parsed_nums[2],
            });
            acc
        });

        mapping.push(Layer { maps });
    }

    ParsedInput { seeds, mapping }
}

fn map_seed(seed: usize, mappings: &[Layer]) -> usize {
    mappings.iter().fold(seed, |acc, it| it.transform(acc))
}

#[aoc(day05, part1)]
pub fn part1(inp: &ParsedInput) -> usize {
    inp.seeds
        .iter()
        .map(|s| map_seed(*s, &inp.mapping))
        .min()
        .expect("minimum")
}

#[derive(Clone, Debug)]
struct SplitRanges {
    leftover: Vec<(usize, usize)>,
    processed: Vec<(usize, usize)>,
}

fn create_new_ranges(number_range: &NumberRange, (start, end): (usize, usize)) -> SplitRanges {
    if number_range.source_start <= start && end <= number_range.end() {
        //   [..., seed_range, ... ]
        // [...., number_range, .... ]
        // number_range fully contains seed_range -> convert seed_range
        SplitRanges {
            leftover: vec![],
            processed: vec![(number_range.map_number(start), number_range.map_number(end))],
        }
    } else if start <= number_range.source_start && number_range.end() <= end {
        // [......, seed_range, ......]
        //    [.., number_range, .. ]
        // seed_range fully contains number_range
        // -> keep the beginning
        // -> convert number_range source to dest
        // -> keep the end
        let processed = vec![(
            number_range.dest_start,
            number_range.dest_start + number_range.length - 1,
        )];

        let leftover = if start == number_range.source_start {
            vec![(number_range.end() + 1, end)]
        } else if end == number_range.end() {
            vec![(start, number_range.source_start - 1)]
        } else {
            vec![
                (start, number_range.source_start - 1),
                (number_range.end() + 1, end),
            ]
        };

        SplitRanges {
            leftover,
            processed,
        }
    } else if number_range.end() >= end
        && start < number_range.source_start
        && number_range.source_start <= end
    {
        // [..., seed_range, ... ]
        //        [..., number_range, ... ]
        // -> keep the beginning
        // -> convert number_range
        SplitRanges {
            leftover: vec![(start, number_range.source_start - 1)],
            processed: vec![(number_range.dest_start, number_range.map_number(end))],
        }
    } else if number_range.source_start <= start
        && start <= number_range.end()
        && number_range.end() < end
    {
        //        [..., seed_range, ... ]
        // [..., number_range, ... ]
        // -> convert the overlap
        // -> keep the end
        SplitRanges {
            leftover: vec![(number_range.end() + 1, end)],
            processed: vec![(
                number_range.map_number(start),
                number_range.map_number(number_range.end()),
            )],
        }
    } else {
        SplitRanges {
            leftover: vec![],
            processed: vec![],
        }
    }
}

fn calculate_seed_mapping(
    seed_range: (usize, usize),
    mapping: &[NumberRange],
) -> Vec<(usize, usize)> {
    let result = mapping.iter().fold(vec![], |mut acc, number_range| {
        let split_ranges = create_new_ranges(number_range, seed_range);
        let mut res = split_ranges.processed.clone();
        for &lo in &split_ranges.leftover {
            let remaining = calculate_seed_mapping(lo, mapping);
            res.extend_from_slice(&remaining);
        }

        acc.extend_from_slice(&res);
        acc
    });

    if result.is_empty() {
        vec![seed_range]
    } else {
        result
    }
}

#[aoc(day05, part2)]
pub fn part2(inp: &ParsedInput) -> usize {
    let mut seed_ranges = inp
        .seeds
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[0] + chunk[1] - 1))
        .collect_vec();

    for layer in &inp.mapping {
        let mapping = &layer.maps;
        seed_ranges = seed_ranges.iter().fold(vec![], |mut acc, &seed_range| {
            let new_ranges = calculate_seed_mapping(seed_range, mapping);
            acc.extend_from_slice(&new_ranges);
            acc
        });
    }

    seed_ranges
        .iter()
        .map(|&(start, _)| start)
        .min()
        .expect("minimum")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13\n\
                              \n\
                              seed-to-soil map:\n\
                              50 98 2\n\
                              52 50 48\n\
                              \n\
                              soil-to-fertilizer map:\n\
                              0 15 37\n\
                              37 52 2\n\
                              39 0 15\n\
                              \n\
                              fertilizer-to-water map:\n\
                              49 53 8\n\
                              0 11 42\n\
                              42 0 7\n\
                              57 7 4\n\
                              \n\
                              water-to-light map:\n\
                              88 18 7\n\
                              18 25 70\n\
                              \n\
                              light-to-temperature map:\n\
                              45 77 23\n\
                              81 45 19\n\
                              68 64 13\n\
                              \n\
                              temperature-to-humidity map:\n\
                              0 69 1\n\
                              1 0 69\n\
                              \n\
                              humidity-to-location map:\n\
                              60 56 37\n\
                              56 93 4";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 35);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 46);
    }

    /*
    #[test]
    fn test_p2_2() {
        let inp = include_str!("../input/2023/day5_2.txt");
        let gen = generate(inp);
        let res = part2(&gen);
        assert_eq!(res, 108_956_227);
    }
    */
}
