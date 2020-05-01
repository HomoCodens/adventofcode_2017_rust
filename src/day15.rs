use regex::Regex;

#[derive(Copy, Clone)]
struct Generator { // He wrote, knowing full well the naive approach would not be feasible in part 2
    state: u64,
    factor: u64,
    modulus: u64,
    picky: u64
}

impl Generator {
    fn new(initial_state: u64, factor: u64, picky: u64) -> Generator {
        Generator {
            state: initial_state,
            factor: factor,
            modulus: 2147483647,
            picky: picky
        }
    }
}

impl Iterator for Generator {
    type Item = u64;
    
    fn next(&mut self) -> Option<u64> {
        loop {
            self.state = (self.factor*self.state) % self.modulus;
            if self.picky == 0 || self.state % self.picky == 0 {
                break;
            }
        }
        Some(self.state)
    }
}

fn compare_lowest_bits(a: u64, b: u64) -> bool {
    (a ^ b) & 65535 == 0
}

#[aoc(day15, part1)]
fn day15_part1(input: &str) -> i32 {
    let re = Regex::new(r"(\d+)").expect("proper regex");
    let inits: Vec<u64> = input.lines().map(|l| {
        let cap = re.captures(l).expect("some captures");
        cap.get(1).expect("a capture").as_str().trim().parse().expect("a number")
    }).collect();


    let n = 40000000;
    let bla: i32 = Generator::new(inits[0], 16807, 0).zip(Generator::new(inits[1], 48271, 0))
    .take(n)
    .map(|(a, b)| if compare_lowest_bits(a, b) { return 1; } else { return 0; })
    .sum();
    
    bla
}

#[aoc(day15, part2)]
fn day15_part2(input: &str) -> i32 {
    let re = Regex::new(r"(\d+)").expect("proper regex");
    let inits: Vec<u64> = input.lines().map(|l| {
        let cap = re.captures(l).expect("some captures");
        cap.get(1).expect("a capture").as_str().trim().parse().expect("a number")
    }).collect();

    let n = 5000000;
    let bla: i32 = Generator::new(inits[0], 16807, 4).zip(Generator::new(inits[1], 48271, 8))
    .take(n)
    .map(|(a, b)| if compare_lowest_bits(a, b) { return 1; } else { return 0; })
    .sum();
    
    bla
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn d15ex1() {
        assert_eq!(day15_part1(&"65 \n 8921"), 588);
    }

    #[test]
    fn d15ex2() {
        assert_eq!(day15_part2(&"65 \n 8921"), 309);
    }
}