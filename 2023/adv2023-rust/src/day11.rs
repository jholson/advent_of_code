use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
// TODO: ^ Why BufRead if it's not used? What's self, just the io namespace?

fn main() {
    println!("Parse image");
    let img = parse_input();
    println!("  {} x {}", img.len(), img[0].len());

    // Expand image
    println!("Expand image");
    let img = expand_img(img);
    println!("  {} x {}", img.len(), img[0].len());

    // Find coords of galaxies
    println!("Find coords of galaxies");
    let mut galaxies = Vec::new();

    for i in 0..img.len() {
        for j in 0..img[0].len() {
            if img[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    // Find distances between each pair of galaxies
    println!("Find distances between each pair of galaxies");
    let mut distances: HashMap<String, usize> = HashMap::new();
    for x in 0..galaxies.len() {
        for y in x+1..galaxies.len() {
            let src = galaxies[x];
            let dest = galaxies[y];
            let key = get_pair_key_str(&src, &dest);

            // Manhattan distance
            let dist = abs_diff(src.0, dest.0) + abs_diff(src.1, dest.1);
            distances.insert(key, dist);
        }
    }
    // println!("  {:?}", distances);
    // println!("  {:?}", distances.values());

    println!("{}", distances.values().sum::<usize>());
}

fn expand_img(img: Vec<Vec<char>>) -> Vec<Vec<char>> {
    // Find indexes of empty rows...
    let mut empty_rows = HashSet::new();
    for (i, row) in img.iter().enumerate() {
        if row.iter().all(|&e| e == '.') {
            empty_rows.insert(i);
        }
    }

    // ...and empty columns
    let mut empty_cols = HashSet::new();
    for j in 0..img[0].len() {
        if img.iter().all(|row| row[j] == '.') {
            empty_cols.insert(j);
        }
    }

    let mut new_img: Vec<Vec<char>> = Vec::new();
    for i in 0..img.len() {
        let mut row = Vec::new();

        for j in 0..img[0].len() {
            row.push(img[i][j]);
            if empty_cols.contains(&j) {
                row.push(img[i][j]);
            }
        }

        if empty_rows.contains(&i) {
            let row_copy = row.to_vec();
            new_img.push(row);
            new_img.push(row_copy);
        } else {
            new_img.push(row);
        }
    }

    return new_img;
}

fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        return a - b;
    } else {
        return b - a;
    }
}

fn get_key_str(galaxy: &(usize, usize)) -> String {
    return format!("{}_{}", galaxy.0, galaxy.1);
}

fn get_pair_key_str(galaxy1: &(usize, usize), galaxy2: &(usize, usize)) -> String {
    if galaxy1 < galaxy2 {
        return format!("{}_{}__{}_{}", galaxy1.0, galaxy1.1, galaxy2.0, galaxy2.1);
    } else {
        return format!("{}_{}__{}_{}", galaxy2.0, galaxy2.1, galaxy1.0, galaxy1.1);
    }
}

fn parse_input() -> Vec<Vec<char>> {
    let mut lines: Vec<Vec<char>> = Vec::new();

    for line in io::stdin().lock().lines() {
        lines.push(line.unwrap().chars().collect());
    }

    return lines;
}
