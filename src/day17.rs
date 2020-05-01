#[aoc_generator(day17)]
fn parse(input: &str) -> usize {
    input.trim().parse().expect("A number in the input")
}

#[aoc(day17, part1)]
fn day17_part1(stepsize: &usize) -> usize {
    let stepsize = *stepsize;

    let mut buffer = vec!(0);
    let mut pos = 0;
    for i in 0..2017 {
        // println!("{}", pos);
        /*println!("{:?}", buffer.iter().enumerate()
            .map(|(x, ii)| {
                if *ii == pos {
                    return format!("({})", x);
                } else {
                    return format!("{}", x);
                }
            }).collect::<Vec<String>>().join(" "));*/
        pos = ((pos + stepsize) % buffer.len()) + 1;
        buffer.insert(pos, i + 1);
    }
    buffer[pos + 1]
}

#[aoc(day17, part2)]
fn day17_part2(stepsize: &usize) -> usize {
    let stepsize = *stepsize;

    let mut pos = 0;
    let mut the_number = 1;
    for i in 0..50000000 {
        /*if i % 10000 == 0 {
            println!("{}", i);
        }*/
        pos = ((pos + stepsize) % (i + 1)) + 1;
        if pos == 1 {
            the_number = i + 1;
        }
    }

    the_number
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn input() {
        assert_eq!(parse(&"1337"), 1337);
    }

    #[test]
    fn input_with_whitespace() {
        assert_eq!(parse(&" 123 \n"), 123);
    }

    #[test]
    fn day17ex1() {
        assert_eq!(day17_part1(&3), 638);
    }
}