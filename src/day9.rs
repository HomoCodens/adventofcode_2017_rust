#[aoc(day9, part1)]
fn day9_part1(stream: &str) -> i32 {
    let mut score = 0;
    let mut skippimg = false;
    let mut garbage: bool = false;
    let mut level: i32 = 0;

    for c in stream.chars() {
        if skippimg {
            skippimg = false;
            continue;
        }

        match c {
            '{' => {if !garbage { level += 1; }},
            '}' => {if !garbage { score += level; level -= 1; }},
            '<' => {garbage = true},
            '>' => {garbage = false},
            '!' => {skippimg = true},
            _ => {}
        }
    }

    score
}

#[aoc(day9, part2)]
fn day9_part2(stream: &str) -> i32 {
    let mut skippimg = false;
    let mut garbage: bool = false;
    let mut n_garbage = 0;
    let mut enter_garbage;

    for c in stream.chars() {
        enter_garbage = false;

        if skippimg {
            skippimg = false;
            continue;
        }

        match c {
            '<' => {if !garbage { enter_garbage = true; } garbage = true},
            '>' => {garbage = false},
            '!' => {skippimg = true},
            _ => {}
        }

        if garbage && !skippimg && !enter_garbage {
            n_garbage += 1;
        }
    }

    n_garbage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d9ex1() {
        assert_eq!(day9_part1(&"{}"), 1)
    }
    #[test]
    fn d9ex2() {
        assert_eq!(day9_part1(&"{{{}}}"), 6)
    }
    #[test]
    fn d9ex3() {
        assert_eq!(day9_part1(&"{{},{}}"), 5)
    }
    #[test]
    fn d9ex4() {
        assert_eq!(day9_part1(&"{{{},{},{{}}}}"), 16)
    }
    #[test]
    fn d9ex5() {
        assert_eq!(day9_part1(&"{<a>,<a>,<a>,<a>}"), 1)
    }
    #[test]
    fn d9ex6() {
        assert_eq!(day9_part1(&"{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9)
    }
    #[test]
    fn d9ex7() {
        assert_eq!(day9_part1(&"{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9)
    }
    #[test]
    fn d9ex8() {
        assert_eq!(day9_part1(&"{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3)
    }
    #[test]
    fn d9ex9() {
        assert_eq!(day9_part2(&"<>"), 0)
    }
    #[test]
    fn d9ex10() {
        assert_eq!(day9_part2(&"<random characters>"), 17)
    }
    #[test]
    fn d9ex11() {
        assert_eq!(day9_part2(&"<<<<>"), 3)
    }
    #[test]
    fn d9ex12() {
        assert_eq!(day9_part2(&"<{!>}>"), 2)
    }
    #[test]
    fn d9ex13() {
        assert_eq!(day9_part2(&"<!!>"), 0)
    }
    #[test]
    fn d9ex14() {
        assert_eq!(day9_part2(&"<!!!>>"), 0)
    }
    #[test]
    fn d9ex15() {
        assert_eq!(day9_part2(&"<{o\"i!a,<{i<a>"), 10)
    }
}