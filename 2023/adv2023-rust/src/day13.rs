use std::io::{self, BufRead};
use std::iter::zip;

fn main() {
    // let result = part1();
    let result = part2();

    println!("{}", result);
}

#[allow(dead_code)]
fn part1() -> usize {
    get_reflection_totals(0)
}

fn part2() -> usize {
    get_reflection_totals(1)
}

fn get_reflection_totals(defects: usize) -> usize {
    let patterns = parse_input();

    let mut total = 0;
    for pattern in patterns {
        if let Some(hor_rows) = find_horizontal_reflection(&pattern, defects) {
            total += 100 * hor_rows;
        } else if let Some(vert_rows) = find_vertical_reflection(&pattern, defects) {
            total += vert_rows;
        }
    }

    return total;
}

fn find_horizontal_reflection(pattern: &Vec<Vec<char>>, defects: usize) -> Option<usize> {
    for i in 0..(pattern.len() - 1) {
        // Test for a mirror after row i
        if zip(
                pattern[..i+1].into_iter().rev(),
                pattern[i+1..].into_iter()
            )
            .map(|(row1, row2)| row1.into_iter()
                .zip(row2.into_iter())
                .filter(|(a, b)| a != b)
                .count()
            )
            .sum::<usize>() == defects
        {
            return Some(i + 1);
        }
    }

    return None;
}

fn find_vertical_reflection(pattern: &Vec<Vec<char>>, defects: usize) -> Option<usize> {
    let num_cols = pattern[0].len();
    for j in 0..(num_cols - 1) {
        // Test for a mirror after column j 
        if zip(
                (0..j+1).into_iter().rev(),
                (j+1..num_cols).into_iter(),
            )
            .map(|(j1, j2)| pattern.into_iter()
                .filter(|row| row[j1] != row[j2])
                .count()
            )
            .sum::<usize>() == defects
        {
            return Some(j + 1);
        }
    }

    return None;
}

fn parse_input() -> Vec<Vec<Vec<char>>> {
    let mut patterns: Vec<Vec<Vec<char>>> = Vec::new();

    let mut this_pattern: Vec<Vec<char>> = Vec::new();
    for line in io::stdin().lock().lines() {
        match line.unwrap().trim() {
            "" => {
                patterns.push(this_pattern);
                this_pattern = Vec::new();
            },
            l => {
                this_pattern.push(l.chars().collect());
            }
        }
    }

    if this_pattern.len() > 0 {
        patterns.push(this_pattern);
    }

    return patterns;
}
