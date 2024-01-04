use std::collections::{HashMap, VecDeque};
use std::io::{self, BufRead};

fn main() {
    let result = part1();
    // let result = part2();

    println!("{result}");
}

#[derive(Clone, Debug)]
struct Pulse {
    value: PulseStrength,
    source: String,
    destination: String,
}

#[derive(Clone, Debug, PartialEq)]
enum PulseStrength {
    Low,
    High,
}

#[derive(Clone, Debug)]
enum Module {
    Broadcast { destinations: Vec<String> },
    Flipflop { destinations: Vec<String>, on: bool },
    Conjunction { destinations: Vec<String>, last_pulse_from: HashMap<String, PulseStrength> },
}

#[allow(dead_code)]
fn part1() -> usize {
    let mut modules = parse_input();

    println!("{:?}", modules);

    let mut total_low_pulses = 0;
    let mut total_high_pulses = 0;

    for _ in 0..1000 {
        let (low_pulses, high_pulses) = push_button(&mut modules);

        total_low_pulses += low_pulses;
        total_high_pulses += high_pulses;
    }

    return total_low_pulses * total_high_pulses;
}

fn push_button(modules: &mut HashMap<String, Module>) -> (usize, usize) {
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    let mut pulses = VecDeque::new();
    pulses.push_back(Pulse {
        value: PulseStrength::Low,
        source: "button".to_string(),
        destination: "broadcaster".to_string(),
    });

    while let Some(pulse) = pulses.pop_front() {
        match pulse.value {
            PulseStrength::Low => { low_pulses += 1; },
            PulseStrength::High => { high_pulses += 1; },
        }

        println!("Processing pulse: {:?}", pulse);
        if let Some(mut module) = modules.get_mut(&pulse.destination) {
            match module {
                Module::Broadcast { destinations } => {
                    for destination in destinations {
                        pulses.push_back(Pulse {
                            value: pulse.value.clone(),
                            source: pulse.destination.clone(),
                            destination: destination.clone(),
                        });
                    }
                },
                Module::Flipflop { destinations, on } => {
                    match pulse.value {
                        PulseStrength::Low => {
                            *on = !*on;

                            let value = if *on { PulseStrength::High } else { PulseStrength::Low };

                            for destination in destinations {
                                pulses.push_back(Pulse {
                                    value: value.clone(),
                                    source: pulse.destination.clone(),
                                    destination: destination.clone(),
                                });
                            }
                        },
                        PulseStrength::High => {},
                    }
                },
                Module::Conjunction { destinations, last_pulse_from } => {
                    last_pulse_from.insert(pulse.source, pulse.value);
                    
                    let value = if last_pulse_from.values().all(|p| *p == PulseStrength::High) {
                        PulseStrength::Low
                    } else {
                        PulseStrength::High
                    };

                    for destination in destinations {
                        pulses.push_back(Pulse {
                            value: value.clone(),
                            source: pulse.destination.clone(),
                            destination: destination.clone(),
                        });
                    }

                },
            }
        }
    }

    (low_pulses, high_pulses)
}

#[allow(dead_code)]
fn part2() -> usize {
    0
}

fn parse_input() -> HashMap<String, Module> {
    let mut modules = io::stdin().lock().lines()
        .into_iter()
        .map(|line| line.unwrap().trim().to_string())
        .filter(|line| line.len() > 0)
        .map(|line| {
            let (module_str, destinations_str) = line.split_once(" -> ").unwrap();
            let module_name = module_str.trim_start_matches(|c| c == '%' || c == '&');
            let destinations = destinations_str
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            let first_char = module_str.chars().next().unwrap();
            let module = match first_char {
                '%' => { Module::Flipflop { destinations, on: false } },
                '&' => { Module::Conjunction { destinations, last_pulse_from: HashMap::new() } },
                _ => { Module::Broadcast { destinations } },
            };

            (module_name.to_string(), module)
        })
        .collect::<HashMap<_, _>>();

    let mut conj_modules_init = HashMap::new();

    for (name, module) in modules.iter() {
        // This is awkward
        let destinations = match module {
            Module::Broadcast { destinations } => { destinations },
            Module::Flipflop { destinations, .. } => { destinations },
            Module::Conjunction { destinations, .. } => { destinations },
        };

        for dest in destinations {
            match modules.get(dest) {
                Some(Module::Conjunction { .. }) => {
                    if !conj_modules_init.contains_key(dest) {
                        conj_modules_init.insert(dest.clone(), HashMap::new());
                    }
                    let mut last_pulse_from = conj_modules_init.get_mut(dest).unwrap();
                    last_pulse_from.insert(name.clone(), PulseStrength::Low);
                },
                _ => { },
            }
        }
    }

    for (name, init_last_pulse_from) in conj_modules_init {
        match modules.get_mut(&name) {
            Some(Module::Conjunction { last_pulse_from, .. }) => {
                *last_pulse_from = init_last_pulse_from;
            },
            _ => { },
        }
    }

    return modules;
}
