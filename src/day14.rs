use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Agent {
    mul: i64,
    chem: String,
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Reaction {
    outputs: i64,
    inputs: Vec<Agent>,
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Equations {
    reactions: HashMap<String, Reaction>,
    refs: HashMap<String, Vec<String>>,
}

impl Equations {
    fn parse<R: BufRead>(f: R) -> Self {
        let mut reactions = HashMap::new();
        let mut refs: HashMap<String, Vec<String>> = HashMap::new();

        for line in f.lines() {
            let l = line.unwrap();
            let blah: Vec<&str> = l.split(" => ").collect();
            if blah.len() != 2 {
                panic!("not 2");
            }
            let inputs: Vec<Agent> = blah[0]
                .split(", ")
                .map(|x| {
                    let y: Vec<&str> = x.split(' ').collect();
                    if y.len() != 2 {
                        panic!("not 2");
                    }
                    Agent {
                        mul: y[0].parse().unwrap(),
                        chem: y[1].to_string(),
                    }
                })
                .collect();

            let y: Vec<&str> = blah[1].split(' ').collect();

            for input in inputs.iter() {
                refs.entry(input.chem.to_string())
                    .or_insert_with(|| Vec::new())
                    .push(y[1].to_string());
            }

            reactions.insert(
                y[1].to_string(),
                Reaction {
                    inputs,
                    outputs: y[0].parse().unwrap(),
                },
            );
        }

        Self { reactions, refs }
    }

    fn clear(&mut self, chem: &str) {
        // remove from refs
        for v in self.refs.values_mut() {
            v.retain(|x| *x != chem);
        }
        self.refs.remove(chem);
    }

    fn reaction(&self, chem: &str) -> &Reaction {
        // return the reaction
        match self.reactions.get(chem) {
            Some(x) => x,
            None => panic!("didn't find chem {}", chem),
        }
    }

    fn unrefed(&self) -> Option<String> {
        self.refs
            .iter()
            .find(|(_, v)| v.is_empty())
            .map(|(k, _)| k.to_string())
    }

    fn react(&self, agents: &[Agent], chem: &str) -> Vec<Agent> {
        let reaction = self.reaction(chem);
        let mut new_agents = Vec::new();

        for agent in agents.iter() {
            if agent.chem == chem {
                let mut mul = agent.mul / reaction.outputs;
                if mul * reaction.outputs < agent.mul {
                    mul += 1;
                }
                for ag in reaction.inputs.iter() {
                    new_agents.push(Agent {
                        chem: ag.chem.to_string(),
                        mul: mul * ag.mul,
                    });
                }
            } else {
                new_agents.push(Agent {
                    chem: agent.chem.to_string(),
                    mul: agent.mul,
                });
            }
        }
        // collect similar terms
        let mut m: HashMap<String, Agent> = HashMap::new();
        for ag in new_agents.iter() {
            let e = m.entry(ag.chem.to_string()).or_insert(Agent {
                chem: ag.chem.to_string(),
                mul: 0,
            });
            e.mul += ag.mul;
        }

        // wtf
        m.values().cloned().collect()
    }

    fn run(&mut self, chem: &str, count: i64) -> i64 {
        let mut inputs = self.reaction(chem).inputs.clone();
        for x in inputs.iter_mut() {
            x.mul *= count;
        }
        self.clear(chem);
        loop {
            match self.unrefed() {
                None => break,
                Some(u) => {
                    if u == "ORE" {
                        break;
                    }
                    inputs = self.react(&inputs, &u);
                    self.clear(&u);
                }
            }
        }
        inputs[0].mul
    }
}

pub fn part1() {
    let f = File::open("day14.txt").expect("file not found");
    let mut f = BufReader::new(f);

    let mut m = Equations::parse(f);
    println!("{}", m.run("FUEL", 1));
}

pub fn part2() {
    let f = File::open("day14.txt").expect("file not found");
    let mut f = BufReader::new(f);
    let mut data = String::new();
    f.read_to_string(&mut data).expect("failed to read string");

    let mut i = 2390225;
    loop {
        let mut m = Equations::parse(data.as_bytes());
        let ore = m.run("FUEL", i);
        println!("{} => {}", i, ore);
        if ore > 1000000000000 {
            break;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {}

    #[test]
    fn test_1_1() {
        let s = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let mut m = Equations::parse(s.as_bytes());
        assert_eq!(165, m.run("FUEL", 1));
    }

    #[test]
    fn test_1_2() {
        let s = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        let mut m = Equations::parse(s.as_bytes());
        assert_eq!(165, m.run("FUEL", 1));
    }
}
