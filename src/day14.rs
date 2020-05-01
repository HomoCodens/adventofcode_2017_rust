use crate::day10::day10_part2;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::algo::connected_components;

fn integer_to_bits(x: i32) -> Vec<bool> {
    (0..8).rev().map(|i| {
        (1 << i) & x > 0
    }).collect()
} 

fn prttyprnt(img: &Vec<Vec<bool>>) -> () {
    for i in 0..8 {
        println!("{}", img[i][0..8].iter().map(|x| if *x { return String::from("#") } else { return String::from(".") }).collect::<Vec<String>>().join(""));
    }
}

#[aoc_generator(day14)]
fn disk_image(input: &str) -> Vec<Vec<bool>> {
    (0..128).map(|i| {
        let hash = day10_part2(&format!("{}-{}", input, i));
        let mut hashi = hash.chars();
        let mut parts = vec!();
        for _i in 0..16 {
            parts.push(i32::from_str_radix(&hashi.by_ref().take(2).collect::<String>(), 16).expect("A proppa hex value"));
        }
        if i == 0 {
            println!("{}", hash);
            println!("{:?}", parts.iter().flat_map(|x| integer_to_bits(*x)).collect::<Vec<bool>>())
        }

        parts.iter().flat_map(|x| integer_to_bits(*x)).collect()
    }).collect()
}

#[aoc(day14, part1)]
fn day14_part1(img: &Vec<Vec<bool>>) -> i32 {
    img.iter().flat_map(|row| {
        row.iter().map(|x| {
            if *x {
                return 1;
            } else {
                return 0;
            }
        })
    }).sum()
}

#[aoc(day14, part2)]
fn day14_part2(img: &Vec<Vec<bool>>) -> usize {
    prttyprnt(img);

    let mut edges = vec!();
    for i in 0..128 {
        for j in 0..128 {
            if img[i][j] {
                edges.push((NodeIndex::new(128*i+j), NodeIndex::new(128*i+j)));
                if i < 127 && img[i + 1][j] {
                    if i < 7 && j < 7 {
                        println!("Making edge {:?}-{:?}", (i, j), (i+1, j));
                    }
                    edges.push((NodeIndex::new(128*i + j), NodeIndex::new(128*(i+1) + j)));
                }
                if j < 127 && img[i][j + 1] {
                    if i < 7 && j < 7 {
                        println!("Making edge {:?}-{:?}", (i, j), (i, j+1));
                    }
                    edges.push((NodeIndex::new(128*i + j), NodeIndex::new(128*i + j + 1)));
                }
            }
        }
    }
    let mut graph = UnGraph::<usize, ()>::from_edges(edges);
    // I en't never asked for that many nodes...
    graph.retain_nodes(|g, n| {
        g.neighbors(n).count() > 0
    });

    connected_components(&graph)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d14ex1() {
        assert_eq!(day14_part1(
            &disk_image("flqrgnkx")), 8108);
    }

    #[test]
    fn d14ex2() {
        assert_eq!(day14_part2(
            &disk_image("flqrgnkx")), 1242);
    }
}