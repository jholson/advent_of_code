use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let result = part1();
    // let result = part2();

    println!("{result}");
}

#[derive(Clone)]
#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

struct Beam {
    row: usize,
    col: usize,
    dir: Dir,
}

#[allow(dead_code)]
fn part1() -> usize {
    let grid = parse_input();
    let mut energized: Vec<Vec<Vec<Dir>>> = vec![vec![vec![]; grid[0].len()]; grid.len()];
    let mut beams = vec![Beam { row: 0, col: 0, dir: Dir::Right }];

    while let Some(beam) = beams.pop() {
        if beam.row < 0 || beam.row >= grid.len() || beam.col < 0 || beam.col >= grid[0].len() {
            // Beam out of bounds, ignore
            continue;
        }

        if energized[beam.row][beam.col].contains(&beam.dir) {
            // No need to continue propagating beam, this direction was already hit on this square
            continue;
        }

        energized[beam.row][beam.col].push(beam.dir.clone());

        let grid_square = grid[beam.row][beam.col];

        match beam.dir {
            Dir::Up => {
                match grid_square {
                    '.' | '|' => {
                        beams.push(up(&beam));
                    },
                    '/' => {
                        beams.push(right(&beam));
                    },
                    '\\' => {
                        beams.push(left(&beam));
                    },
                    '-' => {
                        beams.push(left(&beam));
                        beams.push(right(&beam));
                    },
                    _ => {},
                }
            },
            Dir::Down => {
                match grid_square {
                    '.' | '|' => {
                        beams.push(down(&beam));
                    },
                    '/' => {
                        beams.push(left(&beam));
                    },
                    '\\' => {
                        beams.push(right(&beam));
                    },
                    '-' => {
                        beams.push(left(&beam));
                        beams.push(right(&beam));
                    },
                    _ => {},
                }
            },
            Dir::Left => {
                match grid_square {
                    '.' | '-' => {
                        beams.push(left(&beam));
                    },
                    '/' => {
                        beams.push(down(&beam));
                    },
                    '\\' => {
                        beams.push(up(&beam));
                    },
                    '|' => {
                        beams.push(up(&beam));
                        beams.push(down(&beam));
                    },
                    _ => {},
                }
            },
            Dir::Right => {
                match grid_square {
                    '.' | '-' => {
                        beams.push(right(&beam));
                    },
                    '/' => {
                        beams.push(up(&beam));
                    },
                    '\\' => {
                        beams.push(down(&beam));
                    },
                    '|' => {
                        beams.push(up(&beam));
                        beams.push(down(&beam));
                    },
                    _ => {},
                }
            },
        }
    }

    return energized
        .into_iter()
        .flat_map(|row| row.into_iter().filter(|e| e.len() > 0))
        .count();
}


fn up(beam: &Beam) -> Beam {
    Beam { row: beam.row - 1, col: beam.col, dir: Dir::Up }
}

fn down(beam: &Beam) -> Beam {
    Beam { row: beam.row + 1, col: beam.col, dir: Dir::Down }
}

fn left(beam: &Beam) -> Beam {
    Beam { row: beam.row, col: beam.col - 1, dir: Dir::Left }
}

fn right(beam: &Beam) -> Beam {
    Beam { row: beam.row, col: beam.col + 1, dir: Dir::Right }
}

#[allow(dead_code)]
fn part2() -> usize {
    return 0;
}

fn parse_input() -> Vec<Vec<char>> {
    io::stdin().lock().lines()
        .into_iter()
        .filter_map(|line| {
            match line.unwrap().trim() {
                "" => { None },
                l => { Some(l.chars().collect()) }
            }
        })
        .collect()
}
