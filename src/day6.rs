use std::collections::HashMap;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
    .split_ascii_whitespace()
    .map(|l| {
        l.trim().parse().unwrap()
    })
    .collect()
}

#[aoc(day6, part1)]
fn day6_part1(banks: &Vec<i32>) -> u32 {

    let mut banks = banks.clone();
    let n_banks = banks.len();

    let mut states: HashMap<Vec<i32>, i32> = HashMap::new();
    let mut reps = 0;
    loop {
        let (imax, max) = banks.iter().cloned().enumerate().rev().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

        banks[imax] = 0;
        for ii in (imax+1)..(imax + 1 + max as usize) {
            banks[ii % n_banks] += 1;
        }

        reps += 1;
        if states.contains_key(&banks) {
            return reps;
        }
        
        // This feaws... icky
        states.insert(banks.iter().cloned().collect(), 1);
    }
}

#[aoc(day6, part2)]
fn day6_part2(banks: &Vec<i32>) -> u32 {

    let mut banks = banks.clone();
    let n_banks = banks.len();

    let mut states: HashMap<Vec<i32>, u32> = HashMap::new();
    let mut reps = 0;
    loop {
        reps += 1;
        
        let (imax, max) = banks.iter().cloned().enumerate().rev().max_by(|(_, a), (_, b)| a.cmp(b)).unwrap();

        banks[imax] = 0;
        for ii in (imax+1)..(imax + 1 + max as usize) {
            banks[ii % n_banks] += 1;
        }

        if states.contains_key(&banks) {
            return reps - states.get(&banks).unwrap();
        }
        
        // This feaws... icky
        states.insert(banks.iter().cloned().collect(), reps);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        assert_eq!(input_generator("1\t2\t3"), vec!(1, 2, 3));
    }

    #[test]
    fn example1() {
        assert_eq!(day6_part1(&vec!(0, 2, 7, 0)), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(day6_part2(&vec!(0, 2, 7, 0)), 4);
    }
}