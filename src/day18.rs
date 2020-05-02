use std::collections::HashMap;
use regex::Regex;

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

#[derive(Debug, Clone)]
struct State {
    registers: HashMap<String, i64>,
    sounded: i64,
    ptr: usize
}

impl State {
    fn new() -> State {
        State {
            registers: HashMap::new(),
            sounded: -1,
            ptr: 0
        }
    }

    fn run_instruction(&mut self, instr: Instruction) {
        // That's an awful lot of repeated code for 1 character to change...
        match instr {
            Instruction::Snd(target) => {
                match target {
                    Value::Constant(x) => self.sounded = x,
                    Value::Register(r) => self.sounded = *self.registers.entry(r).or_insert(0)
                }
            }
            Instruction::Set{target, value} => {
                // Eeeh...
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
                    if *t > 0 {
                        println!("toot: {}", t);
                        *t = self.sounded;
                    }
                }
            },
            Instruction::Jgz{register, offset} => {
                if let Value::Register(r) = register {
                    if *self.registers.entry(r).or_insert(0) > 0 {
                        match offset {
                            Value::Constant(x) => self.ptr += x as usize - 1,
                            Value::Register(r2) => self.ptr += *self.registers.entry(r2).or_insert(0) as usize - 1
                        }
                    }
                }
            },
            Instruction::None => {}
        }

        self.ptr += 1;
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

    let mut s = State::new();

    loop {
    //for _k in (0..10) {
        let i = instructions[s.ptr].clone();
        // I've had it!.clone()
        match i.clone() {
            Instruction::Rcv(_x) => {
                s.run_instruction(i);
                break;
            },
            _ => s.run_instruction(i)
        }
    }

    s.sounded
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
jgz a -2")), 2);
    }
}