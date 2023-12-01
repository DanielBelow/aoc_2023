use aoc_runner_derive::aoc;

const NUMS: &[&str; 9] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_digit(s: &str, check_nums: bool) -> u32 {
    for (idx, c) in s.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            return d;
        }

        if check_nums {
            let rest = &s[idx..];
            for (num, n) in NUMS.iter().enumerate() {
                if rest.starts_with(n) {
                    return num as u32 + 1;
                }
            }
        }
    }

    unreachable!("Couldn't find any digit")
}

fn rfind_digit(s: &str, check_nums: bool) -> u32 {
    let len = s.len();

    for (idx, c) in s.chars().rev().enumerate() {
        if let Some(d) = c.to_digit(10) {
            return d;
        }

        if check_nums {
            let rest = &s[..len - idx];
            for (num, n) in NUMS.iter().enumerate() {
                if rest.ends_with(n) {
                    return num as u32 + 1;
                }
            }
        }
    }

    unreachable!("Couldn't find any digit")
}

fn calibration_value(line: &str, check_nums: bool) -> u32 {
    let first = find_digit(line, check_nums);
    let last = rfind_digit(line, check_nums);

    first * 10 + last
}

#[aoc(day01, part1)]
pub fn part1(inp: &str) -> u32 {
    inp.lines()
        .fold(0, |acc, l| acc + calibration_value(l, false))
}

#[aoc(day01, part2)]
pub fn part2(inp: &str) -> u32 {
    inp.lines()
        .fold(0, |acc, l| acc + calibration_value(l, true))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "1abc2\n\
                                 pqr3stu8vwx\n\
                                 a1b2c3d4e5f\n\
                                 treb7uchet";

    const TEST_INPUT_P2: &str = "two1nine\n\
                                 eightwothree\n\
                                 abcone2threexyz\n\
                                 xtwone3four\n\
                                 4nineeightseven2\n\
                                 zoneight234\n\
                                 7pqrstsixteen";

    #[test]
    fn test_p1() {
        let res = part1(TEST_INPUT_P1);
        assert_eq!(res, 142);
    }

    #[test]
    fn test_p2() {
        let res = part2(TEST_INPUT_P2);
        assert_eq!(res, 281);
    }
}
