#![allow(dead_code)]

use modinverse::modinverse;
use std::convert::TryInto;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Deck {
    v: Vec<i64>,
}

impl Deck {
    fn new(size: i64) -> Self {
        let mut v = Vec::new();
        for i in 0..size {
            v.push(i);
        }
        Self { v }
    }

    fn rev(&mut self) {
        self.v.reverse();
    }

    fn cut(&mut self, x: i64) {
        let mut x = x;
        while x < 0 {
            x += self.v.len() as i64;
        }
        let x = x as usize;

        let mut v = Vec::new();
        for i in 0..self.v.len() {
            let index = (i + x) % self.v.len();
            v.push(self.v[index]);
        }
        self.v = v;
    }

    fn deal(&mut self, x: i64) {
        let mut v = vec![0; self.v.len()];
        for i in 0..self.v.len() {
            let index = (i * x as usize) % self.v.len();
            v[index] = self.v[i];
        }
        self.v = v;
    }

    fn result(&self) -> Vec<i64> {
        self.v.clone()
    }
}

struct Deck2 {
    size: i64,
    step: i64,
    first: i64,
}

impl Deck2 {
    fn new(size: i64) -> Self {
        Self {
            size,
            step: 1,
            first: 0,
        }
    }

    fn rev(&mut self) {
        // self.first = self.size - self.first - 1;
        let mut first = self.first;

        // work out the last element
        for _ in 0..(self.size - 1) {
            first += self.step;
            while first < 0 {
                first += self.size
            }
            first %= self.size;
        }
        self.first = first;

        self.step = -self.step;
    }

    fn cut(&mut self, i: i64) {
        self.first = (self.first + i * self.step) % self.size;
        if self.first < 0 {
            self.first += self.size;
        }
    }

    fn deal(&mut self, i: i64) {
        // next value is the value for x where
        // (i*x) mod size = 1
        // below is the naive way

        let mut step = 0;
        for x in 0..self.size {
            if (i * x) % self.size == 1 {
                step = x;
                break;
            }
        }
        if step == 0 {
            panic!("didn't find it");
        }

        self.step = (self.step * step) % self.size;
    }

    fn result(&self) -> Vec<i64> {
        let mut v = Vec::new();
        let mut pos = self.first;

        for _ in 0..self.size {
            v.push(pos);
            pos += self.step;
            while pos < 0 {
                pos += self.size
            }
            pos %= self.size;
        }

        v
    }
}

struct Deck3 {
    size: i64,
    pos: i64,
}

impl Deck3 {
    fn new(size: i64, card: i64) -> Self {
        Self { size, pos: card }
    }

    fn rev(&mut self) {
        self.pos = -self.pos - 1;
    }

    fn cut(&mut self, i: i64) {
        self.pos -= i;
        self.pos %= self.size;
    }

    fn deal(&mut self, i: i64) {
        self.pos = (self.pos * i) % self.size;
    }

    fn result(&self) -> i64 {
        let mut pos = self.pos;
        while pos < 0 {
            pos += self.size;
        }
        pos
    }
}

// f(x) = ax + b
struct Deck4 {
    a: u64,
    b: u64,
    size: u64,
}

impl Deck4 {
    fn new(size: u64) -> Self {
        Self { a: 1, b: 0, size }
    }

    fn rev(&mut self) {
        self.a = self.size - self.a;
        self.b = self.size - self.b - 1;
    }

    fn cut(&mut self, i: i64) {
        let i = self.to_u64(i);
        while self.b < i {
            self.b += self.size;
        }
        self.b -= i;
        self.b %= self.size;
    }

    fn deal(&mut self, i: u64) {
        self.a = (self.a * i) % self.size;
        self.b = (self.b * i) % self.size;
    }

    fn pos(&self, card: u64) -> u64 {
        let r = self.a as u128 * card as u128 + self.b as u128;
        (r % self.size as u128) as u64
    }

    fn inverse(&self) -> Self {
        // pos = a * card + b % self.size
        // therefore:
        // a * card = pos - b % self.size
        // card = (pos - b) * inverse(a) % self.size
        // card = inverse(a) * pos - b * inverse(a) % self.size

        // modinverse apparently uses negative numbers, so pass as i64
        let inv_a = modinverse(self.a as i64, self.size as i64).unwrap() as u64;
        let b = ((self.size - self.b) as u128 * inv_a as u128 % self.size as u128) as u64;
        Self {
            a: inv_a,
            b: b % self.size,
            size: self.size,
        }
    }

    fn card(&self, pos: u64) -> u64 {
        let inv = self.inverse();
        inv.pos(pos)
    }

    fn to_u64(&self, x: i64) -> u64 {
        let mut x = x;
        while x < 0 {
            x += self.size as i64;
        }
        x.try_into().unwrap()
    }

