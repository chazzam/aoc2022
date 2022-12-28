use std::collections::HashMap;

#[derive(Debug)]
struct Monkey {
    inspections: usize,
    items: Vec<u128>,
    operation: fn(u128, u128) -> u128,
    operation2: fn(u128, u128) -> u128,
    operation_arg: usize,
    use_old: bool,
    test: usize,
    res_true: usize,
    res_false: usize,
}

impl Monkey {
    fn evaluate_item(&self, item: u128, use2: bool, factor: u128) -> (usize, u128) {
        let arg = match self.use_old {
            true => item,
            false => self.operation_arg as u128,
        };
        let mut value: u128 = match use2 {
            true => (self.operation2)(item, arg),
            false => (self.operation)(item, arg),
        };
        if use2 {
            value = value % factor;
        }
        match value % self.test as u128 == 0 {
            true => (self.res_true, value),
            false => (self.res_false, value),
        }
    }
}

fn evaluate_monkeys(old_monkeys: Vec<Monkey>, use2: bool) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    // copy the basic parts
    for monkey in old_monkeys.iter() {
        monkeys.push(Monkey {
            inspections: monkey.inspections + monkey.items.len(),
            items: Vec::new(),
            ..*monkey
        })
    }

    let factor: u128 = monkeys.iter().map(|x| x.test as u128).product();

    // update the item worries
    let mut off_index = 0;
    for monkey in old_monkeys.into_iter() {
        // work old items into new list, and chain any waiting new items in
        for item in monkey
            .items
            .iter()
            .chain(monkeys[off_index].items.clone().iter())
        {
            let (index, new_worry) = monkey.evaluate_item(*item, use2, factor);
            monkeys[index].items.push(new_worry);
        }
        monkeys[off_index].inspections += monkeys[off_index].items.len();
        monkeys[off_index].items.clear();
        off_index += 1;
    }

    monkeys
}

