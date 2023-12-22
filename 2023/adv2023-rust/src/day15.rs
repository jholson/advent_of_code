use std::io::{self, BufRead};

fn main() {
    // let result = part1();
    let result = part2();

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
    let mut boxes: Vec<Vec<Option<Lens>>> = vec![vec![]; 256];

    for s in parse_input() {
        if let Some((label, focal_length_str)) = s.split_once('=') {
            // Set
            let focal_length = focal_length_str.parse::<usize>().unwrap();
            let new_lens = Lens { label: label.to_string(), focal_length: focal_length };

            let lens_list = &mut boxes[hash(&new_lens.label)];

            if let Some(lens_idx) = lens_list.into_iter()
                .position(|e| match e {
                    Some(lens) => { lens.label == new_lens.label },
                    None => { false },
                })
            {
                lens_list[lens_idx] = Some(new_lens);
            } else {
                lens_list.push(Some(new_lens));
            }
        } else if let Some((label, _)) = s.split_once('-') {
            // Remove
            let lens_list = &mut boxes[hash(&label.to_string())];

            if let Some(lens_idx) = lens_list.into_iter()
                .position(|e| match e {
                    Some(lens) => { lens.label == label },
                    None => { false },
                })
            {
                lens_list[lens_idx] = None;
            }
        }
    }

    let mut total = 0;
    for (box_idx, lens_list) in boxes.into_iter().enumerate() {
        for (lens_idx, lens) in lens_list.into_iter().filter_map(|e| e).enumerate() {
            total += (box_idx + 1) * (lens_idx + 1) * lens.focal_length;
        }
    }

    return total;
}

#[derive(Clone)]
#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

fn hash(s: &String) -> usize {
    let mut val = 0;

    for c in s.chars() {
        val += c as usize;
        val *= 17;
        val = val % 256;
    }

    return val;
}

fn parse_input() -> Vec<String> {
    io::stdin().lock().lines()
        .next()
        .unwrap()
        .expect("foo")
        .trim()
        .split(',')
        // wat
        .map(|s| String::from(s))
        .collect()
}
