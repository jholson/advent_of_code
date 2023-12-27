use std::cmp::{min,max};
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let result = part1();
    // let result = part2();

    println!("{result}");
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

const ALL_DIRS: [Dir; 4] = [Dir::Up, Dir::Down, Dir::Left, Dir::Right];

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Entry {
    // Grid entry to process
    row: usize,
    col: usize,

    // Direction of incoming segment where the shortest path may have been updated
    dir: Dir,
}

#[derive(Debug)]
struct Path {
    prev_row: usize,
    prev_col: usize,
    length: usize,
}

#[allow(dead_code)]
fn part1() -> usize {
    let grid = parse_input();
    let num_rows = grid.len();
    let num_cols = grid[0].len();

    // shortest[2][4][Dir::Down] is the shortest path to square (2,4) if the immediately previous
    //  path segment was 1-3 consecutive downs. The shortest value includes the square in question
    let mut shortest: HashMap<Entry, Path> = HashMap::new();

    let mut in_queue: HashSet<Entry> = HashSet::new();
    let mut queue = VecDeque::new();

    // Initialize structures for 1-3 squares down and 1-3 squares right of the origin
    for entry in get_impacted_cells(0, 0, None, num_rows, num_cols) {
        for next in get_impacted_cells(
            entry.row,
            entry.col,
            Some(&entry.dir),
            num_rows,
            num_cols,
        )
        {
            if !in_queue.contains(&next) {
                queue.push_back(next.clone());
                in_queue.insert(next);
            }
        }

        let val = get_path_sum_between(&grid, 0, 0, entry.row, entry.col);
        shortest.insert(entry, Path { prev_row: 0, prev_col: 0, length: val });
    }

    while let Some(entry) = queue.pop_front() {
        in_queue.remove(&entry);

        /* Calculate this grid square/direction's shortest path based on the direction the beam
         *  came from
         *
         *  shortest[row][col][down] = min(
         *      shortest[{row-1,row-2,row-3}][col][{left,right}] + cell_values_in_between
         *  )
         */
        let new_min_length = get_cells_in_direction(
                entry.row,
                entry.col,
                opposite(entry.dir),
                num_rows,
                num_cols,
            )
            .into_iter()
            .filter_map(|(row, col)| {
                let path = orthogonal(entry.dir)
                    .into_iter()
                    .filter_map(|d| shortest.get(&Entry { row: row, col: col, dir: d }))
                    .min_by_key(|p| p.length);
                if let Some(&ref path) = path {
                    Some(Path {
                        prev_row: row,
                        prev_col: col,
                        length: path.length + get_path_sum_between(
                            &grid,
                            row,
                            col,
                            entry.row,
                            entry.col,
                        ),
                    })
                } else {
                    None
                }
            })
            .min_by_key(|p| p.length);

        if let Some(new_min_length) = new_min_length {
            let old_min_length = shortest.get(&entry);
            if old_min_length.is_none() || new_min_length.length < (*old_min_length.unwrap()).length {
                // If the shortest path changed, find the possibly-impacted descendant squares and
                //  put them on the queue
                for next in get_impacted_cells(
                    entry.row,
                    entry.col,
                    Some(&entry.dir),
                    num_rows,
                    num_cols,
                )
                {
                    if !in_queue.contains(&next) {
                        queue.push_back(next.clone());
                        in_queue.insert(next);
                    }
                }

                shortest.insert(entry, new_min_length);
            }
        }
    }

    let final_shortest_path = ALL_DIRS
        .into_iter()
        .filter_map(|d| shortest.get(&Entry {
            row: grid.len() - 1,
            col: grid[0].len() - 1,
            dir: d,
        }))
        .min_by_key(|p| p.length)
        .unwrap();

    return final_shortest_path.length;
}

#[allow(dead_code)]
fn part2() -> usize {
    return 0;
}

fn get_path_sum_between(
    grid: &Vec<Vec<usize>>,
    row1: usize,
    col1: usize,
    row2: usize,
    col2: usize,
) -> usize {
    /*
    Return the sum of the cells in the straight path between (row1, col1) and (row2, col2), not
    including (row1, col1)'s path value, but including (row2, col2)'s path value
    */
    if row1 == row2 {
        // Going left or right
        if col1 < col2 {
            grid[row1][col1 + 1..=col2].into_iter().sum()
        } else {
            grid[row1][col2..col1].into_iter().sum()
        }
    } else if col1 == col2 {
        // Going up or down
        if row1 < row2 {
            grid[row1 + 1..=row2].into_iter().map(|row| row[col1]).sum()
        } else {
            grid[row2..row1].into_iter().map(|row| row[col1]).sum()
        }
    } else {
        panic!();
    }
}

fn orthogonal(dir: Dir) -> Vec<Dir> {
    ALL_DIRS
        .into_iter()
        .filter(|d| *d != dir && *d != opposite(dir))
        .collect()
}

fn get_impacted_cells(
    row: usize,
    col: usize,
    prev_dir: Option<&Dir>,
    num_rows: usize,
    num_cols: usize,
) -> Vec<Entry> {
    /*
    If we just updated the shortest path for a cell (row,col) with the previous segment coming from
    'prev_dir', return a list of potential next cells, along with the direction we take to get there
    */

    if prev_dir.is_some() {
        orthogonal(*prev_dir.unwrap())
        .into_iter()
        .flat_map(|d| get_cells_in_direction(row, col, d, num_rows, num_cols)
            .into_iter()
            .map(move |(row, col)| Entry { row: row, col: col, dir: d.clone() })
        )
        .collect()
    } else {
        ALL_DIRS
        .iter()
        .flat_map(|d| get_cells_in_direction(row, col, *d, num_rows, num_cols)
            .into_iter()
            .map(|(row, col)| Entry { row: row, col: col, dir: d.clone() })
        )
        .collect()
    }
}

fn opposite(
    dir: Dir,
) -> Dir {
    match dir {
        Dir::Up => { Dir::Down },
        Dir::Down => { Dir::Up },
        Dir::Left => { Dir::Right },
        Dir::Right => { Dir::Left },
    }
}

fn get_cells_in_direction(
    row: usize,
    col: usize,
    dir: Dir,
    num_rows: usize,
    num_cols: usize,
) -> Vec<(usize, usize)> {
    match dir {
        Dir::Up => {
            (4..=10)
                .filter_map(|x| row.checked_sub(x))
                .map(|r| (r, col))
                .collect()
        },
        Dir::Down => { 
            (4..=10)
                .filter_map(|x| if row + x < num_rows { Some(row + x) } else { None })
                .map(|r| (r, col))
                .collect()
        },
        Dir::Left => {
            (4..=10)
                .filter_map(|x| col.checked_sub(x))
                .map(|c| (row, c))
                .collect()
        },
        Dir::Right => {
            (4..=10)
                .filter_map(|x| if col + x < num_cols { Some(col + x) } else { None })
                .map(|c| (row, c))
                .collect()
        },
    }
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
