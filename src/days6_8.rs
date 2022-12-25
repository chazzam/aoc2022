use std::collections::HashMap;
use std::collections::HashSet;
//use transpose::transpose;
extern crate transpose;

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

/*
#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
    //files: HashMap<&'a str, usize>,
    files_size: usize,
    dirs: HashMap<&'a str, Dir<'a>>,
}

impl<'a> Dir<'a> {
    pub fn new() -> Dir<'a> {
        Dir {
            name: "",
            //files: HashMap::new(),
            files_size: 0,
            dirs: HashMap::new(),
        }
    }

    fn add_file(&mut self, size: usize) {
        //self.files.insert(_name, size);
        self.files_size += size;
    }

    fn _add_dir(&mut self, _name: &'a str) {
        let mut new_dir = Dir::new();
        new_dir.name = _name;
        self.dirs.insert(_name, new_dir);
    }

    fn _size(&self) -> usize {
        //self.dirs.iter().map(|dir| dir.1.size()).sum::<usize>()
        //+ self.files.iter().map(|file| file.1).sum::<usize>()
        self.dirs.iter().map(|dir| dir.1._size()).sum::<usize>() + self.files_size
    }

    fn _update(&mut self, new_dir: Dir<'a>) -> bool {
        // Find a directory under this,
        // that matches new_dir.name, and replace old_dir with new_dir
        let name = new_dir.name;
        if self.dirs.contains_key(name) {
            self.dirs.insert(name, new_dir);
            true
        } else {
            false
        }
    }
}

fn _day_seven_p1_take2(inputs: &str) {
    let mut commands = inputs.split("$ ").map(|x| String::from(x));
    let mut tree: HashMap<String, usize> = HashMap::new();
    let mut pwd = String::from("");
    let mut root = Dir::new();
    root.name = "/";
    let mut dir = Dir { ..root };

    // Skip the initial empty split result
    commands.next();
    let mut command = commands.next();
    //let mut my_command: String;
    while !&command.is_none() {
        dbg!(&command);
        let my_command = command.expect("There should be a command here");
        if my_command.trim().len() == 0 {
            command = commands.next();
            continue;
        }
        dbg!(&my_command);
        match &my_command[..2] {
            "cd" => {
                // overwrite the existing dir in the list with the current root
                let dir_path = my_command.split_once(" ").unwrap();
                if dir_path.1.trim() == ".." {
                    // Need to go up a directory
                    if pwd.as_str() != "/" {
                        pwd.pop();
                        while pwd.pop() != Some('/') {}
                    }
                    //pwd = pwd.rsplit_once("/").unwrap().0.to_string();
                } else {
                    pwd.push_str(dir_path.1.trim());
                    if pwd.as_str() != "/" {
                        pwd.push_str("/");
                    }
                }
                if pwd.trim().len() == 0 {
                    pwd.push_str("/");
                }
                // add a dir, and get a copy mutable
                dbg!(&pwd);
            }
            "ls" => {
                // listing files; call .lines()
                // save size of current path in tree
                let mut dirsize: usize = 0;
                for line in my_command.lines() {
                    if line == "ls" {
                        continue;
                    }
                    dbg!(&line);
                    let parts = line
                        .split_once(" ")
                        .expect("There should be a file/dir entry");
                    match parts.0 {
                        "dir" | "ls" => {
                            continue;
                        }
                        _ => {
                            let size = parts
                                .0
                                .parse::<usize>()
                                .expect("Sections should be integers");
                            //dir.add_file(parts.1.clone(), size);
                            dir.add_file(size);
                            dirsize += size;
                        }
                    };
                }
                dbg!(&pwd, dirsize);
                tree.insert(String::from(&pwd), dirsize);
            }
            _ => {
                // add a file to this path?
                // continue? break?
            }
        }
        command = commands.next();
    }
    dbg!(tree);
}
*/

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

fn trees_count_row(row: &[u8], cols: usize) -> usize {
    row.windows(2)
        .enumerate()
        .inspect(|x| print!("{:?}:", x))
        .map_while(|(_i, x)| {
            if _i == cols - 2 {
                // Explicitly disallow comparing from the inside to the outside edge
                // As the outside is already counted regardless
                None
            } else if x[0] == x[1] {
                Some(0)
            } else if x[0] < x[1] {
                Some(1)
            } else {
                None
            }
        })
        .inspect(|x| print!("{:?}\n", x))
        .sum::<usize>()
        + 1
}

fn trees_count_rows(trees: Vec<u8>, rows: usize, cols: usize) -> usize {
    trees
        .chunks(cols)
        .enumerate()
        .map(|(i, x)| {
            if i == 0 || i == rows - 1 {
                2
            } else {
                let forward = trees_count_row(x, cols);
                forward
                    + trees_count_row(
                        (x.iter().rev().map(|x| *x).collect::<Vec<u8>>()).as_slice(),
                        cols,
                    )
            }
        })
        .sum()
}

fn day_eight_p1(_inputs: &str) {
    // Get the height list, and the dimensions of the grid
    let trees: Vec<_> = _inputs
        .lines()
        .map(|x| {
            dbg!(&x);
            x.split("").filter_map(|x| x.parse::<u8>().ok())
        })
        .flatten()
        .collect();
    let rows = _inputs.trim().lines().count();
    let cols = _inputs.lines().next().unwrap().trim().len();
    let mut trees_transposed = trees.clone();
    transpose::transpose(&trees, &mut trees_transposed, rows, cols);

    // process all rows
    // chunks each row,
    // then windows in pairs until trees are too tall to be seen
    let rows_parsed: usize = trees_count_rows(trees, rows, cols);
    dbg!(rows_parsed);
    println!("\nSwapping to transpose\n");
    // need to build a transpose view of the grid
    let cols_parsed: usize = trees_count_rows(trees_transposed, cols, rows);

    // this returns a number far too large because it counts some trees more than once.
    // need to go back to the drawing board on this one...
    println!("Trees Visible: {:?}", cols_parsed + rows_parsed);
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
    //day_seven_p1_take2(samples);
    print!("Running Day Seven Inputs: ");
    day_seven_p1(_inputs);

    let samples = include_str!("../inputs/08_sample.txt");
    let _inputs = include_str!("../inputs/08_input.txt");

    print!("\nRunning Day Eight, part one sample: ");
    day_eight_p1(samples);
    /*
    print!("Running Day Eight, part one Inputs: ");
    day_eight_p1(_inputs);
    print!("Running Day Eight, part two sample: ");
    day_eight_p2(samples);
    print!("Running Day Eight, part two Inputs: ");
    day_eight_p2(_inputs);
    */
}
