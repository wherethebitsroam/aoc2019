use std::collections::HashMap;

// grid is an i32
// bits:
//  0  1  2  3  4
//  5  6  7  8  9
// 10 11 12 13 14
// 15 16 17 18 19
// 20 21 22 23 24

fn parse(s: &str) -> i32 {
    let mut v = 0;
    for (i, c) in s.chars().enumerate() {
        let b = match c {
            '.' => 0,
            '#' => 1,
            _ => panic!("bad char"),
        };
        v |= b << i;
    }
    v
}

pub fn part1() {
    // real
    // let grid = "...#.#.##.#..###.#####...";
    let grid = "....##..#.#..##..#..#....";

    let mut v = parse(&grid);

    let mut m = HashMap::new();
    m.insert(v, 0);

    loop {
        v = minute(v);
        match m.get(&v) {
            None => m.insert(v, 0),
            Some(_) => {
                println!("double: {}", v);
                break;
            }
        };
    }
}

fn minute(x: i32) -> i32 {
    let mut v = 0;
    for i in 0..25 {
        let mut adj = vec![];
        // exclude left edge
        if i % 5 != 0 {
            adj.push(i - 1);
        }
        // exclude right edge
        if i % 5 != 4 {
            adj.push(i + 1);
        }
        // exclude top
        if i / 5 != 0 {
            adj.push(i - 5);
        }
        // exclude bottom
        if i / 5 != 4 {
            adj.push(i + 5);
        }
        let adj_bugs = adj.iter().filter(|a| (x & (1 << **a)) > 0).count();
        let set = (1 << i & x) > 0;
        if adj_bugs == 1 || (!set && adj_bugs == 2) {
            v |= 1 << i;
        }
    }
    v
}

struct InfiniteBoard {
    boards: Vec<i32>,
    offset: i32,
}

impl InfiniteBoard {
    fn new(initial: i32) -> Self {
        Self {
            boards: vec![initial],
            offset: 0,
        }
    }

    fn next(&self) -> Self {
        let boards = vec![0; self.boards.len() + 2];
        let offset = self.offset + 1;
        Self { boards, offset }
    }

    fn is_set(&self, level: i32, index: i32) -> bool {
        let board = level + self.offset;
        if index >= 25 {
            panic!("bad index");
        }
        if board < 0 {
            return false;
        }
        // we know we are positive, so convert to usize
        let board = board as usize;
        if board >= self.boards.len() {
            return false;
        }
        (self.boards[board] & (1 << index)) > 0
    }

    fn neighbours(index: i32) -> Vec<(i32, i32)> {
        match index {
            // first row
            0 => vec![(-1, 11), (-1, 7), (0, 1), (0, 5)],
            1 => vec![(-1, 7), (0, 0), (0, 2), (0, 6)],
            2 => vec![(-1, 7), (0, 1), (0, 3), (0, 7)],
            3 => vec![(-1, 7), (0, 2), (0, 4), (0, 8)],
            4 => vec![(-1, 7), (-1, 13), (0, 3), (0, 9)],
            // second row
            5 => vec![(-1, 11), (0, 0), (0, 6), (0, 10)],
            6 => vec![(0, 1), (0, 7), (0, 11), (0, 5)],
            7 => vec![
                (0, 2),
                (0, 6),
                (0, 8),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4),
            ],
            // grid is an i32
            // bits:
            //  0  1  2  3  4
            //  5  6  7  8  9
            // 10 11 12 13 14
            // 15 16 17 18 19
            // 20 21 22 23 24
            8 => vec![(0, 3), (0, 7), (0, 9), (0, 13)],
            9 => vec![(-1, 13), (0, 4), (0, 8), (0, 14)],
            // third row
            10 => vec![(-1, 11), (0, 5), (0, 11), (0, 15)],
            11 => vec![
                (0, 6),
                (0, 10),
                (0, 16),
                (1, 0),
                (1, 5),
                (1, 10),
                (1, 15),
                (1, 20),
            ],
            // 12 => is the next layer...
            13 => vec![
                (0, 8),
                (0, 14),
                (0, 18),
                (1, 4),
                (1, 9),
                (1, 14),
                (1, 19),
                (1, 24),
            ],
            14 => vec![(-1, 13), (0, 9), (0, 13), (0, 19)],
            // forth row
            15 => vec![(-1, 11), (0, 10), (0, 16), (0, 20)],
            16 => vec![(0, 11), (0, 15), (0, 21), (0, 17)],
            17 => vec![
                (0, 16),
                (0, 18),
                (0, 22),
                (1, 20),
                (1, 21),
                (1, 22),
                (1, 23),
                (1, 24),
            ],
            18 => vec![(0, 17), (0, 19), (0, 13), (0, 23)],
            19 => vec![(-1, 13), (0, 14), (0, 18), (0, 24)],
            // fifth row
            20 => vec![(-1, 17), (-1, 11), (0, 15), (0, 21)],
            21 => vec![(-1, 17), (0, 20), (0, 22), (0, 16)],
            22 => vec![(-1, 17), (0, 21), (0, 23), (0, 17)],
            23 => vec![(-1, 17), (0, 22), (0, 24), (0, 18)],
            24 => vec![(-1, 17), (-1, 13), (0, 23), (0, 19)],
            // too high (or 12)
            _ => panic!("bad index"),
        }
    }

    fn minute(&self) -> Self {
        let mut next = self.next();

        for (board, b) in next.boards.iter_mut().enumerate() {
            let level = board as i32 - next.offset;
            for i in 0..25 {
                if i == 12 {
                    continue;
                }
                let n = Self::neighbours(i)
                    .iter()
                    .filter(|&(dl, idx)| self.is_set(level + dl, *idx))
                    .count();
                // println!("level: {}, index: {}, neighbours: {}", level, i, n);
                if n == 1 || (!self.is_set(level, i) && n == 2) {
                    *b |= 1 << i;
                }
            }
        }

        next
    }

    fn print(&self) {
        for (x, _) in self.boards.iter().enumerate() {
            let lvl = x as i32 - self.offset;
            println!("Depth: {}", lvl);
            for i in 0..5 {
                for j in 0..5 {
                    let index = i * 5 + j;
                    let c = if self.is_set(lvl, index) { '#' } else { '.' };
                    print!("{}", c);
                }
                println!();
            }
            println!();
        }
    }

    fn count_bits(x: i32) -> i32 {
        let mut count = 0;
        for i in 0..25 {
            if x & 1 << i > 0 {
                count += 1;
            }
        }
        count
    }

    fn count(&self) -> i32 {
        self.boards.iter().map(|&b| Self::count_bits(b)).sum()
    }
}

pub fn part2() {
    // let grid = "....##..#.#..##..#..#....";
    let grid = "...#.#.##.#..###.#####...";
    let b = parse(&grid);

    let mut ib = InfiniteBoard::new(b);
    // ib.print();

    for _ in 0..200 {
        ib = ib.minute();
    }
    // ib.print();

    println!("sum: {}", ib.count());
}
