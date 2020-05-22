use regex::Regex;
use std::collections::HashMap;

fn parse(input: &str) -> Task {
    let mut chunkers = input.split("\n\n").collect::<Vec<&str>>();

    
    let re_startstate = Regex::new(r"Begin in state ([A-Z])").unwrap();
    let startstate = re_startstate.captures(&chunkers[0]).unwrap().get(1).unwrap().as_str();
   
    let re_checksum = Regex::new(r"Perform a diagnostic checksum after (\d+) steps.").unwrap();
    let steps_to_perform: u32 = re_checksum.captures(&chunkers[0]).unwrap().get(1).unwrap().as_str().parse().unwrap(); // Joke present
   
    chunkers.remove(0);
    let transitions = chunkers.iter().map(|chunk| Transition::from_string(chunk)).collect::<Vec<Transition>>();

    Task {
        machine: Masheen::new(transitions, String::from(startstate)),
        steps_to_run: steps_to_perform
    }
}

#[aoc(day25, part1)]
fn day25_part1(input: &str) -> u32 {
    let mut task = parse(input);

    for _i in 0..task.steps_to_run {
        task.machine.step();
    }

    task.machine.tape.iter().map(|x| *x as u32).sum()
}

#[derive(Debug)]
enum Direction {
    Left,
    Right
}


#[derive(Debug)]
struct Instruction {
    to_write: u8,
    dir: Direction,
    next_state: String
}

impl Instruction {
    fn from_string(input: &str) -> Instruction {
        let re_towrite = Regex::new(r"Write the value 1").unwrap();
        let re_dir = Regex::new(r"to the left").unwrap();
        let re_next = Regex::new(r"with state ([A-Z])").unwrap();
        Instruction {
            to_write: match re_towrite.is_match(input) {
                true => 1,
                false => 0
            },
            dir: match re_dir.is_match(input) {
                true => Direction::Left,
                false => Direction::Right
            },
            next_state: String::from(re_next.captures(input).unwrap().get(1).unwrap().as_str())
        }
    }
}

#[derive(Debug)]
struct Transition {
    id: String,
    instructions: Vec<Instruction>
}

impl Transition {
    fn from_string(input: &str) -> Transition {
        let re_id = Regex::new(r"In state ([A-Z])").unwrap();
        let id = re_id.captures(input).unwrap().get(1).unwrap().as_str();

        let instr = input.split("If the current").collect::<Vec<&str>>(); // Feeling a bit like a butcher here

        Transition {
            id: String::from(id),
            instructions: vec!(
                Instruction::from_string(instr[1]),
                Instruction::from_string(instr[2])
            )
        }
    }
}

#[derive(Debug)]
struct Masheen {
    tape: Vec<u8>,
    state: String,
    ptr: i32,
    offset: i32,
    transitions: HashMap<String, Transition>
}

impl Masheen {
    fn new(transitions: Vec<Transition>, state: String) -> Masheen {
        let mut trns = HashMap::new();
        // A+ for naming
        for tr in transitions {
            trns.insert(tr.id.clone(), tr);
        }

        Masheen {
            tape: vec!(0),
            state: state,
            ptr: 0,
            offset: 0,
            transitions: trns
        }
    }

    fn step(&mut self) {
        let pos = (self.ptr + self.offset) as usize;
        let input = self.tape[pos];
        let transitions = self.transitions.get(&self.state).unwrap();
        let instruction = &transitions.instructions[input as usize];

        self.tape[pos] = instruction.to_write;
        self.state = instruction.next_state.clone();
        match instruction.dir {
            Direction::Left => self.ptr -= 1,
            Direction::Right => self.ptr += 1
        }

        if self.ptr + self.offset < 0 {
            let mut grwth = vec![0; self.tape.len()];
            self.offset += self.tape.len() as i32;
            grwth.append(&mut self.tape);
            self.tape = grwth;
        } else if pos + 1 >= self.tape.len() {
            let mut grwth = vec![0; self.tape.len()];
            self.tape.append(&mut grwth);
        }
    }
}

impl std::fmt::Display for Masheen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pos = (self.ptr + self.offset) as usize;
        let oot = self.tape.iter().enumerate().map(|(i, x)| {
            let sym = if *x == 0 { String::from("0") } else { String::from("1") };
            if i == pos {
                format!("[{}]", sym)
            } else {
                format!(" {} ", sym)
            }
        })
        .collect::<Vec<String>>()
        .join("");
        write!(f, "{}", oot)
    }
}

#[derive(Debug)]
struct Task {
    machine: Masheen,
    steps_to_run: u32
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn day25_example() {
        let input = &"Begin in state A.
        Perform a diagnostic checksum after 6 steps.

    In state A:
        If the current value is 0:
            - Write the value 1.
            - Move one slot to the right.
            - Continue with state B.
        If the current value is 1:
            - Write the value 0.
            - Move one slot to the left.
            - Continue with state B.

    In state B:
        If the current value is 0:
            - Write the value 1.
            - Move one slot to the left.
            - Continue with state A.
        If the current value is 1:
            - Write the value 1.
            - Move one slot to the right.
            - Continue with state A.
";

        assert_eq!(day25_part1(input), 3);
    }
}