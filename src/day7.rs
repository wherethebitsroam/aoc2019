use permutohedron::Heap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(PartialEq, Debug, Clone, Copy)]
enum ParamMode {
    Position,
    Immediate,
}

enum Op {
    Halt,
    Add(ParamMode, ParamMode, ParamMode),
    Mul(ParamMode, ParamMode, ParamMode),
    Input(ParamMode),
    Output(ParamMode),
    JumpTrue(ParamMode, ParamMode),
    JumpFalse(ParamMode, ParamMode),
    LessThan(ParamMode, ParamMode, ParamMode),
    Equals(ParamMode, ParamMode, ParamMode),
}

impl Op {
    fn new(i: i32) -> Self {
        let mode = i / 100;
        let code = i - mode * 100;
        match code {
            99 => Op::Halt,
            1 => {
                let m = Self::modes(mode, 3);
                Op::Add(m[0], m[1], m[2])
            }
            2 => {
                let m = Self::modes(mode, 3);
                Op::Mul(m[0], m[1], m[2])
            }
            3 => {
                let m = Self::modes(mode, 1);
                Op::Input(m[0])
            }
            4 => {
                let m = Self::modes(mode, 1);
                Op::Output(m[0])
            }
            5 => {
                let m = Self::modes(mode, 2);
                Op::JumpTrue(m[0], m[1])
            }
            6 => {
                let m = Self::modes(mode, 2);
                Op::JumpFalse(m[0], m[1])
            }
            7 => {
                let m = Self::modes(mode, 3);
                Op::LessThan(m[0], m[1], m[2])
            }
            8 => {
                let m = Self::modes(mode, 3);
                Op::Equals(m[0], m[1], m[2])
            }
            c => panic!("invalid code {}", c),
        }
    }

    fn modes(m: i32, count: i32) -> Vec<ParamMode> {
        let mut v: Vec<ParamMode> = Vec::new();
        let mut m = m;
        for _ in 0..count {
            let rem = m / 10;
            let pm = match m - rem * 10 {
                0 => ParamMode::Position,
                1 => ParamMode::Immediate,
                c => panic!("invalid mode {}", c),
            };
            v.push(pm);
            m = rem;
        }

        v
    }
}

enum ExitMode {
    Halt,
    Output(i32),
}

struct Intcode {
    v: Vec<i32>,
    i: usize,
}

impl fmt::Display for Intcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: Vec<String> = self.v.iter().map(|x| x.to_string()).collect();
        write!(f, "{}", s.join(","))
    }
}

impl Intcode {
    fn new(s: &str) -> Self {
        Intcode {
            v: Self::string_to_vec(s),
            i: 0,
        }
    }

    fn string_to_vec(s: &str) -> Vec<i32> {
        s.trim()
            .split(',')
            .map(|x| x.parse().expect("failed to parse to int"))
            .collect()
    }

    fn get(&self, i: usize, m: ParamMode) -> i32 {
        let val = self.v[i];
        match m {
            // val is a reference to another value
            ParamMode::Position => self.v[val as usize],
            // val is the value
            ParamMode::Immediate => val,
        }
    }

    fn set(&mut self, i: usize, m: ParamMode, value: i32) {
        match m {
            ParamMode::Position => {
                let x = self.v[i] as usize;
                self.v[x] = value;
            }
            ParamMode::Immediate => panic!("write should never be in immediate mode"),
        }
    }

