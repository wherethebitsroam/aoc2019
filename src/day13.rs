use crate::intcode::ExitMode;
use crate::intcode::Intcode;
use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct Point {
    x: i64,
    y: i64,
}

pub fn part1() {
    let f = File::open("day13.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut code = Intcode::new(&data);
    let mut map: HashMap<Point, i64> = HashMap::new();

    loop {
        let x = match code.run(|| 0) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => x,
        };
        let y = match code.run(|| 0) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => x,
        };
        let tile = match code.run(|| 0) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => x,
        };
        map.insert(Point { x, y }, tile);
    }

    let mut count = 0;
    for (_, val) in map.iter() {
        if let 2 = val {
            count += 1
        };
    }
    println!("{}", count);
}

fn render(map: &HashMap<Point, i64>) {
    // get the min and max
    let (x_min, x_max, y_min, y_max) = map.keys().fold(
        (std::i64::MAX, std::i64::MIN, std::i64::MAX, std::i64::MIN),
        |acc, p| {
            (
                cmp::min(acc.0, p.x),
                cmp::max(acc.1, p.x),
                cmp::min(acc.2, p.y),
                cmp::max(acc.3, p.y),
            )
        },
    );

    for y in (y_min..=y_max).rev() {
        for x in x_min..=x_max {
            match map.get(&Point { x, y }) {
                Some(x) => match x {
                    0 => print!(" "),
                    1 => print!("|"),
                    2 => print!("#"),
                    3 => print!("_"),
                    4 => print!("o"),
                    _ => panic!("unknown tile"),
                },
                None => print!(" "),
            }
        }
        println!();
    }
}

pub fn part2() {
    let f = File::open("day13.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut code = Intcode::new(&data);
    let mut map: HashMap<Point, i64> = HashMap::new();
    let mut score = 0;

    let mut ball = Point { x: 0, y: 0 };
    let mut paddle = Point { x: 0, y: 0 };
    let mut joystick: i64 = 0;

    // let mut siv = Cursive::default();
    // siv.add_global_callback('q', |s| s.quit());

    code.set_imm(0, 2);

    loop {
        let x = match code.run(|| joystick) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => x,
        };
        let y = match code.run(|| 0) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => x,
        };
        let tile = match code.run(|| 0) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => x,
        };
        if x == -1 && y == 0 {
            score = tile;
        } else {
            map.insert(Point { x, y }, tile);
        }

        if tile == 4 {
            ball = Point { x, y };
        }
        if tile == 3 {
            paddle = Point { x, y };
        }

        match ball.x.cmp(&paddle.x) {
            Ordering::Equal => joystick = 0,
            Ordering::Less => joystick = -1,
            Ordering::Greater => joystick = 1,
        };
        // println!("b: {:?}, p: {:?}, j: {}", b, p, joystick);

        // render(&map);
    }
    println!("{}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {}

    #[test]
    fn test_angle_and_dist() {}
}
