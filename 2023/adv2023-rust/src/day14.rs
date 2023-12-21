use sha1::{Sha1, Digest};
use std::collections::HashMap;
use std::io::{self, BufRead};


fn main() {
    // let result = part1();
    let result = part2();

    println!("{}", result);
}

#[allow(dead_code)]
fn part1() -> usize {
    let mut pattern = parse_input();
    print_pattern(&pattern);

    slide_north(&mut pattern);

    print_pattern(&pattern);

    return calc_load(&pattern);
}

#[allow(dead_code)]
fn part2() -> usize {
    let mut pattern = parse_input();
    print_pattern(&pattern);

    let mut hash = hash_pattern(&pattern);
    let mut seen_at_cycle = HashMap::new();
    let mut cycles = 0;
    seen_at_cycle.insert(hash, cycles);

    let loop_length = loop {
        cycle_once(&mut pattern);
        cycles += 1;
        
        println!("After {} cycles:", cycles);
        print_pattern(&pattern);

        hash = hash_pattern(&pattern);
        if let Some(first_seen) = seen_at_cycle.get(&hash) {
            break cycles - first_seen;
        }

        println!("{}", hash);
        println!("");

        seen_at_cycle.insert(hash, cycles);
    };

    println!("Loop length: {loop_length} Now at {cycles} cycles");
    let target = 1_000_000_000;
    let more_loops = (target - cycles) / loop_length;
    cycles += more_loops * loop_length;

    while cycles < target {
        cycle_once(&mut pattern);
        cycles += 1;

        println!("After {} cycles:", cycles);
        print_pattern(&pattern);
    }

    return calc_load(&pattern);
}

fn print_pattern(pattern: &Vec<Vec<char>>) {
    for row in pattern {
        println!("{}", row.into_iter().collect::<String>());
    }
    println!("");
}

fn hash_pattern(pattern: &Vec<Vec<char>>) -> String {
    let mut hasher = Sha1::new();

    for row in pattern {
        hasher.update(row.into_iter().collect::<String>());
    }

    return format!("{:x}", hasher.finalize());
}

fn cycle_once(pattern: &mut Vec<Vec<char>>) {
    slide_north(pattern);
    slide_west(pattern);
    slide_south(pattern);
    slide_east(pattern);
}

fn slide_north(pattern: &mut Vec<Vec<char>>) {
    let num_rows = pattern.len();
    let num_cols = pattern[0].len();

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
                    if i != stop {
                        pattern[i][j] = '.';
                        pattern[stop][j] = 'O';
                    }

                    stop += 1;
                },
                _ => {},
            }
        }
    }
}

fn slide_south(pattern: &mut Vec<Vec<char>>) {
    let num_rows = pattern.len();
    let num_cols = pattern[0].len();

    for j in 0..num_cols {
        // 'stop' is what row index a southward sliding rock would stop on
        let mut stop = num_rows - 1;
        for i in (0..num_rows).into_iter().rev() {
            match pattern[i][j] {
                '#' => {
                    stop = i - 1;
                },
                'O' => {
                    // Rock slides southward and stops on 'stop'
                    if i != stop {
                        pattern[i][j] = '.';
                        pattern[stop][j] = 'O';
                    }

                    stop -= 1;
                },
                _ => {},
            }
        }
    }
}


fn slide_west(pattern: &mut Vec<Vec<char>>) {
    let num_rows = pattern.len();
    let num_cols = pattern[0].len();

    for i in 0..num_rows {
        // 'stop' is what row index a westward sliding rock would stop on
        let mut stop = 0;
        for j in 0..num_cols {
            match pattern[i][j] {
                '#' => {
                    stop = j + 1;
                },
                'O' => {
                    // Rock slides westward and stops on 'stop'
                    if j != stop {
                        pattern[i][j] = '.';
                        pattern[i][stop] = 'O';
                    }

                    stop += 1;
                },
                _ => {},
            }
        }
    }
}

fn slide_east(pattern: &mut Vec<Vec<char>>) {
    let num_rows = pattern.len();
    let num_cols = pattern[0].len();

    for i in 0..num_rows {
        // 'stop' is what row index a eastward sliding rock would stop on
        let mut stop = num_cols - 1;
        for j in (0..num_cols).into_iter().rev() {
            match pattern[i][j] {
                '#' => {
                    stop = j - 1;
                },
                'O' => {
                    // Rock slides eastward and stops on 'stop'
                    if j != stop {
                        pattern[i][j] = '.';
                        pattern[i][stop] = 'O';
                    }

                    stop -= 1;
                },
                _ => {},
            }
        }
    }
}

fn calc_load(pattern: &Vec<Vec<char>>) -> usize {
    let num_rows = pattern.len();
    let num_cols = pattern[0].len();

    let mut load = 0;
    for j in 0..num_cols {
        // 'stop' is what row index a northward sliding rock would stop on
        for i in 0..num_rows {
            if pattern[i][j] == 'O' {
                load += num_rows - i;
            }
        }
    }

    return load;
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