    fn multiply(&self, rounds: u64) -> Self {
        // e.g. round 3 = a^3x + b*(a^2 + a + 1)
        // where a is postion and b is scalar
        let (a, b) = matrix_pow(self.a, rounds, self.size);
        let b = ((self.b as u128 * b as u128) % self.size as u128) as u64;
        mod_pow(1, 1, 1);

        Self {
            a,
            b,
            size: self.size,
        }
    }
}

struct Deck5 {
    a: u64,
    b: u64,
    size: u64,
}

impl Deck5 {
    fn new(size: u64) -> Self {
        Self { a: 1, b: 0, size }
    }

    fn rev(&mut self) {
        self.a = self.size - self.a;
        self.b = self.size - self.b - 1;
    }

    fn cut(&mut self, i: i64) {
        let i = self.to_u64(i);
        self.b += (i as u128 * self.a as u128) as u64;
        self.b %= self.size;
    }

    fn deal(&mut self, i: u64) {
        self.a = ((self.a as u128 * (self.size - i) as u128) % self.size as u128) as u64;
        // self.b = (self.b + i) % self.size;
    }

    fn card(&self, pos: u64) -> u64 {
        let r = self.a as u128 * pos as u128 + self.b as u128;
        (r % self.size as u128) as u64
    }

    fn to_u64(&self, x: i64) -> u64 {
        let mut x = x;
        while x < 0 {
            x += self.size as i64;
        }
        x.try_into().unwrap()
    }

    fn multiply(&self, rounds: u64) -> Self {
        // e.g. round 3 = a^3x + b*(a^2 + a + 1)
        // where a is postion and b is scalar
        let (a, b) = matrix_pow(self.a, rounds, self.size);
        let b = ((self.b as u128 * b as u128) % self.size as u128) as u64;

        Self {
            a,
            b,
            size: self.size,
        }
    }
}

// https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_pow(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut base = base % modulus;
    let mut exp = exp;
    let mut result = 1;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result as u128 * base as u128 % modulus as u128) as u64;
        }
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        exp >>= 1;
    }
    result
}

// https://stackoverflow.com/questions/1522825/calculating-sum-of-geometric-series-mod-m
fn geometric(base: u64, n: u64, modulus: u64) -> u64 {
    let mut base = base % modulus;
    let mut n = n;
    let mut t = 1;
    let mut result = 0;
    while n > 0 {
        if n % 2 == 1 {
            result = ((result as u128 * base as u128 + t as u128) % modulus as u128) as u64;
        }
        t = (((base + 1) as u128 * t as u128) % modulus as u128) as u64;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        n >>= 1;
    }
    result
}

// [a 1]^n mod modulus
// [0 1]
fn matrix_pow(a: u64, n: u64, modulus: u64) -> (u64, u64) {
    let mut base = a % modulus;
    let mut n = n;
    let mut t = 1;
    // the result collectors
    let mut ra = 1;
    let mut rb = 0;
    while n > 0 {
        if n % 2 == 1 {
            ra = (ra as u128 * base as u128 % modulus as u128) as u64;
            rb = ((rb as u128 * base as u128 + t as u128) % modulus as u128) as u64;
        }
        t = (((base + 1) as u128 * t as u128) % modulus as u128) as u64;
        base = ((base as u128 * base as u128) % modulus as u128) as u64;
        n >>= 1;
    }
    (ra, rb)
}

enum Op {
    Cut(i64),
    Deal(u64),
    Reverse,
}

fn get_ops() -> Vec<Op> {
    let f = File::open("day22.txt").unwrap();
    let f = BufReader::new(f);

    let mut ops: Vec<Op> = Vec::new();

    for line in f.lines() {
        let l = line.unwrap();
        let parts: Vec<&str> = l.trim().split(' ').collect();

        // deal with increment 30
        // cut 6056
        // deal into new stack

        if parts.len() == 2 {
            let x: i64 = parts[1].parse().unwrap();
            ops.push(Op::Cut(x));
        } else if parts.len() == 4 {
            if parts[1] == "with" {
                let x: u64 = parts[3].parse().unwrap();
                ops.push(Op::Deal(x));
            } else if parts[1] == "into" {
                ops.push(Op::Reverse);
            } else {
                panic!("bad line: {}", l);
            }
        } else {
            panic!("bad line: {}", l);
        }
    }
    ops
}

pub fn part1() {
    let ops = get_ops();
    let mut d = Deck4::new(10007);

    for op in ops.iter() {
        match op {
            Op::Reverse => d.rev(),
            Op::Cut(x) => d.cut(*x),
            Op::Deal(x) => d.deal(*x),
        }
    }

    println!("pos: {}, scaler: {}", d.a, d.a);
    println!("result: {}", d.pos(2019));
}

