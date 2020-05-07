use std::collections::HashMap;
use regex::Regex;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

#[derive(Debug, Clone)]
enum Value {
    Register(String),
    Constant(i64)
}

/*
snd X plays a sound with a frequency equal to the value of X.
set X Y sets register X to the value of Y.
add X Y increases register X by the value of Y.
mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
mod X Y sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
rcv X recovers the frequency of the last sound played, but only when the value of X is not zero. (If it is zero, the command does nothing.)
jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
*/
#[derive(Debug, Clone)]
enum Instruction {
    Snd(Value),
    Set{ target: Value, value: Value },
    Add{ target: Value, value: Value },
    Mul{ target: Value, value: Value },
    Mod{ target: Value, value: Value },
    Rcv(Value),
    Jgz{ register: Value, offset: Value },
    None
}

#[derive(Debug)]
struct State {
    registers: HashMap<String, i64>,
    sounded: i64,
    ptr: usize,
    tx: mpsc::Sender<i64>,
    rx: mpsc::Receiver<i64>,
    part1: bool,
    n_sent: i64,
    id: i64
}

impl State {
    fn new(tx: mpsc::Sender<i64>, rx: mpsc::Receiver<i64>, id: i64, part: bool) -> State {
        let mut s = State {
            registers: HashMap::new(),
            sounded: -1,
            ptr: 0,
            n_sent: 0,
            part1: part,
            tx: tx,
            rx: rx,
            id: id
        };

        s.registers.insert(String::from("p"), id);
        s
    }

    fn run_instruction(&mut self, instr: Instruction) -> Result<(), ()> {
        // That's an awful lot of repeated code for 1 character to change...
        match instr {
            Instruction::Snd(target) => {
                self.n_sent += 1;
                match target {
                    Value::Constant(x) => self.sounded = x,
                    Value::Register(r) => self.sounded = *self.registers.entry(r).or_insert(0)
                }

                // regressing back into clone land...
                //println!("sending! id: {}, message: {}", self.id, self.sounded);
                match self.tx.send(self.sounded.clone()) {
                    Ok(_) => {},
                    Err(_e) => return Err(())
                }
            }
            Instruction::Set{target, value} => {
                // Eeeh...
                //println!("Setting! id: {}, target: {:?}, value: {:?}", self.id, target, value);
                if let Value::Register(r) = target {
                    match value {
                        Value::Constant(x) => *self.registers.entry(r).or_insert(0) = x,
                        Value::Register(r2) => {
                            let v = *self.registers.entry(r2).or_insert(0);
                            *self.registers.entry(r).or_insert(0) = v;
                        }
                    }
                }
            },
            Instruction::Add{target, value} => {
                //println!("Adding! id: {}, target: {:?}, value: {:?}", self.id, target, value);
                if let Value::Register(r) = target {
                    match value {
                        Value::Constant(x) => *self.registers.entry(r).or_insert(0) += x,
                        Value::Register(r2) => {
                            let v = *self.registers.entry(r2).or_insert(0);
                            *self.registers.entry(r).or_insert(0) += v;
                        }
                    }
                }
            },
            Instruction::Mul{target, value} => {
                //println!("Muling! id: {}, target: {:?}, value: {:?}", self.id, target, value);
                if let Value::Register(r) = target {
                    match value {
                        Value::Constant(x) => *self.registers.entry(r).or_insert(0) *= x,
                        Value::Register(r2) => {
                            let v = *self.registers.entry(r2).or_insert(0);
                            *self.registers.entry(r).or_insert(0) *= v;
                        }
                    }
                }
            },
            Instruction::Mod{target, value} => {
                //println!("Modding! id: {}, target: {:?}, value: {:?}", self.id, target, value);
                if let Value::Register(r) = target {
                    match value {
                        Value::Constant(x) => *self.registers.entry(r).or_insert(0) %= x,
                        Value::Register(r2) => {
                            let v = *self.registers.entry(r2).or_insert(0);
                            *self.registers.entry(r).or_insert(0) %= v;
                        }
                    }
                }
            },
            Instruction::Rcv(target) => {
                if let Value::Register(r) = target {
                    let t = self.registers.entry(r).or_insert(0);
                    match self.rx.recv_timeout(Duration::from_secs(3)) {
                        Ok(v) => {
                            //println!("Having received! id: {}, message: {}", self.id, v);
                            *t = v;
                        },
                        Err(_e) => {
                            println!("Stopping due to receive error! id: {}", self.id);
                            return Err(());
                        }
                    }
                }
            },
            Instruction::Jgz{register, offset} => {
                //println!("JGZing! id: {}, register: {:?}, offest: {:?}", self.id, register, offset);
                // Thanks @vash3r lol
                // https://www.reddit.com/r/adventofcode/comments/7kj35s/2017_day_18_solutions/
                match register {
                    Value::Register(r) => {
                        let &mut register_value = self.registers.entry(r).or_insert(0);
                        if register_value <= 0 {
                            //println!("also, I'm not going anywheres!")
                        }
                        if register_value > 0 {
                            match offset {
                                Value::Constant(x) => self.ptr += x as usize - 1,
                                Value::Register(r2) => self.ptr += *self.registers.entry(r2).or_insert(0) as usize - 1
                            }
                        }
                    },
                    Value::Constant(x) => {
                        // This code is very WET (we eternally t...uplicate?)
                        if x > 0 {
                            match offset {
                                Value::Constant(x) => self.ptr += x as usize - 1,
                                Value::Register(r2) => self.ptr += *self.registers.entry(r2).or_insert(0) as usize - 1
                            } 
                        }
                    }
                }
            },
            Instruction::None => {}
        }

        self.ptr += 1;
        Ok(())
    }
}

