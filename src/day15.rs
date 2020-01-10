use crate::intcode::ExitMode;
use crate::intcode::Intcode;
use rand::prelude::*;
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

impl Point {
    fn next(&self, direction: i64) -> Self {
        match direction {
            // north
            1 => Self {
                x: self.x,
                y: self.y + 1,
            },
            // south
            2 => Self {
                x: self.x,
                y: self.y - 1,
            },
            // west
            3 => Self {
                x: self.x - 1,
                y: self.y,
            },
            // east
            4 => Self {
                x: self.x + 1,
                y: self.y,
            },
            c => panic!("unknown direction: {}", c),
        }
    }
}

fn moves_to_exit() {
    let map = "## ############### # ############### ####
#.#...............#.#...............#...#
#.#.#######.#####.#.#.#.###.#########.#.#
#.#.#.......#...#.#.#.#.#...#.........#.#
#.#.#.#######.###.#.#.#.#.###.#########.#
#.#.#.#.....#.....#...#.#.#...........#.#
#.#.#.#.#####.#######.#.###.###########.#
#...#.#.#.....#.....#.#.....#.....#.....#
#.###.#.#.#####.###.#######.#.###.#.###.#
#...#.#.#.#.....#...#.....#.#.#.#.#.#...#
###.#.#.#.###.###.###.###.###.#.#.#.#####
#...#.#...#...#.#.....#.#.#...#.#.#.#...#
#.###.#.###.###.#######.#.#.###.#.#.#.#.#
#.#...#.....#.........#...#.#...#.#...#.#
#.#.#########.#######.#.###.###.#.#.###.#
#.#.....#.....#.....#.#...#...#...#.#...#
#.#####.###.###.#.###.###.#.#.###.###.#.#
#.#...#...#...#.#...#.....#.#...#.....#.#
#.#.#.###.###.#####.###### ####.#######.#
#.#.#.#.#...#.............#...#.#.....#.#
#.#.#.#.###.#######.#####.#.#.#.#.#####.#
#...#.#.#...#.......#X#.....#...#...#...#
#####.#.#.#########.#.###########.#.#.###
#...#.#.#.........#.#.........#...#.#.#.#
###.#.#.#########.###########.#.###.#.#.#
#...#.#.....#.....#.....#...#.#.#...#...#
#.###.#####.#.#.###.#.###.#.#.#.#.#####.#
#...#.......#.#.#...#.....#...#.#.......#
#.#.#####.###.#.#.######## ####.#########
#.#.......#...#.#.........#...#.#.....#.#
#.#########.#.###########.###.#.#.###.#.#
#.....#.#...#.#.........#.#...#...#...#.#
#####.#.#.#####.#.#######.#.#######.###.#
#...#...#.#.....#.......#.#.......#.#...#
#.#.#.###.#.###########.#.#.###.#.#.###.#
#.#...#...#...#.....#...#.#.#.#.#.#...#.#
#.#####.#.###.#.#####.###.#.#.#.#####.#.#
#.....#.#.#...#.....#.#...#...#.....#...#
#####.#.###.#######.#.#.#####.#####.###.#
#O....#.............#...#.........#.....#
###### ############# ### ######### ######";
    let mut stuff: Vec<Vec<char>> = Vec::new();
    for line in map.split('\n') {
        stuff.push(
            line.chars()
                .map(|x| if x == ' ' { '#' } else { x })
                .collect(),
        );
    }

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);

    for _ in 0..100 {
        for y in 1..(stuff.len() - 1) {
            for x in 1..(stuff[y].len() - 1) {
                if stuff[y][x] == 'X' {
                    start = (y, x);
                }
                if stuff[y][x] == 'O' {
                    end = (y, x);
                }
                if stuff[y][x] == '.' {
                    let mut count = 0;
                    if stuff[y][x - 1] == '#' {
                        count += 1;
                    }
                    if stuff[y][x + 1] == '#' {
                        count += 1;
                    }
                    if stuff[y - 1][x] == '#' {
                        count += 1;
                    }
                    if stuff[y + 1][x] == '#' {
                        count += 1;
                    }
                    if count == 3 {
                        // dead end
                        stuff[y][x] = '#';
                    }
                }
            }
        }
    }

    for l in stuff.iter_mut() {
        let blah: String = l.iter().collect();
        println!("{}", blah);
    }
    println!("start={:?}", start);
    println!("end={:?}", end);

    // do it
    let mut i = 0;
    let mut pos = start;
    let mut last = end;
    loop {
        let possible = vec![
            (pos.0 - 1, pos.1),
            (pos.0 + 1, pos.1),
            (pos.0, pos.1 - 1),
            (pos.0, pos.1 + 1),
        ];
        let next = possible
            .iter()
            .filter(|x| **x != last)
            .find(|x| stuff[x.0][x.1] == '.');
        // move
        match next {
            None => break,
            Some(x) => {
                stuff[pos.0][pos.1] = '.';
                last = pos;
                pos = *x;
                stuff[pos.0][pos.1] = 'X';
            }
        }
        i += 1;

        // print
        println!("--");
        for l in stuff.iter_mut() {
            let blah: String = l.iter().collect();
            println!("{}", blah);
        }
    }
    println!("i={}", i);
}