pub fn day_eleven_p1(inputs: &str, use2: bool) {
    let monkeys_raw: Vec<&str> = inputs.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = Vec::new();
    for monkey_raw in monkeys_raw.into_iter() {
        let mut raw_lines = monkey_raw.lines();
        raw_lines.next();
        let starting_items: Vec<_> = raw_lines
            .next()
            .expect("There should be a line here")
            .split_once(":")
            .expect("This should be a heading")
            .1
            .split(",")
            .map(|x| x.trim().parse::<u128>().unwrap())
            .collect();
        let fields: Vec<_> = raw_lines
            .next()
            .unwrap()
            .split_once("old")
            .unwrap()
            .1
            .trim()
            .split_whitespace()
            .collect();
        let mut use_old: bool = false;
        let operation_arg: usize = match fields[1] {
            "old" => {
                use_old = true;
                0
            }
            _ => fields[1]
                .parse::<usize>()
                .expect("This should be an integer value"),
        };

        let test: usize = raw_lines
            .next()
            .unwrap()
            .split_once(" by ")
            .unwrap()
            .1
            .parse::<usize>()
            .expect("This should be an integer divisible by");
        let res_true: usize = raw_lines
            .next()
            .unwrap()
            .split_once(" monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .expect("This should be an integer divisible by");
        let res_false: usize = raw_lines
            .next()
            .unwrap()
            .split_once(" monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .expect("This should be an integer divisible by");
        let monkey = Monkey {
            inspections: 0,
            items: starting_items,
            operation: match fields[0] {
                "+" => |a, arg| ((a + arg) as f64 / 3 as f64).floor() as u128,
                "-" => |a, arg| ((a - arg) as f64 / 3 as f64).floor() as u128,
                "*" => |a, arg| ((a * arg) as f64 / 3 as f64).floor() as u128,
                "/" | _ => |a, arg| ((a / arg) as f64 / 3 as f64).floor() as u128,
            },
            operation2: match fields[0] {
                "+" => |a: u128, arg| a + arg as u128,
                "-" => |a: u128, arg| a - arg,
                "*" => |a: u128, arg| a * arg,
                "/" | _ => |a: u128, arg| a / arg,
            },
            operation_arg,
            use_old,
            test,
            res_true,
            res_false,
        };
        monkeys.push(monkey);
    }

    let business_level = match use2 {
        true => 0..10000,
        false => 0..20,
    };
    for _x in business_level {
        monkeys = evaluate_monkeys(monkeys, use2);
    }
    monkeys.sort_by_key(|x| x.inspections);
    println!(
        "Level of monkey business: {:?}",
        monkeys
            .iter()
            .rev()
            .take(2)
            .fold(1, |acc, x| x.inspections * acc)
    );
}

#[derive(Debug)]
struct Cell {
    head: bool,
    tail: bool,
    head_visited: bool,
    tail_visited: bool,
}

#[derive(Debug)]
struct MultiCell {
    head: bool,
    tails: Vec<bool>,
    head_visited: bool,
    tails_visited: Vec<bool>,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn get_updated_tail_coordinates(
    head_row: i16,
    head_col: i16,
    current_tail_row: i16,
    current_tail_col: i16,
) -> (i16, i16) {
    let mut tail_row = current_tail_row;
    let mut tail_col = current_tail_col;

    // Update the tail
    if head_col == tail_col {
        if head_row > tail_row + 1 {
            tail_row += 1;
        } else if head_row + 1 < tail_row {
            tail_row -= 1;
        }
    } else if head_row == tail_row {
        if head_col > tail_col + 1 {
            tail_col += 1;
        } else if head_col + 1 < tail_col {
            tail_col -= 1;
        }
    } else if head_row > tail_row + 1 {
        if head_col > tail_col {
            tail_row += 1;
            tail_col += 1;
        } else if head_col < tail_col {
            tail_row += 1;
            tail_col -= 1;
        }
    } else if head_row + 1 < tail_row {
        if head_col > tail_col {
            tail_row -= 1;
            tail_col += 1;
        } else if head_col < tail_col {
            tail_row -= 1;
            tail_col -= 1;
        }
    } else if head_col > tail_col + 1 {
        if head_row > tail_row {
            tail_row += 1;
            tail_col += 1;
        } else if head_row < tail_row {
            tail_row -= 1;
            tail_col += 1;
        }
    } else if head_col + 1 < tail_col {
        if head_row > tail_row {
            tail_row += 1;
            tail_col -= 1;
        } else if head_row < tail_row {
            tail_row -= 1;
            tail_col -= 1;
        }
    }

    (tail_row, tail_col)
}

fn move_head(
    mut grid: HashMap<(i16, i16), Cell>,
    mut current_row: i16,
    mut current_col: i16,
    dir: Direction,
    repeat: usize,
) -> (i16, i16, HashMap<(i16, i16), Cell>) {
    let mut row = current_row;
    let mut col = current_col;

    // find tail
    let mut current_tail_row: i16;
    let mut current_tail_col: i16;
    (current_tail_row, current_tail_col) = grid
        .iter()
        .find_map(
            |((row, col), cell)| {
                if cell.tail {
                    Some((*row, *col))
                } else {
                    None
                }
            },
        )
        .expect("Should be a tail");

    for _r in 0..repeat {
        match dir {
            Direction::Up => row += 1,
            Direction::Down => row -= 1,
            Direction::Left => col -= 1,
            Direction::Right => col += 1,
        }

        // Update the head
        grid.entry((current_row, current_col))
            .and_modify(|cell| cell.head = false);

        grid.entry((row, col))
            .and_modify(|cell| {
                cell.head = true;
                cell.head_visited = true;
            })
            .or_insert(Cell {
                head: true,
                tail: false,
                head_visited: true,
                tail_visited: false,
            });

        // Update the tail
        let (tail_row, tail_col) =
            get_updated_tail_coordinates(row, col, current_tail_row, current_tail_col);

        grid.entry((current_tail_row, current_tail_col))
            .and_modify(|cell| cell.tail = false);

        grid.entry((tail_row, tail_col))
            .and_modify(|cell| {
                cell.tail = true;
                cell.tail_visited = true;
            })
            .or_insert(Cell {
                head: false,
                tail: true,
                head_visited: false,
                tail_visited: true,
            });

        // Update the current row and column
        current_row = row;
        current_col = col;
        current_tail_row = tail_row;
        current_tail_col = tail_col;
    }
    (row, col, grid)
}

fn debug_display(grid: &HashMap<(i16, i16), Cell>) -> Vec<String> {
    let rows: i16 = grid
        .keys()
        .max_by_key(|(row, _col)| row)
        .map(|(row, _col)| *row)
        .unwrap();
    let cols: i16 = grid
        .keys()
        .max_by_key(|(_row, col)| col)
        .map(|(_row, col)| *col)
        .unwrap();
    let min_row: i16 = grid
        .keys()
        .min_by_key(|(row, _col)| row)
        .map(|(row, _col)| *row)
        .unwrap();
    let min_col: i16 = grid
        .keys()
        .min_by_key(|(_row, col)| col)
        .map(|(_row, col)| *col)
        .unwrap();
    let total_rows: i16 = (rows - min_row) + 1;
    let mut _display: Vec<_> = vec![String::from(""); total_rows as usize];
    //let mut display_row = total_rows - 1;
    let mut display_row: usize = 0;
    for r in (min_row..=rows as i16).rev() {
        for c in min_col..=cols as i16 {
            let cell = grid.get(&(r, c)).unwrap_or(&Cell {
                head: false,
                tail: false,
                head_visited: false,
                tail_visited: false,
            });
            let mut field = ".";
            if r == 1 && c == 1 {
                field = "$";
            } else if cell.tail_visited {
                field = "#";
            } else if cell.head_visited {
                field = "x";
            }
            _display[display_row].push_str(field);
        }
        display_row += 1;
    }
    _display
}

pub fn day_nine_p1(inputs: &str) {
    let mut grid: HashMap<(i16, i16), Cell> = HashMap::new();
    let mut row = 1;
    let mut col = 1;
    grid.insert(
        (row, col),
        Cell {
            head: true,
            tail: true,
            head_visited: true,
            tail_visited: true,
        },
    );

    for line in inputs.lines() {
        let fields: Vec<_> = line.split_whitespace().collect();
        let dir: Direction = match fields[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" | _ => Direction::Right,
        };
        let repeat: usize = fields[1]
            .parse::<usize>()
            .expect("Repeat should be an integer");
        (row, col, grid) = move_head(grid, row, col, dir, repeat);
    }

    let head_visited: usize = grid.values().filter(|x| x.head_visited).count();
    let visited: usize = grid.values().filter(|x| x.tail_visited).count();

    // display for debugging
    let _display = debug_display(&grid);
    //dbg!(&_display);

    println!("Tail Visited: {:?} ({:?})", visited, head_visited);
    //dbg!(grid);
}

fn update_rope(
    //mut moves: impl Iterator<Item = Option<(i16, i16)>>,
    mut moves: impl Iterator<Item = (i16, i16)>,
    tails: usize,
) -> HashMap<(i16, i16), Cell> {
    let mut grid: HashMap<(i16, i16), Cell> = HashMap::new();
    //let (row, col) = moves.next().unwrap().unwrap();
    let (mut last_row, mut last_col) = moves.next().unwrap();
    grid.insert(
        (last_row, last_col),
        Cell {
            head: true,
            tail: true,
            head_visited: true,
            tail_visited: true,
        },
    );
    //moves.scan((last_row, last_col), |(last_row, last_col), (row,col)|)
    let mut pos_tails: Vec<(i16, i16)> = vec![(last_row, last_col); tails];
    for (head_row, head_col) in moves {
        // Update the head
        grid.entry((last_row, last_col))
            .and_modify(|cell| cell.head = false);
        last_row = head_row;
        last_col = head_col;

        grid.entry((head_row, head_col))
            .and_modify(|cell| {
                cell.head = true;
                cell.head_visited = true;
            })
            .or_insert(Cell {
                head: true,
                tail: false,
                head_visited: true,
                tail_visited: false,
            });

        let (current_tail_row, current_tail_col) = pos_tails[pos_tails.len() - 1];
        //let new_pos_tails: Vec<(i16, i16)> = pos_tails
        pos_tails = pos_tails
            .iter()
            .scan((head_row, head_col), |(head_row, head_col), (row, col)| {
                let (tail_row, tail_col) =
                    get_updated_tail_coordinates(*head_row, *head_col, *row, *col);
                /*println!(
                    "({:?},{:?}) & ({:?},{:?}) -> ({:?},{:?})",
                    head_row, head_col, row, col, tail_row, tail_col
                );*/
                *head_row = tail_row.clone();
                *head_col = tail_col.clone();
                Some((tail_row, tail_col))
            })
            // .inspect(|(r, c)| print!("{:?},{:?}; ", r, c))
            .collect();
        //pos_tails = new_pos_tails;
        //print!("\n");

        grid.entry((current_tail_row, current_tail_col))
            .and_modify(|cell| cell.tail = false);

        let (tail_row, tail_col) = pos_tails.last().unwrap();
        grid.entry((*tail_row, *tail_col))
            .and_modify(|cell| {
                cell.tail = true;
                cell.tail_visited = true;
            })
            .or_insert(Cell {
                head: false,
                tail: true,
                head_visited: false,
                tail_visited: true,
            });
    }
    grid
}

pub fn day_nine_p2(inputs: &str) {
    // Build an iterator over all the Head moves
    let moves = inputs
        .lines()
        .scan((1, 1, false), |(row, col, init), line| {
            let mut fields = line.split_whitespace().rev();
            let repeat: usize = fields
                .next()
                .unwrap()
                .parse::<usize>()
                .expect("Repeat should be an integer");
            let dir = match fields.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" | _ => Direction::Right,
            };
            let mut dirs: Vec<_> = Vec::new();
            if !*init {
                //dirs.push(Some((row.clone(), col.clone())));
                dirs.push((row.clone(), col.clone()));
                *init = true;
            }

            for _r in 0..repeat {
                match &dir {
                    Direction::Up => *row += 1,
                    Direction::Down => *row -= 1,
                    Direction::Left => *col -= 1,
                    Direction::Right => *col += 1,
                };
                //dirs.push(Some((row.clone(), col.clone())));
                dirs.push((row.clone(), col.clone()));
            }
            Some(dirs)
        })
        .flatten();
    //dbg!(&moves.collect::<Vec<_>>());
    let grid = update_rope(moves, 9);
    let head_visited: usize = grid.values().filter(|x| x.head_visited).count();
    let visited: usize = grid.values().filter(|x| x.tail_visited).count();

    // display for debugging
    let _display = debug_display(&grid);
    //dbg!(&_display);

    println!("Tail Visited: {:?} ({:?})", visited, head_visited);
    //dbg!(grid);
}

pub fn run_days() {
    let samples = include_str!("../inputs/09_sample.txt");
    let _inputs = include_str!("../inputs/09_input.txt");
    print!("\nRunning Day Nine, Part one sample: ");
    day_nine_p1(samples);
    print!("Running Day Nine, Part one Inputs: ");
    day_nine_p1(_inputs);

    let _samples2 = include_str!("../inputs/09_sample2.txt");
    print!("Running Day Nine, Part two sample: ");
    day_nine_p2(samples);
    day_nine_p2(_samples2);
    print!("Running Day Nine, Part two Inputs: ");
    day_nine_p2(_inputs);

    let samples = include_str!("../inputs/11_sample.txt");
    let _inputs = include_str!("../inputs/11_input.txt");
    print!("\nRunning Day Eleven, Part one sample: ");
    day_eleven_p1(samples, false);
    print!("Running Day Eleven, Part one Inputs: ");
    day_eleven_p1(_inputs, false);

    print!("Running Day Eleven, Part two sample: ");
    day_eleven_p1(samples, true);
    print!("Running Day Eleven, Part two Inputs: ");
    day_eleven_p1(_inputs, true);
}
