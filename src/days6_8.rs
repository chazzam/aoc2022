use std::collections::HashMap;
use std::collections::HashSet;

pub fn day_six_p1(inputs: &str) {
    let lines: Vec<&str> = inputs.lines().collect();
    /*let packet = inputs.lines().map(|x| {
        x.chars().collect::<Vec<_>>().windows(4).map(|x| {
            let marker: HashSet<_> = x.iter().collect();
          if marker.len() == 4 {}
        })
      if
    });*/
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

fn day_seven_p1(inputs: &str) {
    let mut commands = inputs.split("$ ").map(|x| String::from(x));
    let mut tree: HashMap<String, u64> = HashMap::new();
    let mut pwd = String::from("");
    // dir has size
    // stack of dirs
    // hash of path/to/dir, value is size of files there

    // Skip the initial cd /
    commands.next();
    let mut command = commands.next();
    while !command.is_none() {
        //dbg!(&command);
        let my_command = command.expect("There should be a command here");
        if my_command.trim().len() == 0 {
            command = commands.next();
            continue;
        }
        //dbg!(&my_command);
        match &my_command[..2] {
            "cd" => {
                // update the current filepath
                let dir_path = my_command.split_once(" ").unwrap();
                if dir_path.1.trim() == ".." {
                    // Need to go up a directory
                    if pwd.len() > 1 {
                        pwd.pop();
                        while pwd.pop() != Some('/') {}
                    }
                    pwd.push_str("/");
                    //pwd = pwd.rsplit_once("/").unwrap().0.to_string();
                } else {
                    pwd.push_str(dir_path.1.trim());
                    if pwd.len() > 1 {
                        pwd.push_str("/");
                    }
                }
                if pwd.trim().len() == 0 {
                    pwd.push_str("/");
                }
                //dbg!(dir_path.1, &pwd);
            }
            "ls" => {
                // listing files; call .lines()
                // save size of current path in tree
                let mut dirsize: u64 = 0;
                for line in my_command.lines() {
                    if line == "ls" {
                        continue;
                    }
                    //dbg!(&line);
                    let parts = line
                        .split_once(" ")
                        .expect("There should be a file/dir entry");
                    match parts.0 {
                        "dir" | "ls" => {
                            continue;
                        }
                        _ => {
                            dirsize += parts.0.parse::<u64>().expect("Sections should be integers")
                        }
                    };
                }
                //dbg!(&pwd, dirsize);
                tree.insert(String::from(&pwd), dirsize);
            }
            _ => {
                // add a file to this path?
                // continue? break?
            }
        }
        command = commands.next();
    }
    let mut size_tree: HashMap<String, u64> = HashMap::new();
    let mut sum: u64 = 0;
    for (root, root_size) in tree.iter() {
        let size: u64 = tree.iter().fold(*root_size, |acc, (name, size)| {
            if name.contains(root) && name != root {
                acc + size
            } else {
                acc
            }
        });
        size_tree.insert(root.clone(), size);
        if size <= 100000 {
            sum += size;
        }
    }
    print!("sum size <= 100,000: {:?}; ", sum);

    let total = 70000000; // 70,000,000
    let min_free = 30000000; // 30,000,000
    let _used: u64 = tree.iter().fold(0, |acc, (_name, size)| acc + *size as u64);
    let current_free: i64 =
        total - *size_tree.get(&String::from("/")).expect("should be value") as i64;
    let minimum_deletion = min_free - current_free;

    /*dbg!(
        &size_tree,
        current_free,
        minimum_deletion,
        _used,
        &tree.len()
    );*/
    let min_to_delete = size_tree
        .iter()
        .filter(|(_name, size)| **size >= minimum_deletion as u64)
        .min_by_key(|x| x.1)
        .expect("there should be a minimum size")
        .1;
    //dbg!(min_to_delete);
    println!("Delete {:?} space to update", min_to_delete);
    //dbg!(tree, size_tree);
}

fn check_visibility(trees: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let tree = trees[row][col];
    let cols = trees[0].len();

    // check from up
    let mut vis_up = false;
    for r in 0..row {
        if trees[r][col] >= tree {
            // if any tree in the path is larger, try another direction
            break;
        } else if r + 1 == row {
            vis_up = true;
        }
    }

    // check from down
    let mut vis_down = false;
    for r in row + 1..trees.len() {
        if trees[r][col] >= tree {
            // if any tree in the path is larger, try another direction
            break;
        } else if r == trees.len() - 1 {
            vis_down = true;
        }
    }

    // check from left
    // can just compare the slice from start to here in this vec
    let mut vis_left = false;
    if trees[row][0..col].iter().max().unwrap() < &tree {
        vis_left = true;
    }

    // check from right
    // just compare the slice from the position to the right to the end
    let mut vis_right = false;
    if trees[row][col + 1..cols].iter().max().unwrap() < &tree {
        vis_right = true;
    }
    let visible = vis_right || vis_left || vis_up || vis_down;
    /*print!(
        "\n({:?},{:?}) = {:?}, left: ({:?})={:?}, right: ({:?})={:?}, vis: {:?},{:?},{:?},{:?}->{:?}",
        row,
        col,
        tree,
        trees[row][0..col].len(),
        trees[row][0..col].iter().max().unwrap(),
        trees[row][col + 1..cols].len(),
        trees[row][col + 1..cols].iter().max().unwrap(),
        vis_up,
        vis_down,
        vis_left,
        vis_right,
        visible
    );*/
    match visible {
        true => 1,
        false => 0,
    }
}

fn day_eight_p1(_inputs: &str) {
    // Get the height list, and the dimensions of the grid
    let trees: Vec<Vec<_>> = _inputs
        .lines()
        .map(|x| {
            //dbg!(&x);
            x.split("").filter_map(|x| x.parse::<u8>().ok()).collect()
        })
        .collect();
    let rows = trees.len();
    let cols = trees[0].len();
    let mut visible: usize = 0;

    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let is_visible = check_visibility(&trees, row, col);
            //println!(" ({:?},{:?})={:?}", row, col, is_visible);
            visible += is_visible;
        }
    }
    //dbg!(visible);

    // Count the outside trees, and subtract off the corners
    // so we don't count them twice
    println!("Trees Visible: {:?}", cols * 2 + rows * 2 - 4 + visible);
}