fn fill_oxygen() {
    let map = "## ############### # ############### ####
#.#...............#.#...............#...#
#.#.#######.#####.#.#.#.###.#########.#.#
#.#.#.......#...#.#.#.#.#...#.........#.#
#.#.#.#######.###.#.#.#.#.###.#########.#
#.#.#.#.....#.....#...#.#.#...........#.#
#.#.#.#.#####.#######.#.###.###########.#
#...#.#.#.....#.....#.#.....#.....#.....#
#.###.#.#.#####.###.#######.#.###.#.###.#
#...#.#.#.#.....#...#.....#.#.#.#.#.#...#
###.#.#.#.###.###.###.###.###.#.#.#.#####
#...#.#...#...#.#.....#.#.#...#.#.#.#...#
#.###.#.###.###.#######.#.#.###.#.#.#.#.#
#.#...#.....#.........#...#.#...#.#...#.#
#.#.#########.#######.#.###.###.#.#.###.#
#.#.....#.....#.....#.#...#...#...#.#...#
#.#####.###.###.#.###.###.#.#.###.###.#.#
#.#...#...#...#.#...#.....#.#...#.....#.#
#.#.#.###.###.#####.###### ####.#######.#
#.#.#.#.#...#.............#...#.#.....#.#
#.#.#.#.###.#######.#####.#.#.#.#.#####.#
#...#.#.#...#.......#X#.....#...#...#...#
#####.#.#.#########.#.###########.#.#.###
#...#.#.#.........#.#.........#...#.#.#.#
###.#.#.#########.###########.#.###.#.#.#
#...#.#.....#.....#.....#...#.#.#...#...#
#.###.#####.#.#.###.#.###.#.#.#.#.#####.#
#...#.......#.#.#...#.....#...#.#.......#
#.#.#####.###.#.#.######## ####.#########
#.#.......#...#.#.........#...#.#.....#.#
#.#########.#.###########.###.#.#.###.#.#
#.....#.#...#.#.........#.#...#...#...#.#
#####.#.#.#####.#.#######.#.#######.###.#
#...#...#.#.....#.......#.#.......#.#...#
#.#.#.###.#.###########.#.#.###.#.#.###.#
#.#...#...#...#.....#...#.#.#.#.#.#...#.#
#.#####.#.###.#.#####.###.#.#.#.#####.#.#
#.....#.#.#...#.....#.#...#...#.....#...#
#####.#.###.#######.#.#.#####.#####.###.#
#O....#.............#...#.........#.....#
###### ############# ### ######### ######";
    let mut stuff: Vec<Vec<char>> = Vec::new();
    for line in map.split('\n') {
        stuff.push(
            line.chars()
                .map(|x| {
                    if x == ' ' {
                        '#'
                    } else if x == 'X' {
                        '.'
                    } else {
                        x
                    }
                })
                .collect(),
        );
    }

    for i in 0..1000 {
        let mut found_empty = false;
        let mut new_stuff = stuff.clone();
        for y in 1..(stuff.len() - 1) {
            for x in 1..(stuff[y].len() - 1) {
                if stuff[y][x] == '.' {
                    found_empty = true;
                    if stuff[y][x - 1] == 'O'
                        || stuff[y][x + 1] == 'O'
                        || stuff[y - 1][x] == 'O'
                        || stuff[y + 1][x] == 'O'
                    {
                        new_stuff[y][x] = 'O';
                    }
                }
            }
        }
        stuff = new_stuff;
        if !found_empty {
            break;
        }

        // print
        println!("-- i = {} --", i);
        for l in stuff.iter_mut() {
            let blah: String = l.iter().collect();
            println!("{}", blah);
        }
    }
}

pub fn part1() {
    // discover_map();
    moves_to_exit();
}

fn discover_map() {
    let f = File::open("day15.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut code = Intcode::new(&data);
    let mut map: HashMap<Point, i64> = HashMap::new();
    let mut location = Point { x: 0, y: 0 };
    let mut direction = 1;

    for _ in 0..1_000_000 {
        match code.run(|| direction) {
            ExitMode::Halt => {
                println!("halt");
                break;
            }
            ExitMode::Output(x) => {
                match x {
                    0 => {
                        // hit a wall
                        map.insert(location.next(direction), 0);
                    }
                    1 => {
                        // moved
                        location = location.next(direction);
                        map.insert(location, 1);
                    }
                    2 => {
                        // oxygen
                        map.insert(location.next(direction), 2);
                        location = location.next(direction);
                    }
                    c => panic!("bad code: {}", c),
                }
                // try to move somewhere we haven't been
                let options: Vec<&i64> = [1, 2, 3, 4]
                    .iter()
                    .filter(|d| map.get(&location.next(**d)) == None)
                    .collect();
                if options.len() > 0 {
                    direction = *options[rand::random::<usize>() % options.len()];
                } else {
                    direction = ((rand::random::<u8>() % 4) + 1) as i64;
                }
            }
        };
    }

    render(&map);
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

    println!("-----------------------------------------");

    for y in (y_min..=y_max).rev() {
        for x in x_min..=x_max {
            if x == 0 && y == 0 {
                print!("X");
            } else {
                match map.get(&Point { x, y }) {
                    Some(t) => match t {
                        0 => print!("#"),
                        1 => print!("."),
                        2 => print!("O"),
                        _ => panic!("unknown tile"),
                    },
                    None => print!(" "),
                }
            }
        }
        println!();
    }
}

pub fn part2() {
    fill_oxygen();
}
