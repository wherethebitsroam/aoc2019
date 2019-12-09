use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn input_to_map(s: &str) -> HashMap<String, Vec<String>> {
    let mut m: HashMap<String, Vec<String>> = HashMap::new();
    for l in s.trim().split_whitespace() {
        let vals: Vec<&str> = l.split(')').collect();
        m.entry(vals[0].to_owned())
            .or_insert(Vec::new())
            .push(vals[1].to_owned());
    }
    m
}

fn input_to_rev_map(s: &str) -> HashMap<String, String> {
    let mut m: HashMap<String, String> = HashMap::new();
    for l in s.trim().split_whitespace() {
        let vals: Vec<&str> = l.split(')').collect();
        m.insert(vals[1].to_owned(), vals[0].to_owned());
    }
    m
}

fn start(m: &HashMap<String, Vec<String>>) -> i32 {
    orbits(m, "COM", 0)
}

fn orbits(m: &HashMap<String, Vec<String>>, p: &str, count: i32) -> i32 {
    count
        + match m.get(p) {
            Some(v) => v.iter().fold(0, |acc, x| acc + orbits(m, x, count + 1)),
            None => 0,
        }
}

fn path(m: &HashMap<String, String>, x: &str) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    let mut p = x;

    // while let Some(p2) = m.get(p) { .. }

    while let Some(p2) = m.get(p) {
        v.push(p2.to_owned());
        p = p2;
    }

    v
}

pub fn part1() {
    let f = File::open("day6.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let m = input_to_map(&data);
    let sum = start(&m);

    println!("sum: {}", sum);
}

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

fn common(v1: &[String], v2: &[String]) -> usize {
    let len = min(v1.len(), v2.len());
    let mut x = 0;

    for i in 0..len {
        if v1[i] == v2[i] {
            x += 1;
        } else {
            break;
        }
    }
    x
}

fn hops(s: &str) -> usize {
    let m = input_to_rev_map(&s);

    let mut p1 = path(&m, "YOU");
    let mut p2 = path(&m, "SAN");
    p1.reverse();
    p2.reverse();

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);

    let c = common(&p1, &p2);

    println!("common: {}", c);

    let hops = p1.len() + p2.len() - 2 * c;

    println!("hops: {}", hops);

    hops
}

pub fn part2() {
    let f = File::open("day6.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    println!("hops: {}", hops(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";
        let m = input_to_map(input);
        assert_eq!(42, start(&m));
    }
    #[test]
    fn test_part2() {
        let input = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN";
        assert_eq!(4, hops(&input));
    }
}
