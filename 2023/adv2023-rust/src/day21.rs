use std::io::{self, BufRead};

fn main() {
    let result = part1();
    // let result = part2();

    println!("{result}");
}

#[allow(dead_code)]
fn part1() -> usize {
    let mut grid = parse_input();
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    for n in 0..64 {
        let mut new_grid = vec![vec![' '; num_cols]; num_rows];

        for i in 0..num_rows {
            for j in 0..num_cols {
                new_grid[i][j] = match grid[i][j] {
                    '#' => { '#' },
                    _ => {
                        if get_adjacent_cells(i, j, num_rows, num_cols)
                            .iter()
                            .any(|(adj_i, adj_j)| {
                                grid[*adj_i][*adj_j] == 'O' || grid[*adj_i][*adj_j] == 'S'
                            })
                        {
                            'O'
                        } else {
                            '.'
                        }
                    },
                }
            }
        }

        grid = new_grid;
    }

    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }

    grid
        .iter()
        .map(|row| row
            .iter()
            .filter(|&c| *c == 'O')
            .count()
        )
        .sum()
}

fn get_adjacent_cells(
    row: usize,
    col: usize,
    num_rows: usize,
    num_cols: usize,
) -> Vec<(usize, usize)> {
    [
        (row - 1, col),
        (row + 1, col),
        (row, col - 1),
        (row, col + 1),
    ]
    .into_iter()
    .filter(|(i, j)| *i >= 0 && *i < num_rows && *j >= 0 && *j < num_cols)
    .collect()
}

#[allow(dead_code)]
fn part2() -> usize {
    0
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
