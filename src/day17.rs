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
    let f = File::open("day17.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut code = Intcode::new(&data);

    let mut v: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();

    for _ in 0..10000 {
        match code.run(&|| 0) {
            ExitMode::Halt => {
                println!("halt");
                break;
            }
            ExitMode::Output(x) => {
                let c = match x {
                    35 => '#',
                    46 => '.',
                    10 => '\n',
                    94 => '^',
                    c => panic!("unknown {}", c),
                };
                if c == '\n' {
                    if !row.is_empty() {
                        v.push(row);
                    }
                    row = Vec::new();
                } else {
                    row.push(c);
                }
            }
        };
    }

    for r in v.iter() {
        let blah: String = r.iter().collect();
        println!("{}", blah);
    }

    let mut total = 0;

    for y in 1..(v.len() - 1) {
        for x in 1..(v[y].len() - 1) {
            if v[y][x] == '#'
                && v[y - 1][x] == '#'
                && v[y + 1][x] == '#'
                && v[y][x - 1] == '#'
                && v[y][x + 1] == '#'
            {
                println!("{}, {}", x, y);
                v[y][x] = 'O';
                total += x * y;
            }
        }
    }

    for r in v.iter() {
        let blah: String = r.iter().collect();
        println!("{}", blah);
    }

    println!("total={}", total);
}

pub fn part2() {
    let f = File::open("day17.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut code = Intcode::new(&data);
    // set into active mode
    code.set_imm(0, 2);

    // M: A,B,B,A,C,A,A,C,B,C
    // A: R,8,L,12,R,8
    // B: R,12,L,8,R,10
    // C: R,8,L,8,L,8,R,8,R,10

    let inputs: Vec<i64> =
        "A,B,B,A,C,A,A,C,B,C\nR,8,L,12,R,8\nR,12,L,8,R,10\nR,8,L,8,L,8,R,8,R,10\nn\n"
            .chars()
            .map(|x| x as i64)
            .collect();

    // the input function
    let mut input_index = 0;
    let mut input_fn = || -> i64 {
        let i = input_index;
        input_index += 1;
        println!("{}", inputs[i]);
        inputs[i]
    };

    println!("{:?}", inputs);

    for _ in 0..10_000_000 {
        match code.run(&mut input_fn) {
            ExitMode::Halt => {
                println!("halt");
                break;
            }
            ExitMode::Output(x) => {
                // print!("{}", x as u8 as char);
                println!("{}", x);
            }
        };
    }
}
