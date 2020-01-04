use itertools::Itertools;

pub struct ExcelRow {
    numbers: Vec<i32>
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<ExcelRow> {
    input.lines()
    .map(|l| {
        let numbers : Vec<i32> = l.split_ascii_whitespace()
        .map(|n| {
            n.trim().parse().unwrap()
        })
        .collect();

        ExcelRow {
            numbers
        }
    })
    .collect()
}

#[aoc(day2, part1)]
pub fn day2_part1(rows: &Vec<ExcelRow>) -> i32 {
    rows.iter()
    .map(|row| {
        row.numbers.iter().max().unwrap() - row.numbers.iter().min().unwrap()
    })
    .sum::<i32>()
}

#[aoc(day2, part2)]
pub fn day2_part2(rows: &Vec<ExcelRow>) -> i32 {
    rows.iter()
    .map(|row| {
        for c in row.numbers.iter().combinations(2) {
            let a = c[0];
            let b = c[1];
            if (a % b) == 0 {
                return a / b;
            }

            if (b % a) == 0 {
                return b / a;
            }
        }
        0
    })
    .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::{day2_part1, day2_part2, ExcelRow};

    #[test]
    fn example1() {
        let input = vec!(
            ExcelRow {
                numbers: vec!(5, 1, 9, 5)
            },
            ExcelRow {
                numbers: vec!(7, 5, 3)
            },
            ExcelRow {
                numbers: vec!(2, 4, 6, 8)
            },
        );

        assert_eq!(day2_part1(&input), 18)
    }

    #[test]
    fn example2() {
        let input = vec!(
            ExcelRow {
                numbers: vec!(5, 9, 2, 8)
            },
            ExcelRow {
                numbers: vec!(9, 4, 7, 3)
            },
            ExcelRow {
                numbers: vec!(3, 8, 6, 5)
            },
        );

        assert_eq!(day2_part2(&input), 9)
    }
}