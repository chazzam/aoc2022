pub fn day_one_both_parts(inputs: &str) {
    let mut sums = inputs
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|s| s.parse::<i32>().expect("This should be an integer value"))
                .fold(0, |acc, x| acc + x)
        })
        .collect::<Vec<_>>();

    print!(
        "Elf with most calories: {:?}; ",
        sums.iter()
            .max()
            .expect("Should have a max value of calories")
    );

    // Part 2 begins
    sums.sort();
    let sum: i32 = sums.iter().rev().take(3).sum();
    println!("Top three elves: {:?}", sum);
}

fn _decode_value(play: &str) -> i32 {
    match play {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => 0,
    }
}

fn _get_win_value(play: &str) -> &str {
    match play {
        "A" | "Y" => "B",
        "B" | "Z" => "C",
        "C" | "X" | _ => "A",
    }
}

fn _score_choice(s_opp: &str, s_choice: &str) -> i32 {
    let win = match _get_win_value(s_opp) == s_choice {
        true => 6,
        false => 0,
    };

    let tied = match s_opp == s_choice {
        true => 3,
        false => 0,
    };

    _decode_value(s_choice) + win + tied
}

fn _select_choice<'a>(guide: &str, s_opp: &'a str) -> &'a str {
    // X = lose; Y = tie; Z = win;
    match guide {
        "X" => _get_win_value(_get_win_value(s_opp)),
        "Z" => _get_win_value(s_opp),
        "Y" | _ => s_opp,
    }
}

pub fn day_two_part_one(inputs: &str) {
    // A/X = Rock; B/Y = Paper; C/Z = Scissors
    // X = 1; Y = 2; Z = 3
    // A > 3; B > 1; C > 2
    // Lose = 0; Tie = 3; Win = 6
    let score: i32 = inputs
        .lines()
        .map(|play| {
            play.split_once(" ")
                .expect("Should have at least one space")
        })
        .map(|(x, you)| _score_choice(x, _get_win_value(you)))
        .sum();
    println!("Your score for this game is: {:?}", score);
}

pub fn day_two_part_two(inputs: &str) {
    // A/X = Rock; B/Y = Paper; C/Z = Scissors
    // X = lose; Y = tie; Z = win
    // Lose = 0; Tie = 3; Win = 6
    let score: i32 = inputs
        .lines()
        .map(|play| {
            play.split_once(" ")
                .expect("Should have at least one space")
        })
        .map(|(x, you)| _score_choice(x, _select_choice(you, x)))
        .sum();
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
