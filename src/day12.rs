use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::visit::Bfs;

use regex::Regex;
use std::collections::HashSet;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> UnGraph<usize, ()> {
    let re = Regex::new(r"^(\d+) <-> (.*?)$").expect("A parsing regex");
    
    let edges = input.lines()
    .flat_map(|l| {
        let parts = re.captures(l).expect("captures");
        let node: usize = parts.get(1).expect("Match").as_str().trim().parse().expect("a number");
        parts.get(2).expect("second match")
        .as_str()
        .split(", ")
        .map(move |x| {
            let node2: usize = x.trim().parse().expect("a number 2");
            (NodeIndex::new(node), NodeIndex::new(node2))
        })
    });

    UnGraph::<usize, ()>::from_edges(edges)
}

#[aoc(day12, part1)]
fn day12_part1(g: &UnGraph<usize, ()>) -> i32 {
    let mut bfs = Bfs::new(g, NodeIndex::new(0));
    let mut cnt = 0;
    while let Some(bla) = bfs.next(g) {
        cnt += 1;
    }
    cnt
}

#[aoc(day12, part2)]
fn day12_part2(g: &UnGraph<usize, ()>) -> i32 {
    let mut n_clusters = 0;
    let mut unvisited_nodes: HashSet<usize> = (0..2000).collect();  // Yes I peeked

    loop {
        match unvisited_nodes.iter().next() {
            Some(id) => {
                let mut bfs = Bfs::new(g, NodeIndex::new(*id));
                while let Some(bla) = bfs.next(g) {
                    unvisited_nodes.remove(&bla.index());
                }
            },
            None => { break; }
        }
        n_clusters += 1;
    }
    n_clusters
}