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
    let f = File::open("day19.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut outputs: Vec<Vec<i64>> = Vec::new();
    let code = Intcode::new(&data);

    for y in 0..50 {
        let mut row = Vec::new();
        for x in 0..50 {
            match get(&code, x, y) {
                None => panic!("blah"),
                Some(o) => row.push(o),
            }
        }
        outputs.push(row);
    }

    for (y, row) in outputs.iter().enumerate() {
        print!("{:3} ", y);
        for (x, v) in row.iter().enumerate() {
            print!("{} ", v);
        }
        println!();
    }

    let affected = outputs
        .iter()
        .flat_map(|x| x.iter())
        .filter(|x| **x == 1)
        .count();
    println!("affected={}", affected);
}

fn get(code: &Intcode, x: i64, y: i64) -> Option<i64> {
    let mut code = code.clone();
    let inputs = &[x, y];

    // the input function
    let mut input_index = 0;
    let mut input_fn = || -> i64 {
        let i = input_index;
        input_index += 1;
        //println!("{}", inputs[i]);
        inputs[i]
    };

    match code.run(&mut input_fn) {
        ExitMode::Halt => None,
        ExitMode::Output(o) => Some(o),
    }
}
fn min(code: &Intcode, y: i64) -> i64 {
    let start = 4 * y / 5;
    let min = get(code, start, y);
    if min != Some(0) {
        panic!("missed");
    }
    for x in start..(start + 100) {
        let g = get(code, x, y);
        if g == Some(1) {
            return x;
        }
    }
    panic!("didn't find min");
}

fn max(code: &Intcode, y: i64) -> i64 {
    let start = y;
    let min = get(code, start, y);
    if min != Some(0) {
        panic!("missed");
    }
    for x in (0..start).rev() {
        let g = get(code, x, y);
        if g == Some(1) {
            return x;
        }
    }
    panic!("didn't find min");
}

pub fn part2() {
    let f = File::open("day19.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let code = Intcode::new(&data);

    let test = 1000;

    for i in test..(test + 1000) {
        let max = max(&code, i);
        let min = min(&code, i + 99);

        let diff = max - min;

        println!(
            "top: {} (max: {}), bottom: {} (min: {}), diff: {}",
            i,
            max,
            i + 99,
            min,
            diff
        );

        if diff == 99 {
            let x = max - 99;
            let y = i;
            let result = 10000 * x + y;
            println!("x: {}, y: {}, result: {}", x, y, result);
            break;
        }
    }
}
