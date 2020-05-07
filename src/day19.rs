#[aoc_generator(day19)]
// so yeah, apparently input generators may not be called "input"
fn bliblablobb(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn walk_diagram(diagram: &Vec<Vec<char>>) -> (Vec<char>, usize) {
    let mut n_steps = 0;

    let n_col = diagram[0].len();
    let n_row = diagram.len();
    let mut x = 0;
    for i in 0..n_col {
        if diagram[0][i] != ' ' {
            x = i;
            break;
        }
    }

    let mut pos = (x, 0);
    let mut v: (i32, i32) = (0, 1);
    let mut letters = vec!();

    loop {
        n_steps += 1;
        //println!("{:?} - {:?}", pos, v);
        pos.0 = (pos.0 as i32 + v.0) as usize; // Yes, prettypromise I will make sure you never everer turn into something outside of (n_row, n_col)...
        pos.1 = (pos.1 as i32 + v.1) as usize; // REALLY???
        match diagram[pos.1][pos.0] {
            '|' | '-' => { /* jes keep on going */ },
            '+' => {
                match diagram[(pos.1 as i32 + v.1) as usize][(pos.0 as i32 + v.0) as usize] {
                    ' ' => {
                        if v.0 != 0 {
                            // Moving horizontally -> look up and down
                            if diagram[pos.1 - 1][pos.0] != ' ' {
                                v = (0, -1);
                            } else if pos.1 < (n_row - 1) && diagram[pos.1 + 1][pos.0] != ' ' {
                                v = (0, 1)
                            } else {
                                break;
                            }
                        } else {
                            // Moving vertically -> look left and right
                            if pos.0 > 0 && diagram[pos.1][pos.0 - 1] != ' ' {
                                v = (-1, 0);
                            } else if pos.0 < (n_col - 1) && diagram[pos.1][pos.0 + 1] != ' ' {
                                v = (1, 0);
                            } else {
                                break;
                            }
                        }
                    },
                    _ => { /* Not our intersection (yet/anymore) */ }
                }
            },
            _ => {
                if diagram[pos.1][pos.0] == ' ' {
                    //panic!("volcano!"); // Aah, memory imprints...
                    break;
                }
                println!("Oh, a lettre! {}", diagram[pos.1][pos.0]);
                letters.push(diagram[pos.1][pos.0]);
            }
        }
    }

    (letters, n_steps)
}

#[aoc(day19, part1)]
fn day19_part1(diagram: &Vec<Vec<char>>) -> String {
    let (letters, _) = walk_diagram(diagram);
    
    letters.iter().collect()
}

#[aoc(day19, part2)]
fn day19_part2(diagram: &Vec<Vec<char>>) -> usize {
    let (_, steps) = walk_diagram(diagram);
    
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn day19ex1() {
        assert_eq!(day19_part1(&bliblablobb(&"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
                ")), String::from("ABCDEF"));
    }

    #[test]
    fn day19ex2() {
        assert_eq!(day19_part2(&bliblablobb(&"     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
                ")), 38);
    }
}