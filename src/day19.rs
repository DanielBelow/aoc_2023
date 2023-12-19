use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use parse_display_derive::{Display, FromStr};
use std::ops::RangeInclusive;

#[derive(Display, FromStr, PartialEq, Eq, Copy, Clone, Debug)]
pub enum InputValue {
    #[display("x")]
    X,

    #[display("m")]
    M,

    #[display("a")]
    A,

    #[display("s")]
    S,
}

#[derive(Display, FromStr, PartialEq, Eq, Clone, Debug)]
pub enum Rule {
    #[display("{0}<{1}:{2}")]
    LT(InputValue, usize, String),

    #[display("{0}>{1}:{2}")]
    GT(InputValue, usize, String),

    #[display("{0}")]
    Next(String),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Workflow {
    id: String,
    rules: Vec<Rule>,
}

#[derive(Display, FromStr, PartialEq, Eq, Copy, Clone, Debug)]
#[display("{{x={x},m={m},a={a},s={s}}}")]
pub struct Input {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct InputRange {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl InputRange {
    fn count(&self) -> usize {
        self.x.clone().count()
            * self.m.clone().count()
            * self.a.clone().count()
            * self.s.clone().count()
    }

    #[allow(clippy::range_minus_one)]
    fn sub_range_less(&self, inp: InputValue, n: usize) -> Self {
        match inp {
            InputValue::X => Self {
                x: *self.x.start()..=n - 1,
                ..self.clone()
            },
            InputValue::M => Self {
                m: *self.m.start()..=n - 1,
                ..self.clone()
            },
            InputValue::A => Self {
                a: *self.a.start()..=n - 1,
                ..self.clone()
            },
            InputValue::S => Self {
                s: *self.s.start()..=n - 1,
                ..self.clone()
            },
        }
    }

    fn sub_range_greater(&self, inp: InputValue, n: usize) -> Self {
        match inp {
            InputValue::X => Self {
                x: n + 1..=*self.x.end(),
                ..self.clone()
            },
            InputValue::M => Self {
                m: n + 1..=*self.m.end(),
                ..self.clone()
            },
            InputValue::A => Self {
                a: n + 1..=*self.a.end(),
                ..self.clone()
            },
            InputValue::S => Self {
                s: n + 1..=*self.s.end(),
                ..self.clone()
            },
        }
    }
}

fn find_workflow<'w>(name: &str, wfs: &'w [Workflow]) -> &'w Workflow {
    wfs.iter()
        .find(|it| it.id == name)
        .expect("existing workflow")
}

impl Input {
    const fn rating(&self) -> usize {
        self.x + self.m + self.a + self.s
    }

    const fn take_value(&self, iv: InputValue) -> usize {
        match iv {
            InputValue::X => self.x,
            InputValue::M => self.m,
            InputValue::A => self.a,
            InputValue::S => self.s,
        }
    }

    fn is_accepted_by(&self, current: &Workflow, workflows: &[Workflow]) -> bool {
        let eval_next_state = |state: &str| -> bool {
            if state == "A" {
                return true;
            } else if state == "R" {
                return false;
            }

            let next = find_workflow(state, workflows);
            self.is_accepted_by(next, workflows)
        };

        for rule in &current.rules {
            match rule {
                Rule::LT(iv, n, state) if self.take_value(*iv) < *n => {
                    return eval_next_state(state)
                }
                Rule::GT(iv, n, state) if self.take_value(*iv) > *n => {
                    return eval_next_state(state)
                }
                Rule::Next(state) => return eval_next_state(state),
                _ => {}
            };
        }

        false
    }

    fn is_accepted(&self, workflows: &[Workflow]) -> bool {
        let workflow = find_workflow("in", workflows);
        self.is_accepted_by(workflow, workflows)
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ParsedInput {
    workflows: Vec<Workflow>,
    inputs: Vec<Input>,
}

fn parse_rules(s: &str) -> Vec<Rule> {
    s.split(',').fold(vec![], |mut acc, rule| {
        let r = rule.parse::<Rule>().expect("rule");
        acc.push(r);
        acc
    })
}

fn parse_workflows(s: &str) -> Vec<Workflow> {
    s.lines().fold(vec![], |mut acc, line| {
        let (id, mut rest) = line.split_once('{').expect("opening {");
        rest = rest.strip_suffix('}').expect("closing }");

        let rules = parse_rules(rest);

        acc.push(Workflow {
            id: id.to_string(),
            rules,
        });

        acc
    })
}

#[aoc_generator(day19)]
pub fn generate(inp: &str) -> ParsedInput {
    let (rules, values) = inp.split_once("\n\n").expect("input");

    let workflows = parse_workflows(rules);

    let inputs = values
        .lines()
        .map(|it| it.parse().expect("input"))
        .collect_vec();

    ParsedInput { workflows, inputs }
}

#[aoc(day19, part1)]
pub fn part1(inp: &ParsedInput) -> usize {
    inp.inputs.iter().fold(0, |acc, it| {
        acc + if it.is_accepted(&inp.workflows) {
            it.rating()
        } else {
            0
        }
    })
}

#[allow(clippy::range_minus_one)]
fn count_combinations(mut range: InputRange, cur_wf: &Workflow, wfs: &[Workflow]) -> usize {
    let mut result = 0;

    let count_sub_range = |sub_range: InputRange, state: &str| -> usize {
        if sub_range.count() > 0 {
            if state == "A" {
                return sub_range.count();
            } else if state != "R" {
                let next_wf = find_workflow(state, wfs);
                return count_combinations(sub_range, next_wf, wfs);
            }
        }

        0
    };

    for r in &cur_wf.rules {
        match r {
            Rule::LT(v, n, state) => {
                let sub_range = range.sub_range_less(*v, *n);
                result += count_sub_range(sub_range, state);
                range = range.sub_range_greater(*v, *n - 1);
            }
            Rule::GT(v, n, state) => {
                let sub_range = range.sub_range_greater(*v, *n);
                result += count_sub_range(sub_range, state);
                range = range.sub_range_less(*v, *n + 1);
            }
            Rule::Next(state) => {
                if state == "R" {
                    return result;
                } else if state == "A" {
                    return result + range.count();
                }

                let next_wf = find_workflow(state, wfs);
                result += count_combinations(range.clone(), next_wf, wfs);
            }
        };
    }

    result
}

#[aoc(day19, part2)]
pub fn part2(inp: &ParsedInput) -> usize {
    let input_range = InputRange {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };

    let cur_wf = find_workflow("in", &inp.workflows);
    count_combinations(input_range, cur_wf, &inp.workflows)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}\n\
                              pv{a>1716:R,A}\n\
                              lnx{m>1548:A,A}\n\
                              rfg{s<537:gd,x>2440:R,A}\n\
                              qs{s>3448:A,lnx}\n\
                              qkq{x<1416:A,crn}\n\
                              crn{x>2662:A,R}\n\
                              in{s<1351:px,qqz}\n\
                              qqz{s>2770:qs,m<1801:hdj,R}\n\
                              gd{a>3333:R,R}\n\
                              hdj{m>838:A,pv}\n\
                              \n\
                              {x=787,m=2655,a=1222,s=2876}\n\
                              {x=1679,m=44,a=2067,s=496}\n\
                              {x=2036,m=264,a=79,s=2244}\n\
                              {x=2461,m=1339,a=466,s=291}\n\
                              {x=2127,m=1623,a=2188,s=1013}";
    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 19114);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 167_409_079_868_000);
    }
}
