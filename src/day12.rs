use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct PosVel {
    p: i64,
    v: i64,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
struct Moon {
    x: PosVel,
    y: PosVel,
    z: PosVel,
}

impl Moon {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self {
            x: PosVel { p: x, v: 0 },
            y: PosVel { p: y, v: 0 },
            z: PosVel { p: z, v: 0 },
        }
    }
}

fn state(p: &mut [&mut PosVel]) -> (PosVel, PosVel, PosVel, PosVel) {
    (*p[0], *p[1], *p[2], *p[3])
}

fn find_loop(p: &mut [&mut PosVel]) -> (usize, usize) {
    let mut i = 0;
    let mut map: HashMap<(PosVel, PosVel, PosVel, PosVel), usize> = HashMap::new();

    map.insert(state(p), i);

    loop {
        i += 1;
        do_dim_step(p);
        let s = state(p);
        match map.get(&s) {
            None => map.insert(s, i),
            Some(x) => return (*x, i),
        };
    }
}

fn do_dim_step(posvels: &mut [&mut PosVel]) {
    for i in 0..posvels.len() {
        for j in i + 1..posvels.len() {
            match posvels[i].p.cmp(&posvels[j].p) {
                Ordering::Greater => {
                    posvels[i].v -= 1;
                    posvels[j].v += 1;
                }
                Ordering::Less => {
                    posvels[i].v += 1;
                    posvels[j].v -= 1;
                }
                Ordering::Equal => (),
            }
        }
        posvels[i].p += posvels[i].v
    }
}

fn dostep2(moons: &mut [Moon]) {
    let mut x: Vec<&mut PosVel> = Vec::new();
    let mut y: Vec<&mut PosVel> = Vec::new();
    let mut z: Vec<&mut PosVel> = Vec::new();
    for m in moons.iter_mut() {
        x.push(&mut m.x);
        y.push(&mut m.y);
        z.push(&mut m.z);
    }
    do_dim_step(&mut x);
    do_dim_step(&mut y);
    do_dim_step(&mut z);
}

fn dostuff(mut moons: &mut [Moon], steps: usize) {
    for step in 0..steps {
        dostep2(&mut moons);
    }
}

fn energy(moons: &[Moon]) -> i64 {
    let mut total = 0;
    for m in moons.iter() {
        let pot = m.x.p.abs() + m.y.p.abs() + m.z.p.abs();
        let kin = m.x.v.abs() + m.y.v.abs() + m.z.v.abs();
        total += pot * kin;
    }
    total
}

pub fn part1() {
    let mut moons = vec![
        Moon::new(17, -12, 13),
        Moon::new(2, 1, 1),
        Moon::new(-1, -17, 7),
        Moon::new(12, -14, 18),
    ];
    dostuff(&mut moons, 1000);
    let e = energy(&moons);
    println!("energy: {}", e);
}

pub fn part2() {
    let mut moons = vec![
        Moon::new(17, -12, 13),
        Moon::new(2, 1, 1),
        Moon::new(-1, -17, 7),
        Moon::new(12, -14, 18),
    ];
    let mut x: Vec<&mut PosVel> = Vec::new();
    let mut y: Vec<&mut PosVel> = Vec::new();
    let mut z: Vec<&mut PosVel> = Vec::new();
    for m in moons.iter_mut() {
        x.push(&mut m.x);
        y.push(&mut m.y);
        z.push(&mut m.z);
    }
    let l = find_loop(&mut x);
    println!("loop: {:?}", l);
    let l = find_loop(&mut y);
    println!("loop: {:?}", l);
    let l = find_loop(&mut z);
    println!("loop: {:?}", l);

    // loop: (0, 56344)
    // loop: (0, 231614)
    // loop: (0, 193052)

    // 314917503970904
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stuff() {
        let mut moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        dostuff(&mut moons, 10);
        assert_eq!(179, energy(&moons));
    }

    #[test]
    fn test_stuff2() {
        let mut moons = vec![
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        let mut x: Vec<&mut PosVel> = Vec::new();
        let mut y: Vec<&mut PosVel> = Vec::new();
        let mut z: Vec<&mut PosVel> = Vec::new();
        for m in moons.iter_mut() {
            x.push(&mut m.x);
            y.push(&mut m.y);
            z.push(&mut m.z);
        }
        let l = find_loop(&mut x);
        println!("loop: {:?}", l);
        let l = find_loop(&mut y);
        println!("loop: {:?}", l);
        let l = find_loop(&mut z);
        println!("loop: {:?}", l);
    }
}
