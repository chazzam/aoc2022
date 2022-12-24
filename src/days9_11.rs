#[derive(Debug)]
//struct Monkey<'a> {
struct Monkey {
    inspections: usize,
    items: Vec<u128>,
    operation: fn(u128, u128) -> u128,
    operation2: fn(u128, u128) -> u128,
    operation_arg: usize,
    use_old: bool,
    //test: &'a dyn Fn(usize) -> (usize, usize),
    //test: fn(usize) -> (usize, usize),
    test: usize,
    res_true: usize,
    res_false: usize,
}

//impl<'a> Monkey<'a> {
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

pub fn run_days() {
    let samples = include_str!("../inputs/11_sample.txt");
    let _inputs = include_str!("../inputs/11_input.txt");
    print!("\nRunning Day Eleven, Part one sample: ");
    day_eleven_p1(samples, false);
    print!("Running Day Eleven, Part one Inputs: ");
    day_eleven_p1(_inputs, false);

    print!("\nRunning Day Eleven, Part two sample: ");
    day_eleven_p1(samples, true);
    print!("Running Day Eleven, Part two Inputs: ");
    day_eleven_p1(_inputs, true);

    /*
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
    print!("Running Day Eight, part one Inputs: ");
    day_eight_p1(_inputs);
    print!("Running Day Eight, part two sample: ");
    day_eight_p2(samples);
    print!("Running Day Eight, part two Inputs: ");
    day_eight_p2(_inputs);
    */
}
