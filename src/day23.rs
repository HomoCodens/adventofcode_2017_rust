use std::collections::HashMap;
use regex::Regex;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

// Yes, we c&p

#[derive(Debug, Clone)]
enum Value {
    Register(String),
    Constant(i64)
}

/*
set X Y sets register X to the value of Y.
sub X Y decreases register X by the value of Y.
mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
jnz X Y jumps with an offset of the value of Y, but only if the value of X is not zero. 
        (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
*/
#[derive(Debug, Clone)]
enum Instruction {
    Set{ target: Value, value: Value },
    Sub{ target: Value, value: Value },
    Mul{ target: Value, value: Value },
    Jnz{ register: Value, offset: Value },
    None
}

#[derive(Debug)]
struct State {
    registers: HashMap<String, i64>,
    ptr: usize
}

impl State {
    fn new() -> State {
        let mut s = State {
            registers: HashMap::new(),
            ptr: 0,
        };
        s
    }

    fn run_instruction(&mut self, instr: Instruction) -> Result<(), ()> {
        // That's an awful lot of repeated code for 1 character to change...
        match instr {
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
            Instruction::Sub{target, value} => {
                //println!("Adding! id: {}, target: {:?}, value: {:?}", self.id, target, value);
                if let Value::Register(r) = target {
                    match value {
                        Value::Constant(x) => *self.registers.entry(r).or_insert(0) -= x,
                        Value::Register(r2) => {
                            let v = *self.registers.entry(r2).or_insert(0);
                            *self.registers.entry(r).or_insert(0) -= v;
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
            Instruction::Jnz{register, offset} => {
                //println!("JNZing! id: {}, register: {:?}, offest: {:?}", self.id, register, offset);
                // Thanks @vash3r lol
                // https://www.reddit.com/r/adventofcode/comments/7kj35s/2017_day_18_solutions/
                match register {
                    Value::Register(r) => {
                        let &mut register_value = self.registers.entry(r).or_insert(0);
                        if register_value == 0 {
                            //println!("also, I'm not going anywheres!")
                        }
                        if register_value != 0 {
                            match offset {
                                Value::Constant(x) => self.ptr += x as usize - 1,
                                Value::Register(r2) => self.ptr += *self.registers.entry(r2).or_insert(0) as usize - 1
                            }
                        }
                    },
                    Value::Constant(x) => {
                        // This code is very WET (we eternally t...uplicate?)
                        if x != 0 {
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

#[aoc_generator(day23)]
fn parse_instructions(input: &str) -> Vec<Instruction> {
    let re = Regex::new(r"([a-z]{3}) ([0-9a-z]+)(?: (.+))?").expect("I can write proper regex, ya know...");

    input.lines().map(|l| {
        let caps = re.captures(l).expect("matches");
        let instr = caps.get(1).unwrap().as_str();
        match instr {
            "set" => Instruction::Set{
                target: get_value(caps.get(2).unwrap().as_str()),
                value: get_value(caps.get(3).unwrap().as_str())
            },
            "sub" => Instruction::Sub{
                target: get_value(caps.get(2).unwrap().as_str()),
                value: get_value(caps.get(3).unwrap().as_str())
            },
            "mul" => Instruction::Mul{
                target: get_value(caps.get(2).unwrap().as_str()),
                value: get_value(caps.get(3).unwrap().as_str())
            },
            "jnz" => Instruction::Jnz{
                register: get_value(caps.get(2).unwrap().as_str()),
                offset: get_value(caps.get(3).unwrap().as_str())
            },
            _ => Instruction::None
        }
    }).collect()
}

#[aoc(day23, part1)]
fn day23_part1(instructions: &Vec<Instruction>) -> i64 {
    println!("{:?}", instructions);

    let mut s = State::new();

    let mut n_mul = 0;

    loop {
    //for _k in (0..10) {
        if s.ptr >= instructions.len() {
            println!("program 0 exiting due to pointer out of range");
            break;
        }

        let i = instructions[s.ptr].clone();
        // I've had it!.clone()
        match i.clone() {
            Instruction::Mul{value: _value, target: _target} => {
                n_mul += 1;
            },
            _ => {}
        }

        match s.run_instruction(i) {
            Ok(_) => {},
            Err(_) => break
        }
    }

    n_mul
}

/*
# something something prime numbers I'm sure...

# init
set b 99
set c b

# if a is != 0 goto @part2
jnz a 2

# else goto @main
jnz 1 5

@part2
mul b 100
sub b -100000
set c b
sub c -17000

@main
# init part 1
set f 1
set d 2
set e 2

# outer loop 2:b
# loop 2:b
# g = d*e-b
set g d
mul g e
sub g b

# if d*e == b, set f to 0
jnz g 2
set f 0


sub e -1
set g e
sub g b
jnz g -8
# end loop 2:b

sub d -1
set g d
sub g b
jnz g -13
# end outer loop 2:b

# if b is not a multiple of 2 integers
# increment h
jnz f 2
sub h -1

# if g == 0 exit
# if b == c (always the case in part 1) exit
set g b
sub g c
jnz g 2
jnz 1 3

# else increment b by 17
sub b -17

# goto @main
jnz 1 -23
*/