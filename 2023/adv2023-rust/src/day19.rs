use std::cmp::{max,min};
use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // let result = part1();
    let result = part2();

    println!("{result}");
}

#[derive(Debug)]
struct Workflow {
    // name: String,
    steps: Vec<Step>,
}

#[derive(Debug)]
enum Op {
    Lt,
    Gt,
}

#[derive(Clone, Debug)]
enum Action {
    Accept,
    Reject,
    ToWorkflow(String),
}

#[derive(Debug)]
enum Step {
    PerformConditionally { var: String, op: Op, operand: usize, action: Action },
    Perform(Action),
}

#[derive(Clone, Debug)]
struct RangeInclusive {
    min: usize,
    max: usize,
}

#[allow(dead_code)]
fn part1() -> usize {
    let (workflows, parts) = parse_input();

    // println!("{:?}", workflows);
    // println!("{:?}", parts);

    let mut total = 0;

    for part in parts {
        // Evaluate this part in workflows
        let mut workflow = &workflows["in"];
        let part_accepted: bool = loop {
            // Evaluate which action to take based on steps
            let mut step_idx = 0;
            let action: &Action = loop {
                let step = &workflow.steps[step_idx];
                
                match step {
                    Step::Perform(action) => { break action; },
                    Step::PerformConditionally { var, op, operand, action } => {
                        let val = part[var];
                        let value_satisfies_condition = match op {
                            Op::Lt => { val < *operand },
                            Op::Gt => { val > *operand },
                        };

                        if value_satisfies_condition {
                            break action;
                        }
                    },
                }

                step_idx += 1;
            };

            match action {
                Action::Accept => { break true; },
                Action::Reject => { break false; },
                Action::ToWorkflow(workflow_name) => { workflow = &workflows[workflow_name]; },
            }
        };

        if part_accepted {
            total += part.values().sum::<usize>();
        }
    }

    return total;
}

#[allow(dead_code)]
fn part2() -> usize {
    let (workflows, _) = parse_input();

    let mut all_ranges = HashMap::new();
    for var in ["x", "m", "a", "s"] {
        all_ranges.insert(var.to_string(), RangeInclusive { min: 1, max: 4000 });
        // all_ranges.insert(var.to_string(), Vec::new());
    }

    return calculate_total_accepted(&workflows, &"in".to_string(), 0, all_ranges);
}

fn calculate_total_accepted(
    workflows: &HashMap<String, Workflow>,
    workflow_name: &String,
    step_idx: usize,
    ranges: HashMap<String, RangeInclusive>,
) -> usize {
    /*
    Recursive
    */
    let workflow = &workflows[workflow_name];
    let step = &workflow.steps[step_idx];

    return match step {
        Step::Perform(action) => {
            // Base case: Step matches everything unconditionally, so just sum entire range for
            //  total
            match action {
                Action::Accept => { total_combos_in_ranges(&ranges) },
                Action::Reject => { 0 },
                Action::ToWorkflow(workflow_name) => {
                    // Go to new workflow
                    calculate_total_accepted(workflows, workflow_name, 0, ranges)
                },
            }
        },
        Step::PerformConditionally { var, op, operand, action } => {
            let orig_range = &ranges[var];

            let (succeed_ranges, fail_ranges) = match op {
                Op::Lt => {
                    // val < *operand

                    // Condition success
                    let mut succeed_ranges = ranges.clone();
                    *succeed_ranges.get_mut(var).unwrap() = RangeInclusive {
                        min: orig_range.min,
                        max: min(orig_range.max, *operand - 1),
                    };

                    // Condition fail
                    let mut fail_ranges = ranges.clone();
                    *fail_ranges.get_mut(var).unwrap() = RangeInclusive {
                        min: max(orig_range.min, *operand),
                        max: orig_range.max,
                    };

                    (succeed_ranges, fail_ranges)
                },
                Op::Gt => {
                    // val > *operand

                    // Condition success
                    let mut succeed_ranges = ranges.clone();
                    *succeed_ranges.get_mut(var).unwrap() = RangeInclusive {
                        min: max(orig_range.min, *operand + 1),
                        max: orig_range.max,
                    };

                    // Condition fail
                    let mut fail_ranges = ranges.clone();
                    *fail_ranges.get_mut(var).unwrap() = RangeInclusive {
                        min: orig_range.min,
                        max: min(orig_range.max, *operand),
                    };

                    (succeed_ranges, fail_ranges)
                },
            };

            let mut total = 0;

            // Evaluate the successful branch
            if ranges_are_valid(&succeed_ranges) {
                total += match action {
                    Action::Accept => { total_combos_in_ranges(&succeed_ranges) },
                    Action::Reject => { 0 },
                    Action::ToWorkflow(workflow_name) => {
                        // Go to new workflow
                        calculate_total_accepted(workflows, workflow_name, 0, succeed_ranges)
                    },
                };
            }

            // Evaluate the failure branch (i.e. moving to the next step in the workflow)
            if ranges_are_valid(&fail_ranges) && step_idx + 1 < workflow.steps.len() {
                total += calculate_total_accepted(
                    workflows,
                    workflow_name,
                    step_idx + 1,
                    fail_ranges,
                );
            }

            return total;
        },
    }
}

fn total_combos_in_ranges(ranges: &HashMap<String, RangeInclusive>) -> usize {
    ranges.values().map(|r| r.max - r.min + 1).product()
}

fn ranges_are_valid(ranges: &HashMap<String, RangeInclusive>) -> bool {
    ranges.values().all(|r| r.min <= r.max)
}

fn parse_input() -> (HashMap<String, Workflow>, Vec<HashMap<String, usize>>) {
    let lines: Vec<String> = io::stdin().lock().lines()
        .into_iter()
        .map(|line| line.unwrap().trim().to_string())
        .collect();

    let workflows = lines
        .iter()
        .take_while(|line| line.len() > 1)
        .map(|line| {
            let (name, steps_str) = line.split_once("{").unwrap();
            let steps = steps_str.trim_matches(|c| c == '{' || c == '}').split(",")
                .map(|step_str| {
                    match step_str.rsplit_once(":") {
                        Some((step_str, action_str)) => {
                            let action = match action_str {
                                "A" => { Action::Accept },
                                "R" => { Action::Reject },
                                name => { Action::ToWorkflow(name.to_string()) },
                            };

                            let op_idx = step_str.find(|c| c == '<' || c == '>').unwrap();
                            let op = match &step_str[op_idx..op_idx + 1] {
                                "<" => { Op::Lt },
                                ">" => { Op::Gt },
                                _ => { panic!() },
                            };
                            let var = &step_str[0..op_idx];
                            let operand = step_str[op_idx + 1..].parse::<usize>().unwrap();

                            Step::PerformConditionally { var: var.to_string(), op: op, operand: operand, action: action }
                        },
                        _ => {
                            let action = match step_str {
                                "A" => { Action::Accept },
                                "R" => { Action::Reject },
                                name => { Action::ToWorkflow(name.to_string()) },
                            };

                            Step::Perform(action)
                        },
                    }
                })
                .collect();
            
            (name.to_string(), Workflow { steps: steps })
        })
        .collect::<HashMap<_, _>>();

    let parts = lines
        .iter()
        .skip_while(|line| line.len() > 1)
        .skip_while(|line| line.len() == 0)
        .take_while(|line| line.len() > 1)
        .map(|line| {
            line
                .trim_matches(|c| c == '{' || c == '}')
                .split(',')
                .map(|expr| {
                    let (var, val) = expr.split_once('=').unwrap();

                    (var.to_string(), val.to_string().parse::<usize>().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .collect();

    return (workflows, parts);
}
