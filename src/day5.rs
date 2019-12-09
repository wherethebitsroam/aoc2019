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

struct Intcode {
    input: i32,
    v: Vec<i32>,
}

impl fmt::Display for Intcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: Vec<String> = self.v.iter().map(|x| x.to_string()).collect();
        write!(f, "{}", s.join(","))
    }
}

impl Intcode {
    fn new(s: &str, input: i32) -> Self {
        Intcode {
            input,
            v: Self::string_to_vec(s),
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

    fn run(&mut self) {
        // println!("-- start --\n{}", self.to_string());
        let mut i = 0;
        loop {
            let op = Op::new(self.v[i]);
            match op {
                Op::Halt => break,
                Op::Add(am, bm, rm) => {
                    let a = self.get(i + 1, am);
                    let b = self.get(i + 2, bm);
                    self.set(i + 3, rm, a + b);
                    i += 4;
                }
                Op::Mul(am, bm, rm) => {
                    let a = self.get(i + 1, am);
                    let b = self.get(i + 2, bm);
                    self.set(i + 3, rm, a * b);
                    i += 4;
                }
                Op::Input(im) => {
                    println!("Step: {}, inserting input: {}", i, self.input);
                    self.set(i + 1, im, self.input);
                    i += 2;
                }
                Op::Output(om) => {
                    println!("output: {}", self.get(i + 1, om));
                    i += 2;
                }
                Op::JumpTrue(m1, m2) => match self.get(i + 1, m1) {
                    0 => i += 3,
                    _ => i = self.get(i + 2, m2) as usize,
                },
                Op::JumpFalse(m1, m2) => match self.get(i + 1, m1) {
                    0 => i = self.get(i + 2, m2) as usize,
                    _ => i += 3,
                },
                Op::LessThan(m1, m2, m3) => {
                    let val = if self.get(i + 1, m1) < self.get(i + 2, m2) {
                        1
                    } else {
                        0
                    };
                    self.set(i + 3, m3, val);
                    i += 4;
                }
                Op::Equals(m1, m2, m3) => {
                    let val = if self.get(i + 1, m1) == self.get(i + 2, m2) {
                        1
                    } else {
                        0
                    };
                    self.set(i + 3, m3, val);
                    i += 4;
                }
            };
            // println!("-- {} => {} --\n{}", ori, i, self.to_string());
        }
    }
}

pub fn part1() {
    // let f = File::open("day5.txt").expect("file not found");
    // let mut f = BufReader::new(f);
    // let mut data = String::new();
    // f.read_to_string(&mut data).expect("failed to read string");

    // let mut i = Intcode::new(&data, 1);
    // i.run();
}

pub fn part2() {
    let f = File::open("day5.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut i = Intcode::new(&data, 5);
    i.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modes() {
        let modes = Op::modes(10, 3);
        assert_eq!(ParamMode::Position, modes[0]);
        assert_eq!(ParamMode::Immediate, modes[1]);
        assert_eq!(ParamMode::Position, modes[2]);

        let modes = Op::modes(110, 3);
        assert_eq!(ParamMode::Immediate, modes[0]);
        assert_eq!(ParamMode::Immediate, modes[1]);
        assert_eq!(ParamMode::Position, modes[2]);
    }

    #[test]
    fn test_calculate() {
        assert_eq!("2,0,0,0,99", calculate("1,0,0,0,99"));
        assert_eq!("2,3,0,6,99", calculate("2,3,0,3,99"));
        assert_eq!("2,4,4,5,99,9801", calculate("2,4,4,5,99,0"));
        assert_eq!("30,1,1,4,2,5,6,0,99", calculate("1,1,1,4,99,5,6,0,99"));
    }

    fn calculate(s: &str) -> String {
        let mut i = Intcode::new(s, 0);
        i.run();
        i.to_string()
    }

    #[test]
    fn test_part2() {}
}
