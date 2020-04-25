use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Relation {
    GR,
    GE,
    LT,
    LE,
    EQ,
    NE
}

#[derive(Debug)]
pub struct Condition {
    register: String,
    relation: Relation,
    value: i32
}

#[derive(Debug)]
pub enum Operation {
    Inc,
    Dec
}

#[derive(Debug)]
pub struct Instruction {
    register: String,
    operation: Operation,
    value: i32,
    condition: Condition
}

fn parse_line (text: &str) -> Instruction {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"^(\w+) (inc|dec) (-?\d+) if (\w+) (.{1,2}) (-?\d+)").unwrap();
    }
    let cap = REGEX.captures(text).expect("Regex match");

    Instruction {
        register: String::from(cap.get(1).expect("Register match").as_str()),
        operation: match cap.get(2).expect("Operation").as_str() {
            "inc" => Operation::Inc,
            "dec" => Operation::Dec,
            _ => Operation::Inc
        },
        value: cap.get(3).expect("Value match").as_str().trim().parse().expect("Parsed number"),
        condition: Condition {
            register: String::from(cap.get(4).expect("Comparison register match").as_str()),
            relation: match cap.get(5).expect("Comparator").as_str() {
                "==" => Relation::EQ,
                "!=" => Relation::NE,
                "<" => Relation::LT,
                "<=" => Relation::LE,
                ">" => Relation::GR,
                ">=" => Relation::GE,
                _ => Relation::EQ
            },
            value: cap.get(6).expect("Value match").as_str().trim().parse().expect("Parsed number")
        }
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
    input.lines()
    .map(parse_line)
    .collect()
}

#[aoc(day8, part1)]
fn day8_part1(instructions: &Vec<Instruction>) -> i32 {
    let mut registers: HashMap<String, i32> = HashMap::new();

    for i in instructions {
        if match i.condition.relation {
            // I'd love to not clone... but how?
            Relation::EQ => *registers.entry(i.condition.register.clone()).or_insert(0) == i.condition.value,
            Relation::NE => *registers.entry(i.condition.register.clone()).or_insert(0) != i.condition.value,
            Relation::GR => *registers.entry(i.condition.register.clone()).or_insert(0) > i.condition.value,
            Relation::GE => *registers.entry(i.condition.register.clone()).or_insert(0) >= i.condition.value,
            Relation::LT => *registers.entry(i.condition.register.clone()).or_insert(0) < i.condition.value,
            Relation::LE => *registers.entry(i.condition.register.clone()).or_insert(0) <= i.condition.value
        } {
            match i.operation {
                Operation::Inc => *registers.entry(i.register.clone()).or_insert(0) += i.value,
                Operation::Dec => *registers.entry(i.register.clone()).or_insert(0) -= i.value
            }
        }
    }

    *registers.values().max().expect("...")
}

#[aoc(day8, part2)]
fn day8_part2(instructions: &Vec<Instruction>) -> i32 {
    let mut registers: HashMap<String, i32> = HashMap::new();

    let mut total_max: i32 = -9999;

    for i in instructions {
        if match i.condition.relation {
            // I'd love to not clone... but how?
            Relation::EQ => *registers.entry(i.condition.register.clone()).or_insert(0) == i.condition.value,
            Relation::NE => *registers.entry(i.condition.register.clone()).or_insert(0) != i.condition.value,
            Relation::GR => *registers.entry(i.condition.register.clone()).or_insert(0) > i.condition.value,
            Relation::GE => *registers.entry(i.condition.register.clone()).or_insert(0) >= i.condition.value,
            Relation::LT => *registers.entry(i.condition.register.clone()).or_insert(0) < i.condition.value,
            Relation::LE => *registers.entry(i.condition.register.clone()).or_insert(0) <= i.condition.value
        } {
            match i.operation {
                Operation::Inc => *registers.entry(i.register.clone()).or_insert(0) += i.value,
                Operation::Dec => *registers.entry(i.register.clone()).or_insert(0) -= i.value
            }
        }

        match registers.values().max() {
            Some(x) => { if *x > total_max { total_max = *x }},
            None => {}
        }
    }

    total_max
}