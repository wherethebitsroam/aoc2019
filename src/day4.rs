pub fn part1() {
    let mut count = 0;
    for x in 278_384..824_795 {
        let digits = split(x);
        if conseq(&digits) && incr(&digits) {
            // println!("x: {}", x);
            count += 1;
        }
    }
    println!("count: {}", count);
}

fn conseq_only2(v: &[i32]) -> bool {
    let mut last = v[0];
    let mut mcount: i32 = 0;

    for x in &v[1..] {
        if *x == last {
            mcount += 1;
        } else {
            // check if the previous was a double
            if mcount == 1 {
                return true;
            }
            mcount = 0;
        }
        last = *x;
    }

    mcount == 1
}

fn conseq(v: &[i32]) -> bool {
    let mut last = v[0];
    for x in &v[1..] {
        if *x == last {
            return true;
        }
        last = *x;
    }

    false
}

fn incr(v: &[i32]) -> bool {
    let mut last = v[0];
    for x in &v[1..] {
        if *x < last {
            return false;
        }
        last = *x;
    }

    true
}

fn split(x: i32) -> Vec<i32> {
    let mut v: Vec<i32> = Vec::new();
    let mut x = x;
    for i in (0..6).rev() {
        let pow = (10 as i32).pow(i);
        let a = x / pow;
        x -= a * pow;
        v.push(a);
    }

    v
}

pub fn part2() {
    let mut count = 0;
    for x in 278_384..824_795 {
        let digits = split(x);
        if conseq_only2(&digits) && incr(&digits) {
            // println!("x: {}", x);
            count += 1;
        }
    }
    println!("count: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(true, conseq(&[0, 0]));
        assert_eq!(true, conseq(&[1, 0, 6, 6, 7]));
        assert_eq!(true, incr(&[0, 1, 6, 6, 7]));
        assert_eq!(false, incr(&[0, 1, 6, 4, 7]));
        assert_eq!(false, conseq_only2(&[0, 1, 1, 1, 7]));
        assert_eq!(false, conseq_only2(&[1, 1, 1, 1, 7]));
        assert_eq!(true, conseq_only2(&[1, 1, 1, 7, 7]));
        assert_eq!(true, conseq_only2(&[0, 1, 1, 6, 7]));
        assert_eq!(true, conseq_only2(&[1, 1, 2, 2, 3, 3]));
        assert_eq!(true, conseq_only2(&[1, 1, 2, 2, 2, 2]));
    }

    #[test]
    fn test_part2() {}
}