    fn run(&mut self, input: &[i32]) -> ExitMode {
        let mut input_index = 0;
        // println!("-- start --\n{}", self.to_string());
        loop {
            let op = Op::new(self.v[self.i]);
            match op {
                Op::Halt => return ExitMode::Halt,
                Op::Add(am, bm, rm) => {
                    let a = self.get(self.i + 1, am);
                    let b = self.get(self.i + 2, bm);
                    self.set(self.i + 3, rm, a + b);
                    self.i += 4;
                }
                Op::Mul(am, bm, rm) => {
                    let a = self.get(self.i + 1, am);
                    let b = self.get(self.i + 2, bm);
                    self.set(self.i + 3, rm, a * b);
                    self.i += 4;
                }
                Op::Input(im) => {
                    if input_index >= input.len() {
                        panic!(
                            "not enough inputs. index: {}, len: {}",
                            input_index,
                            input.len()
                        );
                    }

                    // println!("Step: {}, inserting input: {}", i, input);
                    self.set(self.i + 1, im, input[input_index]);
                    input_index += 1;
                    self.i += 2;
                }
                Op::Output(om) => {
                    // println!("output: {}", self.get(i + 1, om));
                    let output = self.get(self.i + 1, om);
                    self.i += 2;
                    return ExitMode::Output(output);
                }
                Op::JumpTrue(m1, m2) => match self.get(self.i + 1, m1) {
                    0 => self.i += 3,
                    _ => self.i = self.get(self.i + 2, m2) as usize,
                },
                Op::JumpFalse(m1, m2) => match self.get(self.i + 1, m1) {
                    0 => self.i = self.get(self.i + 2, m2) as usize,
                    _ => self.i += 3,
                },
                Op::LessThan(m1, m2, m3) => {
                    let val = if self.get(self.i + 1, m1) < self.get(self.i + 2, m2) {
                        1
                    } else {
                        0
                    };
                    self.set(self.i + 3, m3, val);
                    self.i += 4;
                }
                Op::Equals(m1, m2, m3) => {
                    let val = if self.get(self.i + 1, m1) == self.get(self.i + 2, m2) {
                        1
                    } else {
                        0
                    };
                    self.set(self.i + 3, m3, val);
                    self.i += 4;
                }
            };
            // println!("-- {} => {} --\n{}", ori, i, self.to_string());
        }
    }
}

fn amplify(data: &str, phases: &[i32], input: i32) -> i32 {
    let mut input = input;
    for p in phases.iter() {
        let mut code = Intcode::new(data);
        loop {
            match code.run(&[*p, input]) {
                ExitMode::Halt => break,
                ExitMode::Output(x) => input = x,
            }
        }
    }

    input
}

fn amplify2(data: &str, phases: &[i32], input: i32) -> i32 {
    let mut a = Intcode::new(data);
    let mut b = Intcode::new(data);
    let mut c = Intcode::new(data);
    let mut d = Intcode::new(data);
    let mut e = Intcode::new(data);

    let mut signal = input;
    // initial loop
    match a.run(&[phases[0], signal]) {
        ExitMode::Halt => panic!("unexpected halt"),
        ExitMode::Output(x) => signal = x,
    }
    match b.run(&[phases[1], signal]) {
        ExitMode::Halt => panic!("unexpected halt"),
        ExitMode::Output(x) => signal = x,
    }
    match c.run(&[phases[2], signal]) {
        ExitMode::Halt => panic!("unexpected halt"),
        ExitMode::Output(x) => signal = x,
    }
    match d.run(&[phases[3], signal]) {
        ExitMode::Halt => panic!("unexpected halt"),
        ExitMode::Output(x) => signal = x,
    }
    match e.run(&[phases[4], signal]) {
        ExitMode::Halt => panic!("unexpected halt"),
        ExitMode::Output(x) => signal = x,
    }

    // continuous loop
    loop {
        match a.run(&[signal]) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => signal = x,
        }
        match b.run(&[signal]) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => signal = x,
        }
        match c.run(&[signal]) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => signal = x,
        }
        match d.run(&[signal]) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => signal = x,
        }
        match e.run(&[signal]) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => signal = x,
        }
    }

    signal
}

fn best(data: &str, mut phases: Vec<i32>, input: i32) -> i32 {
    let heap = Heap::new(&mut phases);
    let mut b = 0;
    for ps in heap {
        let output = amplify(data, &ps, input);
        if output > b {
            b = output;
        }
    }
    b
}

fn best2(data: &str, mut phases: Vec<i32>, input: i32) -> i32 {
    let heap = Heap::new(&mut phases);
    let mut b = 0;
    for ps in heap {
        let output = amplify2(data, &ps, input);
        if output > b {
            b = output;
        }
    }
    b
}

pub fn part1() {
    let f = File::open("day7.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    println!("best: {}", best(&data, vec![0, 1, 2, 3, 4], 0));
}

pub fn part2() {
    let f = File::open("day7.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    println!("best: {}", best2(&data, vec![9, 8, 7, 6, 5], 0));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplify() {
        assert_eq!(
            43210,
            best(
                "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0",
                vec!(0, 1, 2, 3, 4),
                0
            )
        );
    }

    #[test]
    fn test_amplify2() {
        assert_eq!(
            139629729,
            amplify2(
                "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
                &[9,8,7,6,5],
                0
            )
        );
    }
}
