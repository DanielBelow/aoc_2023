use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day23)]
pub fn generate(inp: &str) -> Vec<Vec<char>> {
    inp.lines().map(|it| it.chars().collect()).collect()
}

fn longest_path(
    grid: &[Vec<char>],
    start: (usize, usize),
    end: (usize, usize),
    is_part_2: bool,
) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    let mut path = Vec::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    dfs(
        grid,
        start,
        end,
        &mut path,
        &mut result,
        &mut visited,
        is_part_2,
    );
    result
}

#[allow(clippy::cast_possible_wrap)]
fn dfs(
    grid: &[Vec<char>],
    (y, x): (usize, usize),
    end: (usize, usize),
    path: &mut Vec<(usize, usize)>,
    result: &mut Vec<(usize, usize)>,
    visited: &mut Vec<Vec<bool>>,
    is_part_2: bool,
) {
    path.push((y, x));
    visited[y][x] = true;

    if (y, x) == end && path.len() > result.len() {
        *result = path.clone();
    }

    if is_part_2 {
        for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let ny = (y as isize + direction.0) as usize;
            let nx = (x as isize + direction.1) as usize;

            if ny < grid.len() && nx < grid[ny].len() && grid[ny][nx] != '#' && !visited[ny][nx] {
                dfs(grid, (ny, nx), end, path, result, visited, is_part_2);
            }
        }
    } else {
        let cur = grid[y][x];
        let mut dirs = vec![];
        if cur == '.' || cur == '^' {
            dirs.push((-1, 0));
        }
        if cur == '.' || cur == 'v' {
            dirs.push((1, 0));
        }
        if cur == '.' || cur == '<' {
            dirs.push((0, -1));
        }
        if cur == '.' || cur == '>' {
            dirs.push((0, 1));
        }

        for direction in dirs {
            let ny = (y as isize + direction.0) as usize;
            let nx = (x as isize + direction.1) as usize;

            if ny < grid.len() && nx < grid[ny].len() && grid[ny][nx] != '#' && !visited[ny][nx] {
                dfs(grid, (ny, nx), end, path, result, visited, is_part_2);
            }
        }
    }

    visited[y][x] = false;
    path.pop();
}

fn find_longest_path(inp: &[Vec<char>], is_part_2: bool) -> usize {
    let start_col = inp[0]
        .iter()
        .position(|c| *c == '.')
        .expect("starting spot");

    let goal = inp
        .last()
        .expect("last row")
        .iter()
        .position(|c| *c == '.')
        .expect("finish");

    assert_eq!(inp[0][start_col], '.');
    assert_eq!(inp[inp.len() - 1][goal], '.');

    longest_path(inp, (0, start_col), (inp.len() - 1, goal), is_part_2).len() - 1
}

#[aoc(day23, part1)]
pub fn part1(inp: &[Vec<char>]) -> usize {
    find_longest_path(inp, false)
}

#[aoc(day23, part2)]
pub fn part2(inp: &[Vec<char>]) -> usize {
    find_longest_path(inp, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#.#####################\n\
                              #.......#########...###\n\
                              #######.#########.#.###\n\
                              ###.....#.>.>.###.#.###\n\
                              ###v#####.#v#.###.#.###\n\
                              ###.>...#.#.#.....#...#\n\
                              ###v###.#.#.#########.#\n\
                              ###...#.#.#.......#...#\n\
                              #####.#.#.#######.#.###\n\
                              #.....#.#.#.......#...#\n\
                              #.#####.#.#.#########v#\n\
                              #.#...#...#...###...>.#\n\
                              #.#.#v#######v###.###v#\n\
                              #...#.>.#...>.>.#.###.#\n\
                              #####v#.#.###v#.#.###.#\n\
                              #.....#...#...#.#.#...#\n\
                              #.#########.###.#.#.###\n\
                              #...###...#...#...#.###\n\
                              ###.###.#.###v#####v###\n\
                              #...#...#.#.>.>.#.>.###\n\
                              #.###.###.#.###.#.#v###\n\
                              #.....###...###...#...#\n\
                              #####################.#";

    #[test]
    fn test_p1() {
        let gen = generate(TEST_INPUT);
        let res = part1(&gen);
        assert_eq!(res, 94);
    }

    #[test]
    fn test_p2() {
        let gen = generate(TEST_INPUT);
        let res = part2(&gen);
        assert_eq!(res, 154);
    }
}