fn get_value(x: &str) -> Value {
    let y = x.trim().parse::<i64>();
    match y {
        Ok(y) => Value::Constant(y),
        Err(_e) => Value::Register(String::from(x))
    }
}

#[aoc_generator(day18)]
fn parse_instructions(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"([a-z]{3}) ([0-9a-z]+)(?: (.+))?").expect("I can write proper regex, ya know...");

    input.lines().map(|l| {
        let caps = re.captures(l).expect("matches");
        let instr = caps.get(1).unwrap().as_str();
        match instr {
            "snd" => Instruction::Snd(get_value(caps.get(2).unwrap().as_str())),
            "set" => Instruction::Set{
                target: get_value(caps.get(2).unwrap().as_str()),
                value: get_value(caps.get(3).unwrap().as_str())
            },
            "add" => Instruction::Add{
                target: get_value(caps.get(2).unwrap().as_str()),
                value: get_value(caps.get(3).unwrap().as_str())
            },
            "mul" => Instruction::Mul{
                target: get_value(caps.get(2).unwrap().as_str()),
                value: get_value(caps.get(3).unwrap().as_str())
            },
            "mod" => Instruction::Mod{
                target: get_value(caps.get(2).unwrap().as_str()),
                value: get_value(caps.get(3).unwrap().as_str())
            },
            "rcv" => Instruction::Rcv(get_value(caps.get(2).unwrap().as_str())),
            "jgz" => Instruction::Jgz{
                register: get_value(caps.get(2).unwrap().as_str()),
                offset: get_value(caps.get(3).unwrap().as_str())
            },
            _ => Instruction::None
        }
    }).collect()
}

#[aoc(day18, part1)]
fn day18_part1(instructions: &Vec<Instruction>) -> i64 {
    println!("{:?}", instructions);

    let (tx, rx) = mpsc::channel();

    let mut s = State::new(tx, rx, 0, true);

    loop {
    //for _k in (0..10) {
        let i = instructions[s.ptr].clone();
        // I've had it!.clone()
        match i.clone() {
            Instruction::Rcv(_x) => {
                let _ = s.run_instruction(i);
                break;
            },
            _ => {
                let _ = s.run_instruction(i);
            }
        }
    }

    s.sounded
}

#[aoc(day18, part2)]
fn day18_part2(instructions: &Vec<Instruction>) -> i64 {
    let (tx_a, rx_a) = mpsc::channel();

    let (tx_b, rx_b) = mpsc::channel();

    let mut a = State::new(tx_a, rx_b, 0, false);
    let mut b = State::new(tx_b, rx_a, 1, false); // Be nice and say hello to each other...

    let instrs_a = instructions.clone();

    let instrs_b = instructions.clone();

    let thingamabob = thread::spawn(move || {
        println!("I am a thread and you are still (somewhat) sane");
        loop {
            if a.ptr >= instrs_a.len() {
                println!("program 0 exiting due to pointer out of range");
                break;
            }
            let i = instrs_a[a.ptr].clone();
            match a.run_instruction(i) {
                Ok(_) => {},
                Err(_) => break
            }
            thread::sleep(Duration::from_millis(1));
        }
        println!("banana for the munkey");
        println!("{}", a.n_sent);
    });

    loop {
        if b.ptr >= instrs_b.len() {
            println!("program 1 exiting due to pointer out of range");
            break;
        }
        let i = instrs_b[b.ptr].clone();
        match b.run_instruction(i) {
            Ok(_) => {},
            Err(_) => break
        }
        thread::sleep(Duration::from_millis(1));
    }

    match thingamabob.join() {
        _ => {}
    }

    b.n_sent
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn input() {
        parse_instructions(&"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2");
    }

    #[test]
    fn day18ex1() {
        assert_eq!(day18_part1(&parse_instructions(&"set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2")), 4);
    }

    #[test]
    fn day18ex2() {
        assert_eq!(day18_part2(&parse_instructions(&"snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d")), 3);
    }
}

/*
Figuring out what it does like a pedestrian

# init
set i 31
set a 1

# If you are program 1, GOTO @ENDFOR
mul p 17
jgz p p

# Set a to 2^31 - 1 (or thereabouts)
mul a 2
add i -1
jgz i -2
add a -1

# 127 times
@BEGINFOR
set i 127
# Set seed
set p 826

# b = (((8505*p % a)*129749 + 12345) % a) % 10000
# i.e. some pseudorandom value
mul p 8505
mod p a
mul p 129749
add p 12345
mod p a
set b p
mod b 10000

# send b
snd b

// decrease i and jgz to @BEGINFOR
add i -1
jgz i -9

@ENDFOR

# if a is zero, receive until you get zero
jgz a 3
rcv b
jgz b -1

# else 
# init
set f 0
set i 126

# get 2 values
rcv a
@BLA
rcv b

# if b > a GOTO @CASE1 else GOTO @CASE2
set p a
mul p -1
add p b
jgz p 4

# Send a (i.e. the larger value)
snd a
# keep the smaller value
set a b
jgz 1 3

@CASE1
snd b
# Sorting not done (at least one larger value received after a smaller one)
set f 1

# if not done looping, go to @BLA (receive a new b)
add i -1
jgz i -11
snd a
jgz f -16
# go lock up?
jgz a -19
*/