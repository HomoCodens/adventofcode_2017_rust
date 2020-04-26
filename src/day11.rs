#[derive(Debug, Clone)]
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW
}


// Can you tell I'm doing this for fun? ;P
#[derive(Debug)]
enum Majiked {
    Fizzle,
    Poof(Direction),
    Bang
}


fn majik(a: Option<&Direction>, b: Option<&Direction>) -> Majiked {
    match a {
        Some(Direction::N) => {
            match b {
                Some(Direction::S) => Majiked::Bang,
                Some(Direction::SW) => Majiked::Poof(Direction::NW),
                Some(Direction::SE) => Majiked::Poof(Direction::NE),
                _ => Majiked::Fizzle
            }
        },
        Some(Direction::NE) => {
            match b {
                Some(Direction::SW) => Majiked::Bang,
                Some(Direction::NW) => Majiked::Poof(Direction::N),
                Some(Direction::S) => Majiked::Poof(Direction::SE),
                _ => Majiked::Fizzle
            }
        },
        Some(Direction::SE) => {
            match b {
                Some(Direction::NW) => Majiked::Bang,
                Some(Direction::N) => Majiked::Poof(Direction::NE),
                Some(Direction::SW) => Majiked::Poof(Direction::S),
                _ => Majiked::Fizzle
            }
        },
        Some(Direction::S) => {
            match b {
                Some(Direction::N) => Majiked::Bang,
                Some(Direction::NE) => Majiked::Poof(Direction::SE),
                Some(Direction::NW) => Majiked::Poof(Direction::SW),
                _ => Majiked::Fizzle
            }
        },
        Some(Direction::SW) => {
            match b {
                Some(Direction::NE) => Majiked::Bang,
                Some(Direction::SE) => Majiked::Poof(Direction::S),
                Some(Direction::N) => Majiked::Poof(Direction::NW),
                _ => Majiked::Fizzle
            }
        },
        Some(Direction::NW) => {
            match b {
                Some(Direction::SE) => Majiked::Bang,
                Some(Direction::S) => Majiked::Poof(Direction::SW),
                Some(Direction::NE) => Majiked::Poof(Direction::N),
                _ => Majiked::Fizzle
            }
        },
        None => Majiked::Fizzle
    }
}

fn reduce_path(mut path: Vec<Direction>) -> Vec<Direction> {
    // Would be easier to just tabulate but I'm too enamoured with my Majik to turn back
    // Think of it as a Fingerübung for Enums
    let mut at = 0;
    let mut ninc = false;
    loop {
        //println!("{:?}", at);
        //println!("{:?}", path.len());
        //println!("{:?}", path);
        for i in at..path.len() {
            match majik(path.get(at), path.get(i)) {
                Majiked::Poof(new) => { 
                    /*let out: Vec<String> = path.iter().enumerate().map(|(ind, s)| format!("{:?}", if ind == at { format!("[{:?}]", s) } else if ind == i { format!("({:?})", s) } else { format!("{:?}", s) })).collect();
                    println!("{:?}", out.join(", "));
                    println!("crackle!");
                    println!("{:?}", new);*/
                    *path.get_mut(at).expect("bla") = new;
                    path.remove(i);
                    at = 0;
                    ninc = true;
                    break;
                },
                Majiked::Bang => { 
                    /*let out: Vec<String> = path.iter().enumerate().map(|(ind, s)| format!("{:?}", if ind == at { format!("[{:?}]", s) } else if ind == i { format!("({:?})", s) } else { format!("{:?}", s) })).collect();
                    println!("{:?}", out.join(", "));
                    println!("pop!");*/
                    path.remove(at);
                    // Note: path[i] moved 1 to the left because of the remove!
                    path.remove(i - 1);
                    at = 0;
                    ninc = true;
                    break;
                },
                Majiked::Fizzle => {
                    ninc = false;
                    //println!("snap!");
                }
            }
        }

        if !ninc {
            at += 1;
        }

        if path.len() == 0 || at == path.len() - 1 {
            break;
        }
    }

    path
}


#[aoc(day11, part1)]
pub fn day11_part1(input: &str) -> usize {
    let path: Vec<Direction> = input.split(",").map(|s| {
        match s {
            "n" => Direction::N,
            "ne" => Direction::NE,
            "se" => Direction::SE,
            "s" => Direction::S,
            "sw" => Direction::SW,
            "nw" => Direction::NW,
            _ => Direction::S // Ever southward. Wait, which pole does Santa live at?
        }
    }).collect();

    let path = reduce_path(path);

    println!("{:?}", path);

    path.len()
}

#[aoc(day11, part2)]
pub fn day11_part2(input: &str) -> usize {
    let mut max = 0;

    let path: Vec<Direction> = input.split(",").map(|s| {
        match s {
            "n" => Direction::N,
            "ne" => Direction::NE,
            "se" => Direction::SE,
            "s" => Direction::S,
            "sw" => Direction::SW,
            "nw" => Direction::NW,
            _ => Direction::S // Ever southward. Wait, which pole does Santa live at?
        }
    }).collect();

    // reduce_path bugs out if path.len() starts <= 1 ;)
    for i in 2..path.len() {
        println!("{:?}", i);
        let pr = reduce_path(path.iter().cloned().take(i).collect());
        if pr.len() > max {
            max = pr.len()
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d11ex1() {
        assert_eq!(day11_part1(&"ne,ne,ne"), 3);
    }
    #[test]
    fn d11ex2() {
        assert_eq!(day11_part1(&"ne,ne,sw,sw"), 0);
    }
    #[test]
    fn d11ex3() {
        assert_eq!(day11_part1(&"ne,ne,s,s"), 2);
    }
    #[test]
    fn d11ex4() {
        assert_eq!(day11_part1(&"se,sw,se,sw,sw"), 3);
    }
}