#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
    .lines()
    .map(|l| {
        l.trim().parse().unwrap()
    })
    .collect()
}

#[aoc(day5, part1)]
fn day5_part1(jumps: &Vec<i32>) -> u32 {
    let mut jumps = jumps.to_vec();

    let mut i = 0;
    let mut n_jumps = 0;
    loop {
        match jumps.get_mut(i as usize) {
            Some(offset) => {
                i += *offset;
                *offset += 1;
            },
            None => break
        }
        n_jumps += 1;
    }
    n_jumps
}

#[aoc(day5, part2)]
fn day5_part2(jumps: &Vec<i32>) -> u32 {
    let mut jumps = jumps.to_vec();

    let mut i = 0;
    let mut n_jumps = 0;
    loop {
        match jumps.get_mut(i as usize) {
            Some(offset) => {
                i += *offset;

                if *offset >= 3 {
                    *offset -= 1;
                } else {
                    *offset += 1;
                }
            },
            None => break
        }
        n_jumps += 1;
    }
    n_jumps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        assert_eq!(input_generator("1\n0\n123\n-54"), vec!(1, 0, 123, -54));
    }

    #[test]
    fn example1() {
        assert_eq!(day5_part1(&vec!(0, 3, 0, 1, -3)), 5);
    }

    #[test]
    fn example2() {
        assert_eq!(day5_part2(&vec!(0, 3, 0, 1, -3)), 10);
    }
}