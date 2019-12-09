use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part1() {
    let f = File::open("day1.txt").unwrap();
    let f = BufReader::new(f);

    let mut sum: i32 = 0;

    for line in f.lines() {
        let l = line.unwrap();
        let mass: i32 = l.parse().unwrap();

        sum += fuel(mass);
    }

    println!("total {}", sum);
}

pub fn part2() {
    let f = File::open("day1.txt").expect("file not found");
    let f = BufReader::new(f);

    let mut sum: i32 = 0;

    for line in f.lines() {
        let l = line.unwrap();
        let mass: i32 = l.parse().unwrap();

        sum += recursive_fuel(mass);
    }

    println!("total {}", sum);
}

fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn recursive_fuel(mass: i32) -> i32 {
    let mut m = fuel(mass);
    let mut total: i32 = 0;

    while m > 0 {
        total += m;
        m = fuel(m);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel() {
        assert_eq!(2, fuel(12));
        assert_eq!(2, fuel(14));
        assert_eq!(654, fuel(1969));
        assert_eq!(33583, fuel(100_756));
    }

    #[test]
    fn test_recursive_fuel() {
        assert_eq!(2, recursive_fuel(12));
        assert_eq!(2, recursive_fuel(14));
        assert_eq!(966, recursive_fuel(1969));
        assert_eq!(50346, recursive_fuel(100_756));
    }
}
