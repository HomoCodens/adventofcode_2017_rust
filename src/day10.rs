fn knotify<'a, I>(sizes: I) -> Vec<i32> 
where I: IntoIterator<Item = &'a usize>,
{
    let mut skip = 0;
    let mut marks: Vec<i32> = (0..256).collect();
    let mut start = 0;

    for s in sizes {
        // You're cloning some stuff again Arnold
        // You're taking the values and making copies...
        let repl: Vec<i32> = marks.iter().cloned().take(*s).rev().collect();
        marks.splice(..s, repl.iter().cloned());
        let rot = (s + skip) % marks.len();
        marks.rotate_left(rot);
        start += rot;
        skip += 1;
    }

    let reset = start % marks.len();
    marks.rotate_right(reset);
    marks
}

#[aoc(day10, part1)]
pub fn day10_part1(input: &str) -> i32 {
    let sizes: Vec<usize> = input.split(",").map(|x| x.trim().parse().expect("An input number")).collect();
    
    let marks = knotify(sizes.iter());

    marks[0]*marks[1]
}

#[aoc(day10, part2)]
pub fn day10_part2(input: &str) -> String {
    let mut sizes: Vec<usize> = input.chars().map(|c| c as usize).collect();
    sizes.append(&mut vec!(17, 31, 73, 47, 23));

    let n_sizes = sizes.len();

    /*let sparse = knotify(sizes.iter().cycle().take(64*n_sizes));

    let dense: Vec<i32> = sparse.chunks(16).map(|chunk| (*chunk).to_vec().iter().fold(0, |acc, x| acc ^ *x)).collect();

    let chunks: Vec<String> = dense.iter().map(|x| format!("{:02x}", x)).collect();

    chunks.join("")*/

    knotify(sizes.iter().cycle().take(64*n_sizes))
    .chunks(16)
    .map(|chunk| (*chunk).to_vec().iter().fold(0, |acc, x| acc ^ *x))
    .map(|x| format!("{:02x}", x))
    .collect::<Vec<String>>()
    .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d10ex1() {
        assert_eq!(day10_part2(&""), "a2582a3a0e66e6e86e3812dcb672a272");
    }

    #[test]
    fn d10ex2() {
        assert_eq!(day10_part2(&"AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
    }

    #[test]
    fn d10ex3() {
        assert_eq!(day10_part2(&"1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
    }

    #[test]
    fn d10ex4() {
        assert_eq!(day10_part2(&"1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
    }
}