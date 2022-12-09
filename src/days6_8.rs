use std::collections::HashSet;

pub fn day_six_p1(inputs: &str) {
    let lines: Vec<&str> = inputs.lines().collect();
    for &line in lines.iter() {
        if line.trim().len() == 0 {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let mut packet = 0;
        for n in 0..chars.len() - 3 {
            // if only present once
            let marker: HashSet<&char> = chars[n..n + 4].iter().collect();
            if marker.len() == 4 {
                packet = n + 4;
                break;
            }
        }
        println!("Found start-of-packet ending at position {:?}", packet);
    }
}

pub fn day_six_p2(inputs: &str) {
    let lines: Vec<&str> = inputs.lines().collect();
    for &line in lines.iter() {
        if line.trim().len() == 0 {
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let mut message = 0;
        for n in 0..chars.len() - 13 {
            // if only present once
            let marker: HashSet<&char> = chars[n..n + 14].iter().collect();
            if marker.len() == 14 {
                message = n + 14;
                break;
            }
        }
        println!("Found start-of-message ending at position {:?}", message);
    }
}
pub fn run_days() {
    let samples = include_str!("../inputs/06_sample.txt");
    let _inputs = include_str!("../inputs/06_input.txt");
    print!("\nRunning Day Six, Part one sample:\n");
    day_six_p1(samples);
    print!("Running Day Six, Part one Inputs: ");
    day_six_p1(_inputs);

    print!("Running Day Six, Part two sample:\n");
    day_six_p2(samples);
    print!("Running Day Six, Part two Inputs: ");
    day_six_p2(_inputs);

    /*
    let samples = include_str!("../inputs/04_sample.txt");
    let _inputs = include_str!("../inputs/04_input.txt");

    print!("\nRunning Day Four sample: ");
    day_four_p1(samples);
    print!("Running Day Four Inputs: ");
    day_four_p1(_inputs);

    let samples = include_str!("../inputs/05_sample.txt");
    let _inputs = include_str!("../inputs/05_input.txt");

    print!("\nRunning Day Five, part one sample: ");
    day_five_p1(samples);
    print!("Running Day Five, part one Inputs: ");
    day_five_p1(_inputs);
    print!("Running Day Five, part two sample: ");
    day_five_p2(samples);
    print!("Running Day Five, part two Inputs: ");
    day_five_p2(_inputs);
    */
}
