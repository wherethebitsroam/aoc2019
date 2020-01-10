use crate::intcode::ExitMode;
use crate::intcode::Intcode;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}

pub fn part1() {
    let f = File::open("day21.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut code = Intcode::new(&data);

    let inst = "NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
";

    let inputs: Vec<i64> = inst.chars().map(|x| x as i64).collect();

    // the input function
    let mut input_index = 0;
    let mut input_fn = || -> i64 {
        let i = input_index;
        input_index += 1;
        //println!("{}", inputs[i]);
        inputs[i]
    };

    for _ in 0..10000 {
        match code.run(&mut input_fn) {
            ExitMode::Halt => {
                //println!("halt");
                break;
            }
            ExitMode::Output(x) => {
                println!("{}", x);
            }
        };
    }
}

pub fn part2() {
    // options();
    part2x();
}

pub fn part2x() {
    let f = File::open("day21.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut code = Intcode::new(&data);

    let inst = "OR E J
OR H J
OR A T
AND B T
AND C T
NOT T T
AND T J
AND D J
RUN
";

    let inputs: Vec<i64> = inst.chars().map(|x| x as i64).collect();

    // the input function
    let mut input_index = 0;
    let mut input_fn = || -> i64 {
        let i = input_index;
        input_index += 1;
        //println!("{}", inputs[i]);
        inputs[i]
    };

    for _ in 0..10000 {
        match code.run(&mut input_fn) {
            ExitMode::Halt => {
                // println!("halt");
                break;
            }
            ExitMode::Output(x) => {
                println!("{}", x);
                // print!("{}", x as u8 as char);
            }
        };
    }
}

fn options() {
    for i in 0..512 {
        // we do nothing if everything looks fine ahead
        if i & 0b11111 == 31 {
            continue;
        }
        // remove impossible 4 or more wide holes
        if i & 0b0000_1111 == 0
            || i & 0b0001_1110 == 0
            || i & 0b0011_1100 == 0
            || i & 0b0111_1000 == 0
            || i & 0b1111_0000 == 0
            || i & 0b1_1110_0000 == 0
        {
            continue;
        }
        // remove any where jump is death
        if i & 0b1000 == 0 {
            continue;
        }
        // remove any where after jump is death
        // if i & 0b1_0000 == 0 && i & 0b1000_0000 == 0 {
        //     continue;
        // }
        // if next if empty, we must jump
        if i & 0b1 == 0 {
            continue;
        }
        print_line(i);
    }
}

fn print_line(i: i32) {
    for x in 0..9 {
        if i & (1 << x) > 0 {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}
