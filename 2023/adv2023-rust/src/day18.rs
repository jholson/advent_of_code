use std::cmp::{min, max};
use std::io::{self, BufRead};

fn main() {
    // let result = part1();
    let result = part2();

    println!("{result}");
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Step {
    dir: Dir,
    dist: isize,
}

// +row = -y, +col = +x
#[derive(Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

#[allow(dead_code)]
fn part1() -> usize {
    let steps = parse_input1();
    
    find_area(&steps)
}

#[allow(dead_code)]
fn part2() -> usize {
    let steps = parse_input2();
    
    find_area(&steps)
}

fn find_area(steps: &Vec<Step>) -> usize {
    let mut points = vec![Point { x: 0, y: 0 }];

    for step in steps {
        let (delta_x, delta_y) = match step.dir {
            Dir::Up => { (0, step.dist) },
            Dir::Down => { (0, -step.dist) },
            Dir::Left => { (-step.dist, 0) },
            Dir::Right => { (step.dist, 0) },
        };

        let prev_point = &points[points.len() - 1];
        let new_point = Point { x: prev_point.x + delta_x, y: prev_point.y + delta_y };
       
        points.push(new_point);
    }

    assert!(&points[0] == &points[points.len() - 1]);

    // Trapezoid polygon area formula
    let trap_area = (0..points.len() - 1)
        .map(|i| (points[i].y + points[i + 1].y) * (points[i].x - points[i + 1].x))
        .sum::<isize>()
        .abs() / 2;

    let total_step_dist = steps.iter().map(|s| s.dist).sum::<isize>();

    // Add in an extra 0.5 times the perimeter of the shape the trapezoid formula is calculating the
    //  area for.
    // +1 on the end: Each convex corner adds 0.25 area, each concave corner subtracts 0.25 area.
    //  There will always be 4 more convex corners than concave corners, so 4 * 0.25 = 1
    let area = trap_area + total_step_dist / 2 + 1;

    // println!("Trapezoid area: {trap_area}");
    // println!("Total step dist: {total_step_dist}");
    // println!("Final area: {area}");

    return area as usize;
}

fn parse_input1() -> Vec<Step> {
    io::stdin().lock().lines()
        .into_iter()
        .filter_map(|line| {
            match line.unwrap().trim() {
                "" => { None },
                l => {
                    let parts = l.split_ascii_whitespace().collect::<Vec<_>>();
                    let dir = match parts[0] {
                        "U" => { Dir::Up },
                        "D" => { Dir::Down },
                        "L" => { Dir::Left },
                        "R" => { Dir::Right },
                        _ => { panic!() },
                    };
                    let dist = parts[1].parse::<isize>().unwrap();

                    Some(Step { dir: dir, dist: dist })
                },
            }
        })
        .collect()
}

fn parse_input2() -> Vec<Step> {
    io::stdin().lock().lines()
        .into_iter()
        .filter_map(|line| {
            match line.unwrap().trim() {
                "" => { None },
                l => {
                    let parts = l.split_ascii_whitespace().collect::<Vec<_>>();
                    // let dir = match parts[0] {
                    //     "U" => { Dir::Up },
                    //     "D" => { Dir::Down },
                    //     "L" => { Dir::Left },
                    //     "R" => { Dir::Right },
                    //     _ => { panic!() },
                    // };
                    // let dist = parts[1].parse::<isize>().unwrap();
                    let color = parts[2].trim_matches(|c| c == '(' || c == ')' || c == '#');
                    let dist = isize::from_str_radix(&color[0..5], 16).unwrap();
                    let dir = match &color[5..6] {
                        "0" => { Dir::Right },
                        "1" => { Dir::Down },
                        "2" => { Dir::Left },
                        "3" => { Dir::Up },
                        _ => { panic!() },
                    };

                    Some(Step { dir: dir, dist: dist })
                },
            }
        })
        .collect()
}
