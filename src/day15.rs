use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day15)]
pub fn generate(inp: &str) -> Vec<String> {
    inp.split(',').map(ToString::to_string).collect()
}

fn hash(c: &str) -> usize {
    // Determine the ASCII code for the current character of the string.
    // Increase the current value by the ASCII code you just determined.
    // Set the current value to itself multiplied by 17.
    // Set the current value to the remainder of dividing itself by 256.
    c.chars()
        .fold(0, |acc, it| ((acc + (it as usize)) * 17) % 256)
}

#[aoc(day15, part1)]
pub fn part1(inp: &[String]) -> usize {
    inp.iter().fold(0, |acc, it| acc + hash(it))
}

#[derive(Clone, Debug)]
struct BoxWithLens {
    label: String,
    focal: usize,
}

#[aoc(day15, part2)]
pub fn part2(inp: &[String]) -> usize {
    let mut boxes: Vec<Vec<BoxWithLens>> = Vec::with_capacity(256);
    boxes.resize(256, vec![]);

    for s in inp {
        let label = s.chars().take_while(|it| *it != '-' && *it != '=').join("");
        let op = if s.contains('-') { '-' } else { '=' };

        let box_num = hash(&label);
        assert!((0..256).contains(&box_num));

        match op {
            '=' => {
                let b = &mut boxes[box_num];
                let focal = s
                    .chars()
                    .filter(char::is_ascii_digit)
                    .join("")
                    .parse::<usize>()
                    .expect("number");
                if let Some(slot) = b.iter_mut().find(|it| it.label == label) {
                    slot.focal = focal;
                } else {
                    b.push(BoxWithLens { label, focal });
                }
            }
            '-' => {
                let b = &mut boxes[box_num];
                if let Some(slot) = b.iter_mut().position(|it| it.label == label) {
                    b.remove(slot);
                }
            }
            _ => panic!("unknown operation"),
        }
    }

    // The focusing power of a single lens is the result of multiplying together:
    // One plus the box number of the lens in question.
    // The slot number of the lens within the box: 1 for the first lens, 2 for the second lens, and so on.
    // The focal length of the lens.
    boxes.iter().enumerate().fold(0, |acc, (box_num, it)| {
        let mut result = 0;

        for (slot, b) in it.iter().enumerate() {
            result += (box_num + 1) * (slot + 1) * b.focal;
        }

        acc + result
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 1320);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 145);
    }
}
