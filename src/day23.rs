use crate::intcode::ExitMode;
use crate::intcode::Intcode;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Nic {
    code: Intcode,
    rx: Receiver<i64>,
    tx: Sender<Vec<i64>>,
    id: i64,
}

impl Nic {
    fn new(code: &Intcode, id: i64, rx: Receiver<i64>, tx: Sender<Vec<i64>>) -> Self {
        Self {
            code: code.clone(),
            id,
            rx,
            tx,
        }
    }

    fn get_input(&self) -> i64 {
        match self.rx.try_recv() {
            Ok(x) => {
                println!("nic {}: input: {}", self.id, x);
                x
            }
            Err(_) => -1,
        }
    }

    fn run(&mut self) {
        let mut code = self.code.clone();
        let mut v = Vec::new();

        println!("nic {}: starting", self.id);

        loop {
            match code.run(&|| self.get_input()) {
                ExitMode::Halt => {
                    println!("halt");
                    break;
                }
                ExitMode::Output(x) => {
                    v.push(x);
                    if v.len() == 3 {
                        println!("nic {}: sending {:?}", self.id, v);
                        self.tx.send(v).unwrap();
                        v = Vec::new();
                    }
                }
            };
        }
    }
}

pub fn part1() {
    let f = File::open("day23.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let code = Intcode::new(&data);

    let (router_tx, router_rx) = mpsc::channel();

    let mut nic_tx = Vec::new();
    let mut handles = vec![];

    for i in 0..50 {
        let (tx, rx) = mpsc::channel();
        // send id
        tx.send(i).unwrap();
        let mut nic = Nic::new(&code, i, rx, mpsc::Sender::clone(&router_tx));
        nic_tx.push(tx);

        // start a thread for the nic
        let handle = thread::spawn(move || {
            nic.run();
        });
        handles.push(handle);
    }

    // router
    let handle = thread::spawn(move || loop {
        let p = router_rx.recv().unwrap();
        println!("router: received: {:?}", p);
        if p.len() != 3 {
            panic!("bad packet, len: {}", p.len());
        }
        if p[0] == 255 {
            println!("Packet to 255: x: {}, y: {}", p[1], p[2]);
            break;
        }
        let tx = &nic_tx[p[0] as usize];
        tx.send(p[1]).unwrap();
        tx.send(p[2]).unwrap();
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn part2() {
    let f = File::open("day23.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let code = Intcode::new(&data);

    let (router_tx, router_rx) = mpsc::channel();

    let mut nic_tx = Vec::new();
    let mut handles = vec![];

    for i in 0..50 {
        let (tx, rx) = mpsc::channel();
        // send id
        tx.send(i).unwrap();
        let mut nic = Nic::new(&code, i, rx, mpsc::Sender::clone(&router_tx));
        nic_tx.push(tx);

        // start a thread for the nic
        let handle = thread::spawn(move || {
            nic.run();
        });
        handles.push(handle);
    }

    // router
    let handle = thread::spawn(move || {
        let mut nat: Option<Vec<i64>> = None;
        let mut lasty: Option<i64> = None;
        loop {
            match router_rx.recv_timeout(Duration::from_millis(1000)) {
                Ok(p) => {
                    println!("router: received: {:?}", p);
                    if p.len() != 3 {
                        panic!("bad packet, len: {}", p.len());
                    }
                    if p[0] == 255 {
                        nat = Some(p);
                    } else {
                        let tx = &nic_tx[p[0] as usize];
                        tx.send(p[1]).unwrap();
                        tx.send(p[2]).unwrap();
                    }
                }
                Err(_) => {
                    // assume this is timeout
                    println!("timeout");
                    match &nat {
                        None => panic!("timeout with empty nat"),
                        Some(n) => {
                            // check againt last sent
                            if let Some(lasty) = lasty {
                                if lasty == n[2] {
                                    println!("Sending {} twice", lasty);
                                    break;
                                }
                            }
                            let tx = &nic_tx[0];
                            tx.send(n[1]).unwrap();
                            tx.send(n[2]).unwrap();
                            lasty = Some(n[2]);
                        }
                    }
                }
            }
        }
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}
