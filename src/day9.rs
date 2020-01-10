use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(PartialEq, Debug, Clone, Copy)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
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
    AdjustRelativeBase(ParamMode),
}

impl Op {
    fn new(i: i64) -> Self {
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
            9 => {
                let m = Self::modes(mode, 1);
                Op::AdjustRelativeBase(m[0])
            }
            c => panic!("invalid code {}", c),
        }
    }

    fn modes(m: i64, count: usize) -> Vec<ParamMode> {
        let mut v: Vec<ParamMode> = Vec::new();
        let mut m = m;
        for _ in 0..count {
            let rem = m / 10;
            let pm = match m - rem * 10 {
                0 => ParamMode::Position,
                1 => ParamMode::Immediate,
                2 => ParamMode::Relative,
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
    Output(i64),
}

struct Intcode {
    v: Vec<i64>,
    i: i64,
    relative_base: i64,
    memory: HashMap<i64, i64>,
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
            relative_base: 0,
            memory: HashMap::new(),
        }
    }

    fn string_to_vec(s: &str) -> Vec<i64> {
        s.trim()
            .split(',')
            .map(|x| x.parse().expect("failed to parse to int"))
            .collect()
    }

    fn get_mem(&self, i: i64) -> i64 {
        if i >= self.v.len() as i64 {
            // use memory
            *self.memory.get(&i).unwrap_or(&0)
        } else {
            self.v[i as usize]
        }
    }

    fn index(&self, i: i64, m: ParamMode) -> i64 {
        match m {
            // val is a reference to another value
            ParamMode::Position => self.get_mem(i),
            // val is the value
            ParamMode::Immediate => i,
            ParamMode::Relative => self.get_mem(i) + self.relative_base,
        }
    }

    fn get(&self, i: i64, m: ParamMode) -> i64 {
        self.get_mem(self.index(i, m))
    }

    fn set_mem(&mut self, i: i64, value: i64) {
        if i >= self.v.len() as i64 {
            // use memory
            self.memory.insert(i, value);
        } else {
            self.v[i as usize] = value;
        }
    }

    fn set(&mut self, i: i64, m: ParamMode, value: i64) {
        self.set_mem(self.index(i, m), value);
    }

    fn run(&mut self, input: &[i64]) -> ExitMode {
        let mut input_index = 0;
        // println!("-- start --\n{}", self.to_string());
        loop {
            let op = Op::new(self.v[self.i as usize]);
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
                    _ => self.i = self.get(self.i + 2, m2),
                },
                Op::JumpFalse(m1, m2) => match self.get(self.i + 1, m1) {
                    0 => self.i = self.get(self.i + 2, m2),
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
                Op::AdjustRelativeBase(m1) => {
                    let val = self.get(self.i + 1, m1);
                    self.relative_base += val;
                    self.i += 2;
                }
            };
            // println!("-- {} => {} --\n{}", ori, i, self.to_string());
        }
    }
}

pub fn part1() {
    let f = File::open("day9.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut code = Intcode::new(&data);

    while let ExitMode::Output(x) = code.run(&[1]) {
        println!("Output: {}", x);
    }
}

pub fn part2() {
    let f = File::open("day9.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut code = Intcode::new(&data);

    while let ExitMode::Output(x) = code.run(&[2]) {
        println!("Output: {}", x);
    }
}