fn count_visibility(trees: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let tree = trees[row][col];
    let cols = trees[0].len();

    // check from up
    let mut vis_up = 0;
    for r in (0..row).rev() {
        vis_up += 1;
        if trees[r][col] >= tree {
            // if any tree in the path is larger, try another direction
            break;
        }
    }

    // check from down
    let mut vis_down = 0;
    for r in row + 1..trees.len() {
        vis_down += 1;
        if trees[r][col] >= tree {
            // if any tree in the path is larger, try another direction
            break;
        }
    }

    // check from left
    // can just compare the slice from start to here in this vec
    let mut vis_left = 0;
    for view in trees[row][0..col].iter().rev() {
        vis_left += 1;
        if view >= &tree {
            break;
        }
    }

    // check from right
    // just compare the slice from the position to the right to the end
    let mut vis_right = 0;
    for view in trees[row][col + 1..cols].iter() {
        vis_right += 1;
        if view >= &tree {
            break;
        }
    }
    let visible = vis_right * vis_left * vis_up * vis_down;
    /*print!(
        "\n({:?},{:?}) = {:?}, left: ({:?})={:?}, right: ({:?})={:?}, vis: {:?},{:?},{:?},{:?}->{:?}",
        row,
        col,
        tree,
        trees[row][0..col].len(),
        trees[row][0..col].iter().max().unwrap(),
        trees[row][col + 1..cols].len(),
        trees[row][col + 1..cols].iter().max().unwrap(),
        vis_up,
        vis_down,
        vis_left,
        vis_right,
        visible
    );*/
    visible
}

fn day_eight_p2(_inputs: &str) {
    // Get the height list, and the dimensions of the grid
    let trees: Vec<Vec<_>> = _inputs
        .lines()
        .map(|x| {
            //dbg!(&x);
            x.split("").filter_map(|x| x.parse::<u8>().ok()).collect()
        })
        .collect();
    let rows = trees.len();
    let cols = trees[0].len();

    let mut visibility_scores: Vec<usize> = Vec::new();
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let visible = count_visibility(&trees, row, col);
            //println!(" ({:?},{:?})={:?}", row, col, visible);
            visibility_scores.push(visible);
        }
    }

    println!(
        "Highest Tree Visiblility Score: {:?}",
        visibility_scores.iter().max().unwrap()
    );
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

    let samples = include_str!("../inputs/07_sample.txt");
    let _inputs = include_str!("../inputs/07_input.txt");

    print!("\nRunning Day Seven sample: ");
    day_seven_p1(samples);
    print!("Running Day Seven Inputs: ");
    day_seven_p1(_inputs);

    let samples = include_str!("../inputs/08_sample.txt");
    let _inputs = include_str!("../inputs/08_input.txt");

    print!("\nRunning Day Eight, part one sample: ");
    day_eight_p1(samples);
    print!("Running Day Eight, part one Inputs: ");
    day_eight_p1(_inputs);
    print!("Running Day Eight, part two sample: ");
    day_eight_p2(samples);
    print!("Running Day Eight, part two Inputs: ");
    day_eight_p2(_inputs);
}
