use std::cmp;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

struct Matrix {
    m: Vec<Vec<i32>>
}

fn mul(a: &[i32], b: &[i32]) -> Vec<i32> {
    if a.len() != b.len() {
        panic!("missmatched lengths");
    }
    let mut x = vec![0; a.len()];
    for i in 0..a.len() {
        x[i] = a[i] * b[i];
    }
    x
}

fn sum(x: &[i32]) -> i32 {
    x.iter().sum::<i32>()
}

fn sum_abs_mod(x: &[i32]) -> i32 {
    x.iter().sum::<i32>().abs() % 10
}

fn pattern_matrix(size: usize) -> Vec<Vec<i32>> {
    let mut v: Vec<Vec<i32>> = Vec::new();
    for r in 0..size {
        v.push(pattern(r + 1, size));
    }
    v
}

fn invert(m: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let rows = m.len();
    if rows == 0 {
        panic!("empty matrix");
    }
    let cols = m[0].len();
    if cols == 0 {
        panic!("empty row");
    }

    let mut v: Vec<Vec<i32>> = Vec::new();
    for c in 0..cols {
        let mut row: Vec<i32> = Vec::new();
        for r in 0..rows {
            row.push(m[r][c]);
        }
        v.push(row);
    }
    v
}

fn matrix_mul(m1: &[Vec<i32>], m2: &[Vec<i32>], sum: fn(&[i32]) -> i32) -> Vec<Vec<i32>> {
    // should check sizes
    let mut v = vec![vec![0; m2.len()]; m1.len()];
    for i in 0..m1.len() {
        for j in 0..m2.len() {
            let mul = mul(&m1[i], &m2[j]);
            let sum = sum(&mul);
            v[i][j] = sum;
        }
    }
    v
}

fn matrix_scalar_mul(m: &[Vec<i32>], s: &[i32], sum: fn(&[i32]) -> i32) -> Vec<i32> {
    let mut v = vec![0; s.len()];
    for i in 0..m.len() {
        if m[i].len() != s.len() {
            panic!(
                "size mismatch row {} of matrix size {}, scalar size {}",
                i,
                m[i].len(),
                s.len()
            );
        }
        v[i] = sum(&mul(&m[i], &s));
    }
    v
}

fn pattern(round: usize, size: usize) -> Vec<i32> {
    let base = vec![0, 1, 0, -1];
    let mut patt = vec![0; size + 1];
    for i in 0..=(size + 1 / round) {
        let x = base[i % 4];
        for j in 0..round {
            let index = i * round + j;
            if index >= patt.len() {
                break;
            }
            patt[index] = x;
        }
    }
    patt[1..].to_vec()
}

fn phases(v: &[i32], n: usize) -> Vec<i32> {
    let pat = pattern_matrix(v.len());
    let mut v = v.to_vec();
    for i in 0..n {
        println!("{}", i);
        v = matrix_scalar_mul(&pat, &v, sum_abs_mod);
    }
    v
}

fn phases_sum(v: &[i32], n: usize, sum: fn(&[i32]) -> i32) -> Vec<i32> {
    let pat = pattern_matrix(v.len());
    let mut v = v.to_vec();
    for i in 0..n {
        println!("{}", i);
        v = matrix_scalar_mul(&pat, &v, sum);
    }
    v
}

pub fn part1() {
    let f = File::open("day16.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let input: Vec<i32> = data
        .trim()
        .chars()
        .map(|x| x.to_string().parse().unwrap())
        .collect();

    let output = phases(&input, 100);

    println!("{:?}", &output[..8]);
}

pub fn part2() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul() {
        assert_eq!(
            4,
            sum_abs_mod(&mul(&[1, 2, 3, 4, 5, 6, 7, 8], &[1, 0, -1, 0, 1, 0, -1, 0]))
        )
    }

    #[test]
    fn test_pattern() {
        assert_eq!(vec![1, 0, -1, 0, 1, 0, -1, 0], pattern(1, 8));
        assert_eq!(vec![0, 1, 1, 0, 0, -1, -1, 0], pattern(2, 8));
    }

    #[test]
    fn test_phases() {
        let x = phases(
            &[
                8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4,
                5, 5, 9, 5,
            ],
            100,
        );
        assert_eq!(&[2, 4, 1, 7, 6, 1, 7, 6], &x[..8],)
    }

    #[test]
    fn test_phases_sum() {
        let x = phases_sum(&[1, 2, 3, 4, 5, 6, 7, 8], 4, sum);
        let x: Vec<i32> = x.iter().map(|x| x.abs() % 10).collect();
        assert_eq!(&[0, 1, 0, 2, 9, 4, 9, 8], &x[..8],)
    }

    #[test]
    fn test_phases2() {
        let x = phases(&[1, 2, 3, 4, 5, 6, 7, 8], 1);
        println!("{:?}", x);
        let x = phases(&[1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8], 1);
        println!("{:?}", x);
        let x = phases(&[1, 2, 3, 4, 5, 6, 7, 8], 2);
        println!("{:?}", x);
        let x = phases(&[1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8], 2);
        println!("{:?}", x);
    }
}
