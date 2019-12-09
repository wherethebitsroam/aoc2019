use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn man_dist(self, p: Point) -> i32 {
        (self.x - p.x).abs() + (self.y - p.y).abs()
    }

    fn vbetween(self, a: Point, b: Point) -> bool {
        (self.x >= a.x && self.x <= b.x) || (self.x >= b.x && self.x <= a.x)
    }
    fn hbetween(self, a: Point, b: Point) -> bool {
        (self.y >= a.y && self.y <= b.y) || (self.y >= b.y && self.y <= a.y)
    }
}

#[derive(PartialEq)]
enum Direction {
    Horizontal,
    Vertical,
}

struct Line {
    a: Point,
    b: Point,
}

impl Line {
    fn direction(&self) -> Direction {
        if self.a.x == self.b.x {
            Direction::Vertical
        } else if self.a.y == self.b.y {
            Direction::Horizontal
        } else {
            panic!("angled line");
        }
    }

    fn length(&self) -> i32 {
        self.a.man_dist(self.b)
    }

    fn intersection(&self, l: &Self) -> Option<Point> {
        // assume we need vertical and horizontal for intersection
        if self.direction() == l.direction() {
            return None;
        }

        match self.direction() {
            Direction::Horizontal => {
                if l.a.vbetween(self.a, self.b) && self.a.hbetween(l.a, l.b) {
                    return Some(Point {
                        x: l.a.x,
                        y: self.a.y,
                    });
                }
            }
            Direction::Vertical => {
                if l.a.hbetween(self.a, self.b) && self.a.vbetween(l.a, l.b) {
                    return Some(Point {
                        y: l.a.y,
                        x: self.a.x,
                    });
                }
            }
        }

        None
    }
}

pub fn part1() {
    let f = File::open("day3.txt").expect("file not found");
    let f = BufReader::new(f);

    let lines: Vec<String> = f.lines().map(|x| x.unwrap()).collect();

    let dist = dist(&lines[0], &lines[1]);
    println!("dist: {}", dist);
}

fn dist(l1: &str, l2: &str) -> i32 {
    let l1 = process_line(l1);
    let l2 = process_line(l2);

    let mut min_dist = 10_000_000;
    let origin = Point { x: 0, y: 0 };

    for s1 in l1.iter() {
        for s2 in l2.iter() {
            if let Some(p) = s1.intersection(s2) {
                // println!("Intersection: x: {}, y: {}", p.x, p.y);
                let d = p.man_dist(origin);
                if d < min_dist && d > 0 {
                    min_dist = d;
                }
            }
        }
    }

    min_dist
}

fn steps(l1: &str, l2: &str) -> i32 {
    let l1 = process_line(l1);
    let l2 = process_line(l2);

    let mut min_steps = 10_000_000;

    let mut s1_steps = 0;
    let mut s2_steps;

    for s1 in l1.iter() {
        s2_steps = 0;
        for s2 in l2.iter() {
            if let Some(p) = s1.intersection(s2) {
                // println!("Intersection: x: {}, y: {}, s1_steps: {}, extra: {}, s2_steps: {}, extra: {}", p.x, p.y, s1_steps, p.man_dist(&s1.a), s2_steps, p.man_dist(&s2.a));
                // we know 'a' it the start point of the segment
                let s1_total = s1_steps + p.man_dist(s1.a);
                let s2_total = s2_steps + p.man_dist(s2.a);
                let t = s1_total + s2_total;
                if t < min_steps && t > 0 {
                    min_steps = t;
                }
            }
            // add the length of segment
            s2_steps += s2.length()
        }
        // add the length of segment
        s1_steps += s1.length()
    }

    min_steps
}

fn process_line(line: &str) -> Vec<Line> {
    let mut ls: Vec<Line> = Vec::new();
    let mut p = Point { x: 0, y: 0 };

    for op in line.split(',') {
        let dir = &op[0..1];
        let d: i32 = op[1..].parse().unwrap();
        // println!("dir: {}, dist: {}", dir, d);

        let e = match dir {
            "U" => Point { y: p.y + d, x: p.x },
            "D" => Point { y: p.y - d, x: p.x },
            "L" => Point { y: p.y, x: p.x - d },
            "R" => Point { y: p.y, x: p.x + d },
            c => panic!("invalid dir: {}", c),
        };

        ls.push(Line { a: p, b: e });

        p = e;
    }
    ls
}

pub fn part2() {
    let f = File::open("day3.txt").expect("file not found");
    let f = BufReader::new(f);

    let lines: Vec<String> = f.lines().map(|x| x.unwrap()).collect();

    let dist = steps(&lines[0], &lines[1]);
    println!("steps: {}", dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(6, dist("R8,U5,L5,D3", "U7,R6,D4,L4"));
        assert_eq!(
            159,
            dist(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            )
        );
        assert_eq!(
            135,
            dist(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(30, steps("R8,U5,L5,D3", "U7,R6,D4,L4"));
        assert_eq!(
            610,
            steps(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            )
        );
        assert_eq!(
            410,
            steps(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            )
        );
    }
}
