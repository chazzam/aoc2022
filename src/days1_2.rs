pub fn day_one_both_parts(inputs: &str) {
    let calorie_groups: Vec<&str> = inputs.split("\n\n").collect();

    let mut sums: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;
    for group in calorie_groups.iter() {
        let cals: Vec<&str> = group.split("\n").collect();
        for cal in cals.iter() {
            if cal.trim().len() == 0 {
                continue;
            }
            sum += cal.parse::<i32>().unwrap();
        }
        sums.push(sum);
        sum = 0;
    }
    //println!("calories: {:?}", sums);
    print!("Elf with most calories: {:?}; ", sums.iter().max().unwrap());
    if sums.len() < 3 {
        return;
    }
    sums.sort();
    //println!("{:?}", sums);
    sums.reverse();
    for elf in &sums[..3] {
        sum += elf;
    }
    println!("Top three elves: {:?}", sum);
}

pub fn _base_three_inc(input: i32) -> i32 {
    let mut out = input + 1;
    if out > 3 {
        out = 1;
    }
    return out;
}

pub fn day_two_part_one(inputs: &str) {
    // A/X = Rock; B/Y = Paper; C/Z = Scissors
    // X = 1; Y = 2; Z = 3
    // A > 3; B > 1; C > 2
    // Lose = 0; Tie = 3; Win = 6
    let plays: Vec<&str> = inputs.split("\n").collect();
    let mut score = 0;
    for play in plays.iter() {
        if play.trim().len() == 0 {
            continue;
        }
        let choices: Vec<&str> = play.split(" ").collect();
        let opp: i32 = match choices[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => 0,
        };
        let choice: i32 = match choices[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };
        let win = match _base_three_inc(opp) == choice {
            true => 6,
            false => 0,
        };
        let tied = match opp == choice {
            true => 3,
            false => 0,
        };
        score += choice + win + tied;
    }
    println!("Your score for this game is: {:?}", score);
}

pub fn day_two_part_two(inputs: &str) {
    // A/X = Rock; B/Y = Paper; C/Z = Scissors
    // X = 1; Y = 2; Z = 3
    // A > 3; B > 1; C > 2
    // Lose = 0; Tie = 3; Win = 6
    let plays: Vec<&str> = inputs.split("\n").collect();
    let mut score = 0;
    for play in plays.iter() {
        if play.trim().len() == 0 {
            continue;
        }
        let choices: Vec<&str> = play.split(" ").collect();
        let opp: i32 = match choices[0] {
            "A" => 1,
            "B" => 2,
            "C" => 3,
            _ => 0,
        };
        let result: i32 = match choices[1] {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        };
        let choice: i32 = match result {
            1 => _base_three_inc(_base_three_inc(opp)),
            3 => _base_three_inc(opp),
            2 | _ => opp,
        };
        let win = match _base_three_inc(opp) == choice {
            true => 6,
            false => 0,
        };
        let tied = match opp == choice {
            true => 3,
            false => 0,
        };
        score += choice + win + tied;
    }
    println!("Your score for this game is: {:?}", score);
}

pub fn run_days() {
    let samples = include_str!("../inputs/01_sample.txt");
    print!("\nRunning Day One sample: ");
    day_one_both_parts(samples);

    print!("Running Day One Inputs: ");
    let inputs = include_str!("../inputs/01_input.txt");
    day_one_both_parts(inputs);

    print!("\nRunning Day Two, part one sample: ");
    let samples = include_str!("../inputs/02_sample.txt");
    day_two_part_one(samples);

    print!("Running Day Two, part one Inputs: ");
    let inputs = include_str!("../inputs/02_input.txt");
    day_two_part_one(inputs);

    print!("Running Day Two, part two sample: ");
    day_two_part_two(samples);
    print!("Running Day Two, part two Inputs: ");
    day_two_part_two(inputs);
}
