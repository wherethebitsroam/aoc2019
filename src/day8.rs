use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub fn part1() {
    let f = File::open("day8.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let layer_size = 25 * 6;
    let layers = data.trim().len() / layer_size;

    let mut blah = Vec::new();

    for i in 0..layers {
        let sums = data[i * layer_size..(i + 1) * layer_size].chars().fold(
            HashMap::new(),
            |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            },
        );
        blah.push(sums);
    }

    let min = blah
        .iter()
        .min_by(|a, b| a.get(&'0').unwrap().cmp(b.get(&'0').unwrap()));
    let result = min.unwrap().get(&'1').unwrap() * min.unwrap().get(&'2').unwrap();

    println!("min: {:?}", min);
    println!("result: {:?}", result);
}

pub fn part2() {
    let f = File::open("day8.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let layer_size = 25 * 6;
    let mut layers: Vec<Vec<char>> = Vec::new();

    for i in 0..100 {
        layers.push(data[i * layer_size..(i + 1) * layer_size].chars().collect());
    }

    for row in 0..6 {
        for col in 0..25 {
            let x = row * 25 + col;
            for layer in layers.iter() {
                match layer[x] {
                    '0' => {
                        print!(" ");
                        break;
                    }
                    '1' => {
                        print!("#");
                        break;
                    }
                    _ => (),
                }
            }
        }
        println!();
    }
}
