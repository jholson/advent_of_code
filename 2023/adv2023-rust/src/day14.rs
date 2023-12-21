use std::io::{self, BufRead};

fn main() {
    let result = part1();
    // let result = part2();

    println!("{}", result);
}

#[allow(dead_code)]
fn part1() -> usize {
    let pattern = parse_input();
    let num_rows = pattern.len();
    let num_cols = pattern[0].len();

    let mut load = 0;
    for j in 0..num_cols {
        // 'stop' is what row index a northward sliding rock would stop on
        let mut stop = 0;
        for i in 0..num_rows {
            match pattern[i][j] {
                '#' => {
                    stop = i + 1;
                },
                'O' => {
                    // Rock slides northward and stops on 'stop'
                    load += num_cols - stop;
                    stop += 1;
                },
                _ => {},
            }
        }
    }

    return load;
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
