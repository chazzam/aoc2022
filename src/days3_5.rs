use std::collections::HashSet;

pub fn day_three_p1(inputs: &str) {
    let packs: Vec<&str> = inputs.lines().collect();
    let mut sum: u32 = 0;
    for pack in packs.iter() {
        if pack.trim().len() == 0 {
            continue;
        }
        let v_pack: Vec<char> = pack.chars().collect();
        if v_pack.len() % 2 != 0 {
            println!("Pack didn't have an even number of items: {}", pack);
        }
        let half = v_pack.len() / 2;
        let c1: HashSet<&char> = v_pack[..half].iter().collect();
        let c2: HashSet<&char> = v_pack[half..].iter().collect();

        let intersect: Vec<&&char> = c1.intersection(&c2).collect();
        let copied_char: char = if intersect.len() > 0 {
            **intersect[0]
        } else {
            '0'
        };
        let char_priority: u32 = match copied_char {
            'a'..='z' => copied_char as u32 - 97 + 1,
            'A'..='Z' => copied_char as u32 - 65 + 27,
            _ => 0,
        };
        sum += char_priority;
    }
    println!("Priority sum is {:?}", sum);
}

pub fn day_three_p2(inputs: &str) {
    let packs: Vec<&str> = inputs.lines().collect();
    let mut sum: u32 = 0;
    let mut index = 0;
    while index < packs.len() {
        if index + 3 >= packs.len() {
            break;
        }
        let elf1: HashSet<char> = packs[index + 0].chars().collect();
        let elf2: HashSet<char> = packs[index + 1].chars().collect();
        let elf3: HashSet<char> = packs[index + 2].chars().collect();

        let e1_2: HashSet<&char> = elf1.intersection(&elf2).collect();
        let e2_3: HashSet<&char> = elf2.intersection(&elf3).collect();

        let badge_set: Vec<&&char> = e1_2.intersection(&e2_3).collect();
        let badge: char = if badge_set.len() > 0 {
            **badge_set[0]
        } else {
            '0'
        };
        let badge_priority: u32 = match badge {
            'a'..='z' => badge as u32 - 97 + 1,
            'A'..='Z' => badge as u32 - 65 + 27,
            _ => 0,
        };
        sum += badge_priority;
        index += 3;
    }
    println!("Priority sum is {:?}", sum);
}

pub fn day_four_p1(inputs: &str) {
    let id_pairs: Vec<&str> = inputs.lines().collect();
    let mut wholly_contained = 0;
    let mut any_overlap = 0;
    for id_pair in id_pairs.iter() {
        if id_pair.trim().len() == 0 {
            continue;
        }
        //let ids: Vec<&str> = id_pair.split(",").collect();
        let sections: Vec<&str> = id_pair.split(&['-', ','][..]).collect();
        //let mut s_ids: Vec<RangeInclusive<i32>> = Vec::new();
        let mut s_ids: Vec<i32> = Vec::new();
        for id in sections.iter() {
            s_ids.push(id.parse::<i32>().expect("Sections should be integers"));
        }
        let range1 = s_ids[0]..=s_ids[1];
        let range2 = s_ids[2]..=s_ids[3];

        if (range2.contains(range1.start()) && range2.contains(range1.end()))
            || (range1.contains(&range2.start()) && range1.contains(range2.end()))
        {
            wholly_contained += 1;
        }
        if range2.contains(range1.start())
            || range2.contains(range1.end())
            || range1.contains(&range2.start())
            || range1.contains(range2.end())
        {
            any_overlap += 1;
        }
        //sections.extend_from_slice(ids[1].split("-").collect()[..]);
    }

    println!(
        "Sections wholly contained: {:?}; Sections with Overlap: {:?}",
        wholly_contained, any_overlap
    );
}

pub fn day_five_p1(inputs: &str) {
    let lines: Vec<&str> = inputs.lines().collect();
    let mut columns = 0;
    // Get the row with the column numbers
    for line in lines.iter() {
        if !line.trim().starts_with("1") {
            continue;
        }
        let cols: Vec<&str> = line.trim().split("   ").collect();
        columns = cols.len();
    }
    let mut stacks: Vec<String> = Vec::new();
    for _c in 0..columns {
        stacks.push(String::from(""));
    }
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
    let mut columns = 0;
    // Get the row with the column numbers
    for line in lines.iter() {
        if !line.trim().starts_with("1") {
            continue;
        }
        let cols: Vec<&str> = line.trim().split("   ").collect();
        columns = cols.len();
    }
    let mut stacks: Vec<String> = Vec::new();
    for _c in 0..columns {
        stacks.push(String::from(""));
    }
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
