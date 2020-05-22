#[aoc_generator(day24)]
fn parse(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|l| {
        l.split("/").map(|p| {
            p.trim().parse().unwrap()
        }).collect() // good boy inferring the type. good boy!
    }).collect()
}

#[aoc(day24, part1)]
fn day24_part1(parts: &Vec<Vec<u32>>) -> u32 {
    let bridge = bridge_builder(0, vec!(), parts, false);

    bridge_strength(&bridge, parts)
}

#[aoc(day24, part2)]
fn day24_part2(parts: &Vec<Vec<u32>>) -> u32 {
    let bridge = bridge_builder(0, vec!(), parts, true);

    bridge_strength(&bridge, parts)
}

fn bridge_builder(exposed_port: u32, bridge: Vec<usize>, parts: &Vec<Vec<u32>>, optimize_length: bool) -> Vec<usize> {
    let candidate_parts = (0..parts.len())
                            // What is this, tidyverse?
                            .filter(|part_no| !bridge.contains(part_no))
                            .filter(|part_no| parts[*part_no].contains(&exposed_port))
                            .collect::<Vec<usize>>();

    if candidate_parts.len() == 0 {
        return bridge;
    }

    let mut best = 0;
    let mut best_bridge = vec!();
    for part_no in candidate_parts {
        let mut new_bridge = bridge.clone();
        new_bridge.push(part_no);
        let ep = if parts[part_no][0] == exposed_port {
            parts[part_no][1]
        } else {
            parts[part_no][0]
        };

        let next = bridge_builder(ep, new_bridge, parts, optimize_length);
        let next_str = bridge_strength(&next, parts);
        if next_str > best && (!optimize_length || next.len() >= best_bridge.len()) {
            best = next_str;
            best_bridge = next;
        }
    }

    best_bridge
}

fn bridge_strength(bridge: &Vec<usize>, parts: &Vec<Vec<u32>>) -> u32 {
    parts.iter().enumerate().filter(|(i, _)| {
        bridge.contains(i)
    })
    .map(|(_, part)| part)
    .flatten()
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn day24_bridge_strength() {
        let parts = vec!(
            vec!(0, 1),
            vec!(2, 3),
            vec!(3 ,4)
        );

        let bridge = vec!(0, 2);

        assert_eq!(bridge_strength(&bridge, &parts), 8);
    }

    #[test]
    fn day24_example() {
        let parts = parse(&"0/2
                            2/2
                            2/3
                            3/4
                            3/5
                            0/1
                            10/1
                            9/10");

        let br = bridge_builder(0, vec!(), &parts, false);

        assert_eq!(br, vec!(5, 6, 7));
    }

    #[test]
    fn day24_example_p2() {
        let parts = parse(&"0/2
                            2/2
                            2/3
                            3/4
                            3/5
                            0/1
                            10/1
                            9/10");

        let br = bridge_builder(0, vec!(), &parts, true);

        assert_eq!(br, vec!(0, 1, 2, 4));
    }

    #[test]
    fn day24_example_solver() {
        let parts = parse(&"0/2
                            2/2
                            2/3
                            3/4
                            3/5
                            0/1
                            10/1
                            9/10");

        assert_eq!(day24_part1(&parts), 31);
    }
}