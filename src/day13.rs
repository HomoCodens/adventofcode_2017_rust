#[derive(Debug)]
pub struct Layer {
    depth: i32,
    range: i32
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Layer> {
    input.lines()
    .map(|l| {
        let mut comps = l.split(": ");
        let depth: i32 = comps.next().expect("a depth").parse().expect("a parsed depth");
        let range: i32 = comps.next().expect("a range").parse().expect("a parsed range");
        Layer {
            depth: depth,
            range: range
        }
    })
    .collect()
}

// Because part 2 is soo not obvious
fn severity(t_0: i32, layers: &Vec<Layer>) -> i32 {
    layers.iter().fold(0, |acc, layer| {
        if (layer.depth + t_0) % (2*layer.range - 2) == 0 {
            return acc + layer.depth * layer.range;
        } else {
            return acc;
        }
    })
}

// Because layer 0 does not add severity when caught >.<
fn caught_at_all(t_0: i32, layers: &Vec<Layer>) -> bool {
    layers.iter().any(|layer| {
        (layer.depth + t_0) % (2*layer.range - 2) == 0
    })
}

#[aoc(day13, part1)]
pub fn day13_part1(layers: &Vec<Layer>) -> i32 {
    severity(0, layers)
}

#[aoc(day13, part2)]
pub fn day13_part2(layers: &Vec<Layer>) -> i32 {
    let mut delay = 0;
    
    loop {
        if !caught_at_all(delay, layers) {
            break;
        }
        delay += 1;
    }

    delay
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d13ex1() {
        assert_eq!(day13_part1(&vec!(
            Layer{ depth: 0, range: 3 },
            Layer{ depth: 1, range: 2 },
            Layer{ depth: 4, range: 4 },
            Layer{ depth: 6, range: 4 })), 24);
    }

    #[test]
    fn d13ex2() {
        assert_eq!(day13_part2(&vec!(
            Layer{ depth: 0, range: 3 },
            Layer{ depth: 1, range: 2 },
            Layer{ depth: 4, range: 4 },
            Layer{ depth: 6, range: 4 })), 10);
    }
}