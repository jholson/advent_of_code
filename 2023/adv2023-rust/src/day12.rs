use std::collections::HashMap;
use std::io::{self, BufRead};
use std::iter;

fn main() {
    // let rows = parse_input1();
    let rows = parse_input2();

    let result = process_rows(rows);
    println!("{}", result);
}

fn process_rows(rows: Vec<(Vec<char>, Vec<usize>)>) -> usize {
    /*
    Brute force:
    1. Generate all possible positioning of the groups
    2. Check each position against the springs to see if it's valid

    Better:
    Maybe dynamic programming?
    Define a function `num_positions(...)` that takes in `spring_start_idx` and `group_start_idx`,
    and returns the number of possible positions. Call recursively. Memoize the result.
    */
    
    rows.into_iter()
        .map(|(springs, groups)| {
            let mut cache = HashMap::new();
            num_possible_positions(&groups, 0, &springs, 0, &mut cache)
        })
        .sum()
}

// fn cache() -> &'static HashMap<String, usize> {
//     static CACHE: OnceLock<HashMap<String, usize>> = OnceLock::new();
//     CACHE.get_or_init(|| {
//         let mut m = HashMap::new();
//         m
//     });
//     CACHE.get_mut().unwrap()
// }

// lazy_static! {
//     static ref CACHE: HashMap<String, usize> = HashMap::new();
// }

fn num_possible_positions(
    all_groups: &Vec<usize>,
    group_idx: usize,
    all_springs: &Vec<char>,
    spring_idx: usize,
    cache: &mut HashMap<String, usize>,
) -> usize {
    let cache_key = key_str(group_idx, spring_idx);
    if let Some(result) = cache.get(&cache_key) {
        return *result;
    }

    assert!(group_idx < all_groups.len());
    assert!(spring_idx < all_springs.len());

    let groups = &all_groups[group_idx..];
    let springs = &all_springs[spring_idx..];
    
    let cur_group = groups[0];

    // Base case: current group doesn't fit into remaining springs
    if cur_group > springs.len() {
        cache.insert(cache_key, 0);
        return 0;
    }

    let mut positions = 0;

    let more_groups = groups.len() > 1;
    let suffix_spaces = if more_groups { 1 } else { springs.len() - cur_group };

    // Place current group starting at spring_idx
    if springs.into_iter().take(cur_group).all(|&x| x != '.')
        && springs.into_iter().skip(cur_group).take(suffix_spaces).all(|&x| x != '#')
    {
        if more_groups {
            // At least one more group after this one
            if cur_group + suffix_spaces + 1 <= springs.len() {
                positions += num_possible_positions(
                    all_groups,
                    group_idx + 1,
                    all_springs,
                    spring_idx + cur_group + suffix_spaces,
                    cache,
                )
            }
        } else {
            // Base case: This is the last group
            positions += 1
        }
    }

    // Base case: Just put a space at spring_idx
    if springs[0] != '#' {
        if springs.len() > 1 {
            positions += num_possible_positions(
                all_groups,
                group_idx,
                all_springs,
                spring_idx + 1,
                cache,
            )
        }
    }

    cache.insert(cache_key, positions);
    return positions;
}

fn key_str(group_idx: usize, spring_idx: usize) -> String {
    format!("{}_{}", group_idx, spring_idx)
}

#[allow(dead_code)]
fn parse_input1() -> Vec<(Vec<char>, Vec<usize>)> {
    let mut rows = Vec::new();
    for line in io::stdin().lock().lines() {
        match line.unwrap().trim().split_once(" ") {
            Some((springs_str, groups_str)) => {
                let springs: Vec<_> = springs_str.chars().collect();
                let groups: Vec<_> = groups_str
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();

                rows.push((springs, groups));
            },
            None => {}
        }
    }

    return rows;
}

fn parse_input2() -> Vec<(Vec<char>, Vec<usize>)> {
    let mut rows = Vec::new();
    for line in io::stdin().lock().lines() {
        match line.unwrap().trim().split_once(" ") {
            Some((springs_str, groups_str)) => {
                let springs: Vec<char> = iter::repeat(springs_str)
                    .take(5)
                    .collect::<Vec<_>>()
                    .join("?")
                    .chars()
                    .collect();
                let groups: Vec<_> = iter::repeat(groups_str)
                    .take(5)
                    .collect::<Vec<_>>()
                    .join(",")
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();

                rows.push((springs, groups));
            },
            None => {}
        }
    }

    return rows;
}
