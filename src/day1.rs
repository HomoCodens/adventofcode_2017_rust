#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
    .chars()
    .map(|c| {
        c.to_digit(10).unwrap()
    })
    .collect()
}

#[aoc(day1, part1)]
pub fn day1_part1(numbers: &Vec<u32>) -> u32 {
    let mut numbers_shifted = numbers.to_vec();
    numbers_shifted.rotate_left(1);

    let mut sum = 0;
    for (a, b) in numbers.iter().zip(numbers_shifted.iter()) {
        if a == b {
            sum += a;
        }
    }

    sum
}

#[aoc(day1, part2)]
pub fn day1_part2(numbers: &Vec<u32>) -> u32 {    
    let mut numbers_shifted = numbers.to_vec();
    let n_elements = numbers_shifted.len();
    numbers_shifted.rotate_left(n_elements / 2);

    let mut sum = 0;
    for (a, b) in numbers.iter().zip(numbers_shifted.iter()) {
        if a == b {
            sum += a;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::{day1_part1, day1_part2};

    #[test]
    fn example1() {
        assert_eq!(day1_part1(&vec!(1, 1, 2, 2)), 3)
    }

    #[test]
    fn example2() {
        assert_eq!(day1_part1(&vec!(1, 1, 1, 1)), 4)
    }

    #[test]
    fn example3() {
        assert_eq!(day1_part1(&vec!(1, 2, 3, 4)), 0)
    }

    #[test]
    fn example4() {
        assert_eq!(day1_part1(&vec!(9, 1, 2, 1, 2, 1, 2, 9)), 9)
    }

    #[test]
    fn example5() {
        assert_eq!(day1_part2(&vec!(1, 2, 1, 2)), 6)
    }
    #[test]
    fn example6() {
        assert_eq!(day1_part2(&vec!(1, 2, 2, 1)), 0)
    }
    #[test]
    fn example7() {
        assert_eq!(day1_part2(&vec!(1, 2, 3, 4, 2, 5)), 4)
    }
    #[test]
    fn example8() {
        assert_eq!(day1_part2(&vec!(1, 2, 3, 1, 2, 3)), 12)
    }

    #[test]
    fn example9() {
        assert_eq!(day1_part2(&vec!(1, 2, 1, 3, 1, 4, 1, 5)), 4)
    }
}