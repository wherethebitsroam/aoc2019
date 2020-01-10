use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn part1() {
    let f = File::open("day2.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut v = string_to_vec(&data);

    // initialise
    v[1] = 12;
    v[2] = 2;

    // println!("input: {:?}", v);

    computer(&mut v);

    println!("result: {}", v[0]);
}

fn calculate(s: &str) -> String {
    let mut v = string_to_vec(s);
    computer(&mut v);
    vec_to_string(&v)
}

fn string_to_vec(s: &str) -> Vec<i32> {
    s.trim()
        .split(',')
        .map(|x| x.parse().expect("failed to parse to int"))
        .collect()
}

fn vec_to_string(v: &[i32]) -> String {
    let s: Vec<String> = v.iter().map(|x| x.to_string()).collect();
    s.join(",")
}

fn computer(v: &mut Vec<i32>) {
    let mut i = 0;
    loop {
        if v[i] == 99 {
            break;
        }
        let a = v[i + 1] as usize;
        let b = v[i + 2] as usize;
        let r = v[i + 3] as usize;
        v[r] = match v[i] {
            1 => v[a] + v[b],
            2 => v[a] * v[b],
            x => panic!("unknown opcode {}", x),
        };
        // println!("i={}: {:?}", i, v);
        i += 4;
    }
}

pub fn part2() {
    let f = File::open("day2.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    for noun in 0..100 {
        for verb in 0..100 {
            let mut v = string_to_vec(&data);
            v[1] = noun;
            v[2] = verb;
            computer(&mut v);

            // println!("trying: noun: {}, verb: {}: result: {}", noun, verb, v[0]);

            if v[0] == 19_690_720 {
                println!("noun: {}, verb: {}", noun, verb);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate() {
        assert_eq!("2,0,0,0,99", calculate("1,0,0,0,99"));
        assert_eq!("2,3,0,6,99", calculate("2,3,0,3,99"));
        assert_eq!("2,4,4,5,99,9801", calculate("2,4,4,5,99,0"));
        assert_eq!("30,1,1,4,2,5,6,0,99", calculate("1,1,1,4,99,5,6,0,99"));
    }
}
