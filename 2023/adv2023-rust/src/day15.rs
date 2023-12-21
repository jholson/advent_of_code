use std::io::{self, BufRead};

fn main() {
    let result = part1();
    // let result = part2();

    println!("{result}");
}

#[allow(dead_code)]
fn part1() -> usize {
    parse_input()
        .into_iter()
        .map(|s| hash(&s))
        .sum()
}

#[allow(dead_code)]
fn part2() -> usize {
    return 0;
}

fn hash(s: &Vec<char>) -> usize {
    let mut val = 0;

    for c in s {
        val += *c as usize;
        val *= 17;
        val = val % 256;
    }

    return val;
}

fn parse_input() -> Vec<Vec<char>> {
    io::stdin().lock().lines()
        .next()
        .unwrap()
        .expect("foo")
        .trim()
        .split(',')
        // wat
        .map(|s| String::from(s).chars().collect())
        .collect()
}
