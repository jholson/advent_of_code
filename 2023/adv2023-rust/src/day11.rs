use std::cmp::{min, max};
use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    // let result = part1();
    let result = part2();

    println!("{result}");
}

#[allow(dead_code)]
fn part1() -> usize {
    println!("Parse image");
    let img = parse_input();
    println!("  {} x {}", img.len(), img[0].len());

    // Expand image
    println!("Expand image");
    let img = expand_img(img);
    println!("  {} x {}", img.len(), img[0].len());

    // Find coords of galaxies
    println!("Find coords of galaxies");
    let galaxies = find_galaxies(&img);

    // Find distances between each pair of galaxies
    println!("Find distances between each pair of galaxies");
    // let mut total_dist = 0;
    // for x in 0..galaxies.len() {
    //     for y in x+1..galaxies.len() {
    //         let src = galaxies[x];
    //         let dest = galaxies[y];

    //         // Manhattan distance
    //         total_dist += src.0.abs_diff(dest.0) + src.1.abs_diff(dest.1);
    //     }
    // }
    // println!("  {:?}", distances);
    // println!("  {:?}", distances.values());

    let total_dist = galaxies
        .iter()
        .enumerate()
        .flat_map(|(i, &src)| galaxies
            .iter()
            .skip(i + 1)
            .map(move |&dest| src.0.abs_diff(dest.0) + src.1.abs_diff(dest.1))
        )
        .sum();

    return total_dist;
}

fn part2() -> usize {
    println!("Parse image");
    let img = parse_input();
    println!("  {} x {}", img.len(), img[0].len());

    let (empty_rows, empty_cols) = find_empty_rows_and_cols(&img);

    // Find coords of galaxies
    println!("Find coords of galaxies");
    let galaxies = find_galaxies(&img);

    // Find distances between each pair of galaxies
    println!("Find distances between each pair of galaxies");
    let mut total_dist = 0;
    let expansion_factor = 1_000_000;
    for x in 0..galaxies.len() {
        for y in x+1..galaxies.len() {
            let src = galaxies[x];
            let dest = galaxies[y];

            // Manhattan distance
            total_dist += src.0.abs_diff(dest.0) + src.1.abs_diff(dest.1);
            let num_empty_rows = empty_rows
                .iter()
                .filter(|&r| min(src.0, dest.0) < *r && *r < max(src.0, dest.0))
                .count();
            let num_empty_cols = empty_cols
                .iter()
                .filter(|&r| min(src.1, dest.1) < *r && *r < max(src.1, dest.1))
                .count();

            total_dist += (num_empty_rows + num_empty_cols) * (expansion_factor - 1);
        }
    }
    // println!("  {:?}", distances);
    // println!("  {:?}", distances.values());

    return total_dist;
}

fn find_empty_rows_and_cols(img: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    /*
    Returns vectors of row indexes and column indexes (respectively) that correspond to empty
    rows/columns. Each vector sorted in ascending order.
    */

    let empty_rows = (0..img.len())
        .into_iter()
        .filter(|&i| img[i].iter().all(|&e| e == '.'))
        .collect();

    let empty_cols = (0..img[0].len())
        .into_iter()
        .filter(|&j| img.iter().all(|row| row[j] == '.'))
        .collect();
        
    return (empty_rows, empty_cols);
}

fn find_galaxies(img: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    return img
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row
            .iter()
            .enumerate()
            .filter(|(_, &e)| e == '#')
            .map(move |(j, _)| (i, j))
        )
        .collect();
}

fn expand_img(img: Vec<Vec<char>>) -> Vec<Vec<char>> {
    /*
    Return a new image, with each empty row/column *doubled*
    */
    let (empty_rows, empty_cols) = find_empty_rows_and_cols(&img);
    let empty_rows_set: HashSet<usize> = HashSet::from_iter(empty_rows);
    let empty_cols_set: HashSet<usize> = HashSet::from_iter(empty_cols);

    let mut new_img: Vec<Vec<char>> = Vec::new();
    for i in 0..img.len() {
        let mut row = Vec::new();

        for j in 0..img[0].len() {
            row.push(img[i][j]);
            if empty_cols_set.contains(&j) {
                row.push(img[i][j]);
            }
        }

        if empty_rows_set.contains(&i) {
            let row_copy = row.to_vec();
            new_img.push(row);
            new_img.push(row_copy);
        } else {
            new_img.push(row);
        }
    }

    return new_img;
}

fn parse_input() -> Vec<Vec<char>> {
    return io::stdin().lock().lines()
        .into_iter()
        .map(|line| line.unwrap().chars().collect())
        .collect();
}
