use crate::intcode::ExitMode;
use crate::intcode::Intcode;
use std::cmp;
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
    let f = File::open("day11.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut code = Intcode::new(&data);
    let mut map: HashMap<Point, i64> = HashMap::new();

    let mut p = Point { x: 0, y: 0 };
    // 0: ^
    // 1: >
    // 2: v
    // 3: <
    let mut direction = 0;

    loop {
        let colour = map.entry(p).or_insert(0);
        // get the colour
        match code.run(|| *colour) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => *colour = x,
        }
        // get the direction
        direction = match code.run(|| 0) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => match x {
                0 => (direction - 1 + 4) % 4,
                1 => (direction + 1) % 4,
                c => panic!("Invalid turn {}", c),
            },
        };
        // move
        match direction {
            0 => p.y += 1,
            1 => p.x += 1,
            2 => p.y -= 1,
            3 => p.x -= 1,
            c => panic!("Invalid direction {}", c),
        }
    }

    // for (key, val) in map.iter() {
    //     println!("{:?}: {}", key, val);
    // }
    println!("{}", map.len());
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
                    1 => print!("#"),
                    0 => print!(" "),
                    _ => panic!("aaaahhhhh"),
                },
                None => print!(" "),
            }
        }
        println!();
    }
}

pub fn part2() {
    let f = File::open("day11.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut code = Intcode::new(&data);
    let mut map: HashMap<Point, i64> = HashMap::new();

    let mut p = Point { x: 0, y: 0 };
    // 0: ^
    // 1: >
    // 2: v
    // 3: <
    let mut direction = 0;

    // insert a white initial point
    map.insert(p, 1);

    loop {
        let colour = map.entry(p).or_insert(0);
        // get the colour
        match code.run(|| *colour) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => *colour = x,
        }
        // get the direction
        direction = match code.run(|| 0) {
            ExitMode::Halt => break,
            ExitMode::Output(x) => match x {
                0 => (direction - 1 + 4) % 4,
                1 => (direction + 1) % 4,
                c => panic!("Invalid turn {}", c),
            },
        };
        // move
        match direction {
            0 => p.y += 1,
            1 => p.x += 1,
            2 => p.y -= 1,
            3 => p.x -= 1,
            c => panic!("Invalid direction {}", c),
        }
    }

    render(&map);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {}

    #[test]
    fn test_angle_and_dist() {}
}