pub fn part2() {
    let ops = get_ops();

    let cards = 119_315_717_514_047;
    let rounds = 101_741_582_076_661;

    let mut d = Deck4::new(cards);
    for op in ops.iter() {
        match op {
            Op::Reverse => d.rev(),
            Op::Cut(x) => d.cut(*x),
            Op::Deal(x) => d.deal(*x),
        }
    }

    println!("single: a: {}, b: {}", d.a, d.b);

    let m = d.multiply(rounds);
    println!("multiple: a: {}, b: {}", m.a, m.b);
    let i = m.inverse();
    println!("inverse: a: {}, b: {}", i.a, i.b);

    println!("card: {}", i.pos(2020));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rev() {
        let mut d = Deck2::new(10);
        d.rev();
        let r = d.result();
        assert_eq!(&r, &[9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_cut() {
        let mut d = Deck::new(10);
        d.cut(3);
        let r = d.result();
        assert_eq!(&r, &[3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_cut_negative() {
        let mut d = Deck::new(10);
        d.cut(-4);
        let r = d.result();
        assert_eq!(&r, &[6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal() {
        let mut d = Deck::new(10);
        d.deal(3);
        let r = d.result();
        assert_eq!(&r, &[0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }

    #[test]
    fn test_comb1() {
        let mut d = Deck::new(10);
        d.deal(7);
        d.rev();
        d.rev();
        let r = d.result();
        assert_eq!(&r, &[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_comb2() {
        let mut d = Deck::new(10);
        d.cut(6);
        d.deal(7);
        d.rev();
        let r = d.result();
        assert_eq!(&r, &[3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_comb3() {
        let mut d = Deck::new(10);
        d.deal(7);
        d.deal(9);
        d.cut(-2);
        let r = d.result();
        assert_eq!(&r, &[6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_comb4() {
        let mut d = Deck2::new(10);
        d.rev();
        d.cut(-2);
        d.deal(7);
        d.cut(8);
        d.cut(-4);
        d.deal(7);
        d.cut(3);
        d.deal(9);
        d.deal(3);
        d.cut(-1);
        let r = d.result();
        assert_eq!(&r, &[9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }

    #[test]
    fn test_deck3() {
        let mut d = Deck3::new(10, 7);
        d.rev();
        d.cut(-2);
        d.deal(7);
        d.cut(8);
        d.cut(-4);
        d.deal(7);
        d.cut(3);
        d.deal(9);
        d.deal(3);
        d.cut(-1);
        assert_eq!(d.result(), 6);
    }

    #[test]
    fn test_deck4() {
        let mut d = Deck4::new(10);
        d.rev();
        d.cut(-2);
        d.deal(7);
        d.cut(8);
        d.cut(-4);
        d.deal(7);
        d.cut(3);
        d.deal(9);
        d.deal(3);
        d.cut(-1);
        assert_eq!(d.pos(7), 6);
    }

    #[test]
    fn test_mod_pow() {
        let x = mod_pow(28, 10, 47);
        assert_eq!(4, x);
    }

    #[test]
    fn test_geometric() {
        // the calculates for powers up to but not including 5
        // 1 + 3 + 3^2 + 3^3 + 3^4 mod 7
        // 1 + 3 + 9 + 27 + 81 mod 7
        // 121 mod 7 => 2
        let x = geometric(3, 5, 7);
        assert_eq!(2, x);
    }

    #[test]
    fn deck5_test_rev() {
        let mut d = Deck5::new(10);
        d.rev();
        assert_eq!(2, d.card(7));
    }

    #[test]
    fn deck5_test_cut() {
        let mut d = Deck5::new(10);
        d.cut(-4);
        assert_eq!(1, d.card(5));
    }

    #[test]
    fn deck5_test_deal() {
        let mut d = Deck5::new(10);
        d.deal(3);
        assert_eq!(8, d.card(4));
    }

    #[test]
    fn test_deck5() {
        let mut d = Deck5::new(10);
        d.rev();
        for i in 0..10 {
            print!("{} ", d.card(i));
        }
        println!();
        d.cut(-2);
        for i in 0..10 {
            print!("{} ", d.card(i));
        }
        println!();
        d.deal(7);
        for i in 0..10 {
            print!("{} ", d.card(i));
        }
        println!();
        d.cut(8);
        for i in 0..10 {
            print!("{} ", d.card(i));
        }
        println!();
        d.cut(-4);
        for i in 0..10 {
            print!("{} ", d.card(i));
        }
        println!();
        d.deal(7);
        for i in 0..10 {
            print!("{} ", d.card(i));
        }
        println!();
        d.cut(3);
        for i in 0..10 {
            print!("{} ", d.card(i));
        }
        println!();
        d.deal(9);
        for i in 0..10 {
            print!("{} ", d.card(i));
        }
        println!();
        d.deal(3);
        for i in 0..10 {
            print!("{} ", d.card(i));
        }
        println!();
        d.cut(-1);
        for i in 0..10 {
            print!("{} ", d.card(i));
        }
        println!();
        // extra
        d.rev();
        for i in 0..10 {
            print!("{} ", d.card(i));
        }
        println!();
        // assert_eq!(9, d.card(0));
        // assert_eq!(2, d.card(1));
        // assert_eq!(7, d.card(6));
    }
}
