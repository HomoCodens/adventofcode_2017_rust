use std::collections::HashMap;
use regex::Regex;

// Ya... https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

#[derive(Debug, Clone)]
pub struct Node {
    id: String,
    weight: i32,
    parent: Option<String>,
    childs: Vec<String>
}

impl Node {
    fn from_aoc_string(string: &str) -> Node {
        let re = Regex::new(r"^(\w+) \((\d+)\)(?: -> (.*))?$").unwrap();
        let components = re.captures(&string).unwrap();

        let id = components.get(1).unwrap().as_str();
        let weight = components.get(2).unwrap().as_str().trim().parse().unwrap();
        let childs = match components.get(3) {
            Some(list) => list.as_str().split(", ").map(|s| { String::from(s) }).collect(),
            None => vec!()
        };

        Self {
            id: String::from(id),
            weight,
            parent: None,
            childs
        }
    }
}

pub struct Tree {
    nodes: HashMap<String, Node>
}

impl Tree {
    fn get_root(&self) -> Option<&Node> {
        if let Some(first) = self.nodes.iter().next() {
            let mut n = first.1;
            loop {
                if n.parent == None {
                    return Some(n);
                }
    
                let abla = (n.parent.clone()).unwrap();
    
                n = self.nodes.get(&abla).unwrap();
            }
        } else {
            None
        }
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Tree {
    let nodes: Vec<Node> = input.lines()
    .map(|l| { Node::from_aoc_string(l) })
    .collect();

    let mut tree_nodes = HashMap::new();

    // This is so not how it should be done (methinks...)
    for n in nodes.iter().cloned() {
        tree_nodes.insert(n.id.clone(), n);
    }

    for n in nodes.iter() {
        for c in n.childs.iter() {
            if let Some(child) = tree_nodes.get_mut(c) {
                (*child).parent = Some(n.id.clone()); // For a language that claims to avoid pointers this cares an awful lot about cloning primitives (Yeahyeah, String is not a primitive...)
            }
        }
    }

    Tree {
        nodes: tree_nodes
    }
}

#[aoc(day7, part1)]
fn day7_part1(tree: &Tree) -> String {
    if let Some(node) = tree.get_root() {
        return /* Now that's just plain stoopy... */ node.id.clone();
    } else {
        String::from("Nope")
    }
}

// Isn't Rust supposed to make me better?
fn calculate_tree_weight(tree: &Tree, node: &String, weights: &mut HashMap<String, i32>) -> i32 {
    if weights.contains_key(node) {
        return *weights.get(node).unwrap();
    } else {
        let node = tree.nodes.get(node).unwrap();
        if node.childs.len() == 0 {
            weights.insert(node.id.clone(), node.weight);
            return node.weight;
        } else {
            let mut weight = node.weight;
            for c in node.childs.iter().cloned() {
                weight += calculate_tree_weight(tree, &c, weights)
            }
            weights.insert(node.id.clone(), weight);
            return weight;
        }
    }
}

fn tree_walk(current_node_id: &String, tree: &Tree, weights: &HashMap<String, i32>) -> Option<i32> {
    let node = tree.nodes.get(current_node_id).expect("What gives, Eric?");
    let children_weights: HashMap<&String, &i32> = weights.iter().filter(|(id, _w)| node.childs.contains(id)).collect();

    // All children are the same weight: We are not part of the broken path
    if children_weights.values().min() == children_weights.values().max() {
        return None;
    }

    let out: Vec<Option<i32>> = children_weights.keys().map(|k| tree_walk(*k, tree, weights)).collect();

    for bla in out {
        match bla {
            Some(x) => return Some(x),
            None => ()
        }
    }

    // Oh noes one of our childs is the one!
    let weight_table = children_weights.iter().fold(HashMap::new(), |mut map, (id, weight)| {
        // Look ma, I'm almost good enough to be clever!
        map.entry(*weight).and_modify(|e| *e = None).or_insert(Some(*id));
        return map;
    });

    let off_node: HashMap<&i32, &String> = weight_table.iter().filter(|(_id, x)| x.is_some()).map(|(id, x)| (*id, x.unwrap())).collect();
    let off_id: Vec<&String> = off_node.values().map(|x| *x).collect();
    let off_weight: Vec<&i32> = off_node.keys().map(|x| *x).collect();
    let correct_weight: Vec<&i32> = weight_table.iter().filter(|(_id, x)| !x.is_some()).map(|(id, _x)| *id).collect();

    let weight_offset = off_weight[0] - correct_weight[0];
    let off_node_weight = tree.nodes.get(off_id[0]).unwrap().weight;
    Some(off_node_weight - weight_offset)
}

#[aoc(day7, part2)]
fn day7_part2(tree: &Tree) -> i32 {
    let root = day7_part1(tree);
    println!("The root is {:?}", root);
    
    let mut weights = HashMap::new();
    calculate_tree_weight(tree, &root, &mut weights);

    match tree_walk(&root, tree, &weights) {
        Some(x) => x,
        None => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d7input() {
        input_generator("pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)");
    }

    #[test]
    fn part2() {
        day7_part2(&input_generator("pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)"));
    }
}