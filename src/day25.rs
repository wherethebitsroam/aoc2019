use crate::intcode::ExitMode;
use crate::intcode::Intcode;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;

#[derive(Debug)]
struct Room {
    name: String,
    west: Option<String>,
    east: Option<String>,
    north: Option<String>,
    south: Option<String>,
    item: Option<String>,
}

#[derive(Debug)]
struct Game {
    code: Intcode,
    input: Vec<i64>,
    input_index: usize,
    rooms: HashMap<String, Room>,
    commands: Vec<&'static str>,
    command_index: usize,
}

impl Game {
    fn new(code: &Intcode) -> Self {
        Self {
            code: code.clone(),
            input: vec![],
            input_index: 0,
            rooms: HashMap::new(),
            commands: vec![
                "south",
                "take fixed point",
                "north",
                "north",
                "take spool of cat6",
                "north",
                "take monolith",
                "west",
                "take planetoid",
                "east",
                "north",
                "take hypercube",
                "south",
                "south",
                "east",
                "north",
                "take candy cane",
                "south",
                "east",
                "take easter egg",
                "east",
                "south",
                "take ornament",
                "west",
                "south",
                "west",
            ],
            command_index: 0,
        }
    }

    fn get_input(&mut self) -> i64 {
        if self.input_index >= self.input.len() {
            if self.command_index < self.commands.len() {
                self.input = self.commands[self.command_index]
                    .chars()
                    .map(|x| x as i64)
                    .collect();
                self.input.push('\n' as i64);
                println!("** {} **", self.commands[self.command_index]);
                self.input_index = 0;
                self.command_index += 1;
            } else {
                let mut guess = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
                self.input = guess.chars().map(|x| x as i64).collect();
                self.input_index = 0;
            }
        }
        let i = self.input[self.input_index];
        self.input_index += 1;
        i
    }

    fn run(&mut self) {
        let mut code = self.code.clone();
        loop {
            match code.run(&mut || self.get_input()) {
                ExitMode::Halt => {
                    println!("halt");
                    break;
                }
                ExitMode::Output(x) => {
                    print!("{}", x as u8 as char);
                }
            };
        }
    }

    // fn explore(&self) {
    //     let mut code = self.code.clone();
    //     loop {
    //         match code.run(&mut || self.get_input()) {
    //             ExitMode::Halt => {
    //                 println!("halt");
    //                 break;
    //             }
    //             ExitMode::Output(x) => {
    //                 print!("{}", x as u8 as char);
    //             }
    //         };
    //     }
    // }
}

pub fn part1() {
    let f = File::open("day25.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let code = Intcode::new(&data);

    let mut g = Game::new(&code);
    g.run();
}

pub fn part2() {}
