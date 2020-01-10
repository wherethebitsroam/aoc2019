use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    // we times by 1000 to give approx equality for f64
    // we really want to test that the difference is less than f64::EPSILON
    fn angle(&self, p: &Self) -> i64 {
        let d = p.diff(self);
        let angle = (d.x as f64).atan2(d.y as f64);
        // convert to degrees
        let angle = angle * 180_f64 * std::f64::consts::FRAC_1_PI + 180_f64;
        (angle * 1000_f64) as i64 % 360_000
    }

    fn diff(&self, p: &Self) -> Self {
        Point {
            x: p.x - self.x,
            y: self.y - p.y,
        }
    }

    fn man(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    fn dist(&self, p: &Self) -> i64 {
        self.diff(p).man()
    }
}

fn parse<R: BufRead>(f: R) -> Vec<Point> {
    f.lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.unwrap()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| Point {
                    x: x as i64,
                    y: y as i64,
                })
                .collect::<Vec<Point>>()
        })
        .collect()
}

fn max(points: &[Point]) -> (&Point, usize) {
    points
        .iter()
        .map(|a| {
            (
                a,
                points
                    .iter()
                    .filter(|x| *x != a)
                    .map(|x| x.angle(a))
                    .unique()
                    .count(),
            )
        })
        .max_by(|(_, c1), (_, c2)| c1.cmp(c2))
        .unwrap()
}

fn angle_and_dist(p: &Point, points: &[Point]) -> Vec<(Point, i64, i64)> {
    points
        .iter()
        .filter(|x| *x != p)
        .map(|x| (*x, p.angle(x), p.dist(x)))
        .collect()
}

fn angle_and_index(ad: &mut [(Point, i64, i64)]) -> Vec<(Point, i64, i64)> {
    // sort by the angle
    ad.sort_by(|(_, a1, _), (_, a2, _)| a1.cmp(a2));

    let mut groups = Vec::new();

    for (_, group) in &ad.iter().group_by(|(_, a, _)| a) {
        let mut sorted = group.collect::<Vec<&(Point, i64, i64)>>();
        sorted.sort_by(|(_, _, d1), (_, _, d2)| d1.cmp(d2));

        let indexed = sorted
            .iter()
            .enumerate()
            .map(|(i, (p, a, _))| (*p, *a, i as i64))
            .collect::<Vec<(_, _, _)>>();
        groups.push(indexed);
    }

    groups.into_iter().flatten().collect()
}

fn find_nth(p: &Point, points: &[Point], n: usize) -> Point {
    let mut x = angle_and_dist(p, &points);
    let mut x = angle_and_index(&mut x);
    x.sort_by(|(_, a1, i1), (_, a2, i2)| i1.cmp(i2).then(a1.cmp(a2)));

    x[n - 1].0
}

pub fn part1() {
    let f = File::open("day10.txt").expect("file not found");
    let f = BufReader::new(f);

    let points = parse(f);
    let max = max(&points);

    println!("max: {:?}", max);
}

pub fn part2() {
    let f = File::open("day10.txt").expect("file not found");
    let f = BufReader::new(f);

    let points = parse(f);
    let m = max(&points);
    let p = find_nth(&m.0, &points, 200);

    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let s = ".#..#
.....
#####
....#
...##";
        let points = parse(s.as_bytes());
        assert_eq!((&Point { x: 3, y: 4 }, 8), max(&points));
    }

    #[test]
    fn test_angle_and_dist() {
        let s = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

        let points = parse(s.as_bytes());
        let p = find_nth(&Point { x: 11, y: 13 }, &points, 200);
        assert_eq!(Point { x: 8, y: 2 }, p);
    }
}
