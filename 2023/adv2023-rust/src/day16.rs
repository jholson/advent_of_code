use std::io::{self, BufRead};

fn main() {
    // let result = part1();
    let result = part2();

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
    row: isize,
    col: isize,
    dir: Dir,
}

#[allow(dead_code)]
fn part1() -> usize {
    let grid = parse_input();
    let start_beam = Beam { row: 0, col: 0, dir: Dir::Right };

    count_energized(&grid, start_beam)
}

#[allow(dead_code)]
fn count_energized(grid: &Vec<Vec<char>>, start_beam: Beam) -> usize {
    let mut energized: Vec<Vec<Vec<Dir>>> = vec![vec![vec![]; grid[0].len()]; grid.len()];
    let mut beams = vec![start_beam];

    while let Some(beam) = beams.pop() {
        if beam.row < 0
            || beam.row >= grid.len().try_into().unwrap()
            || beam.col < 0
            || beam.col >= grid[0].len().try_into().unwrap()
        {
            // Beam out of bounds, ignore
            continue;
        }

        let energized_dirs = &mut energized[beam.row as usize][beam.col as usize];
        if energized_dirs.contains(&beam.dir) {
            // No need to continue propagating beam, this direction was already hit on this square
            continue;
        }

        energized_dirs.push(beam.dir.clone());

        let grid_square = grid[beam.row as usize][beam.col as usize];

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

    energized
        .into_iter()
        .flat_map(|row| row.into_iter().filter(|e| e.len() > 0))
        .count()
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
    let grid = parse_input();
    let num_rows = grid.len() as isize;
    let num_cols = grid[0].len() as isize;

    (0..num_rows)
        .flat_map(|row| [
            Beam { row: row, col: 0, dir: Dir::Right },
            Beam { row: row, col: num_cols - 1, dir: Dir::Left },
        ])
        .chain(
            (0..num_cols)
                .flat_map(|col| [
                    Beam { row: 0, col: col, dir: Dir::Down },
                    Beam { row: num_rows - 1, col: col, dir: Dir::Up },
                ])
        )
        .map(|beam| count_energized(&grid, beam))
        .max()
        .unwrap()
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
