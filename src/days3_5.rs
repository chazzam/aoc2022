use std::collections::HashSet;

fn _score_priority(item: &char) -> u32 {
    match item {
        'a'..='z' => *item as u32 - 'a' as u32 + 1,
        'A'..='Z' => *item as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

fn _pack_priority(pack: std::str::Chars, half: usize) -> u32 {
    let left = pack.clone().take(half).collect::<HashSet<_>>();
    let right = pack.skip(half).collect::<HashSet<_>>();
    left.intersection(&right).map(|x| _score_priority(x)).sum()
}

pub fn day_three_p1(inputs: &str) {
    let sum: u32 = inputs
        .lines()
        .into_iter()
        .map(|pack| _pack_priority(pack.chars(), pack.len() / 2))
        .sum();
    println!("Priority sum is {:?}", sum);
}

pub fn day_three_p2(inputs: &str) {
    let packs: Vec<&str> = inputs.lines().collect();
    let mut sum: u32 = 0;
    let mut index = 0;
    while index < packs.len() {
        if index + 3 > packs.len() {
            break;
        }
        let elf1: HashSet<char> = packs[index + 0].chars().collect();
        let elf2: HashSet<char> = packs[index + 1].chars().collect();
        let elf3: HashSet<char> = packs[index + 2].chars().collect();

        let badge_priority: u32 = elf1
            .intersection(&elf2)
            .map(|x| *x)
            .collect::<HashSet<_>>()
            .intersection(&elf3)
            .map(|x| _score_priority(x))
            .sum();

        sum += badge_priority;
        index += 3;
    }
    println!("Priority sum is {:?}", sum);
}

pub fn day_four_p1(inputs: &str) {
    let section_ranges: Vec<_> = inputs
        .lines()
        .map(|id_pair| {
            let sections: Vec<_> = id_pair
                .split(&['-', ','][..])
                .map(|x| x.parse::<i32>().expect("Sections should be integers"))
                .collect();
            (sections[0]..=sections[1], sections[2]..=sections[3])
        })
        .collect();

    let wholly_contained: u32 = section_ranges
        .iter()
        .map(|(range1, range2)| {
            if (range2.contains(range1.start()) && range2.contains(range1.end()))
                || (range1.contains(&range2.start()) && range1.contains(range2.end()))
            {
                1
            } else {
                0
            }
        })
        .sum();

    let any_overlap: u32 = section_ranges
        .iter()
        .map(|(range1, range2)| {
            if range2.contains(range1.start())
                || range2.contains(range1.end())
                || range1.contains(&range2.start())
                || range1.contains(range2.end())
            {
                1
            } else {
                0
            }
        })
        .sum();

    println!(
        "Sections wholly contained: {:?}; Sections with Overlap: {:?}",
        wholly_contained, any_overlap
    );
}
//std::str::Lines
fn _get_columns(lines: &Vec<&str>) -> usize {
    let mut columns = 0;
    // Get the row with the column numbers
    for line in lines.iter() {
        if !line.trim().starts_with("1") {
            continue;
        }
        let cols: Vec<&str> = line.trim().split("   ").collect();
        columns = cols.len();
        break;
    }
    columns
}

pub fn day_five_p1(inputs: &str) {
    let lines: Vec<&str> = inputs.lines().collect();
    let columns = _get_columns(&lines);
    let mut stacks: Vec<String> = Vec::new();
    stacks.resize(columns, String::from(""));

    for &line in lines.iter() {
        if line.trim().len() == 0 {
            continue;
        }
        // parse the crates
        if line.contains("[") {
            let big_line = String::from(line);
            // This is a crate line
            for c in 0..columns {
                // look at this line in three character blocks with a 1 char spacer
                let start = c * 4;
                let end = (c + 1) * 4 - 1;
                let block = &big_line[start..end];
                if block.contains("[") {
                    stacks[c].insert(
                        0,
                        block
                            .chars()
                            .nth(1)
                            .expect("Expected at least 2 characters of input in the 'crate'"),
                    );
                }
            }
        }
        // parse the moves
        if !line.trim().starts_with("move") {
            // if this line isn't a move, just skip it
            continue;
        }
        // move X from M to N
        let instructions: Vec<&str> = line.trim().split_ascii_whitespace().collect();
        let count = instructions[1]
            .parse::<usize>()
            .expect("Count should be numeric");
        let source = instructions[3]
            .parse::<usize>()
            .expect("Source column should be numeric")
            - 1;
        let destination = instructions[5]
            .parse::<usize>()
            .expect("Destination column should be numeric")
            - 1;

        for _n in 0..count {
            // pop the back of source stack and push to destination stack
            let block = stacks[source]
                .pop()
                .expect("There should have been a 'crate' on the stack here");
            stacks[destination].push(block);
        }
        //println!("{:?}", stacks);
    }
    let mut tops: String = String::new();
    for stack in stacks {
        if stack.len() > 0 {
            tops.push_str(
                stack
                    .get(stack.len() - 1..)
                    .expect("There should have been a 'crate' on the stack here"),
            );
        }
    }

    println!("Top of stacks: {:?}", tops);
}

pub fn day_five_p2(inputs: &str) {
    let lines: Vec<&str> = inputs.lines().collect();
    let columns = _get_columns(&lines);
    let mut stacks: Vec<String> = Vec::new();
    stacks.resize(columns, String::from(""));

    for &line in lines.iter() {
        if line.trim().len() == 0 {
            continue;
        }
        // parse the crates
        if line.contains("[") {
            let big_line = String::from(line);
            // This is a crate line
            for c in 0..columns {
                // look at this line in three character blocks with a 1 char spacer
                let start = c * 4;
                let end = (c + 1) * 4 - 1;
                let block = &big_line[start..end];
                if block.contains("[") {
                    stacks[c].insert(
                        0,
                        block
                            .chars()
                            .nth(1)
                            .expect("Expected at least 2 characters of input in the 'crate'"),
                    );
                }
            }
        }
        // parse the moves
        if !line.trim().starts_with("move") {
            // if this line isn't a move, just skip it
            continue;
        }
        // move X from M to N
        let instructions: Vec<&str> = line.trim().split_ascii_whitespace().collect();
        let count = instructions[1]
            .parse::<usize>()
            .expect("Count should be numeric");
        let source = instructions[3]
            .parse::<usize>()
            .expect("Source should be numeric")
            - 1;
        let destination = instructions[5]
            .parse::<usize>()
            .expect("Destination should be numeric")
            - 1;

        let destination_top = stacks[destination].len();
        for _n in 0..count {
            // pop the back of source stack and insert on destination stack
            let block = stacks[source]
                .pop()
                .expect("'crate' stack should not have been empty here");
            stacks[destination].insert(destination_top, block);
        }
        //println!("{:?}", stacks);
    }
    let mut tops: String = String::new();
    for stack in stacks {
        if stack.len() > 0 {
            tops.push_str(
                stack
                    .get(stack.len() - 1..)
                    .expect("There should have been at least one 'crate' here"),
            );
        }
    }

    println!("Top of stacks: {:?}", tops);
}

pub fn run_days() {
    let samples = include_str!("../inputs/03_sample.txt");
    let _inputs = include_str!("../inputs/03_input.txt");
    print!("\nRunning Day Three, Part one sample: ");
    day_three_p1(samples);
    print!("Running Day Three, Part one Inputs: ");
    day_three_p1(_inputs);

    print!("Running Day Three, Part two sample: ");
    day_three_p2(samples);
    print!("Running Day Three, Part two Inputs: ");
    day_three_p2(_inputs);

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
}
