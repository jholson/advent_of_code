use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let result = part1();
    // let result = part2();

    println!("{result}");
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Entry {
    // Grid entry to process
    row: usize,
    col: usize,

    // Direction of incoming segment where the shortest path may have been updated
    dir: Dir,
}


#[allow(dead_code)]
fn part1() -> usize {
    let grid = parse_input();
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let all_dirs = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

    // shortest[2][4][Dir::Down] is the shortest path to square (2,4) if the immediately previous
    //  path segment was 1-3 consecutive downs. The shortest value includes the square in question
    let mut shortest: Vec<Vec<HashMap<Dir, usize>>> = vec![
        vec![HashMap::new(); num_cols];
        num_rows
    ];

    // TODO: Stack/queue? Also, do we need a 'seen' set to avoid adding the same thing into the
    //  stack?
    let mut stack = Vec::new();

    for row in 1..=3 {
        shortest[row][0].insert(Dir::Down, grid[row][0]);
        stack.push(Entry { row: row, col: 0, dir: Dir::Down });
    }
    for col in 1..=3 {
        shortest[0][col].insert(Dir::Right, grid[0][col]);
        stack.push(Entry { row: 0, col: col, dir: Dir::Right });
    }

    while let Some(entry) = stack.pop() {
        // Calculate this grid square/direction's shortest path based on the direction the beam
        //  came from

        // If the shortest path changed, find the possibly-impacted descendant squares and put them
        //  on the stack
    }

    return 0;
}

#[allow(dead_code)]
fn part2() -> usize {
    return 0;
}

fn parse_input() -> Vec<Vec<usize>> {
    io::stdin().lock().lines()
        .into_iter()
        .filter_map(|line| {
            match line.unwrap().trim() {
                "" => { None },
                l => { Some(l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()) }
            }
        })
        .collect()
}
