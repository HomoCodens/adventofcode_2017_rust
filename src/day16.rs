use regex::Regex;

#[derive(Debug)]
enum DanceMove {
    Spin(usize),
    Exchange { a: usize, b: usize },
    Partner { a: String, b: String }
}

#[aoc_generator(day16)]
fn parse_dance_moves(input: &str) -> Vec<DanceMove> {
    let re = Regex::new(r"([sxp])([0-9a-p]+)/?(.+)?").expect("A parsing regex");

    input.split(",")
    .map(|mov| {
        let caps = re.captures(mov).expect("Captures");
        match caps.get(1).expect("captured id").as_str() {
            "s" => {
                DanceMove::Spin(caps.get(2).expect("captured spin value").as_str().trim().parse().expect("a parsed int"))
            },
            "x" => {
                DanceMove::Exchange {
                    a: caps.get(2).expect("captured exange partner 1").as_str().trim().parse().expect("xchange partner 1 parsed"),
                    b: caps.get(3).expect("captured exange partner 2").as_str().trim().parse().expect("xchange partner 2 parsed")
                }
            },
            "p" => {
                DanceMove::Partner {
                    a: String::from(caps.get(2).expect("captured exange partner 1").as_str().trim()),
                    b: String::from(caps.get(3).expect("captured exange partner 2").as_str().trim())
                }
            },
            _ => DanceMove::Spin(0)
        }
    }).collect()
}

fn do_de_dance(moves: &Vec<DanceMove>, times: usize) -> Vec<&str> {
    let mut dancers = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"];
    let n_moves = moves.len();

    for mov in moves.iter().cycle().take(times*n_moves) {
        //println!("{:?}", mov);
        match mov {
            DanceMove::Spin(n) => dancers.rotate_right(*n),
            DanceMove::Exchange{a, b} => dancers.swap(*a, *b),
            DanceMove::Partner{a, b} => {
                let p1 = dancers.iter().position(|x| x == a).unwrap();
                let p2 = dancers.iter().position(|x| x == b).unwrap();
                dancers.swap(p1, p2)
            }
        }
        //println!("{:?}", dancers);
    }

    dancers.to_vec()
}

fn dance_period(moves: &Vec<DanceMove>) -> usize {
    let dancers0 = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p"].to_vec();
    
    let mut period = 1;
    while !do_de_dance(moves, period).iter().eq(dancers0.iter()) {
        period += 1;
    }

    period
}

#[aoc(day16, part1)]
fn day16_part1(moves: &Vec<DanceMove>) -> String {
    do_de_dance(moves, 1).join("")
}

#[aoc(day16, part2)]
fn day16_part2(moves: &Vec<DanceMove>) -> String {
    let period = dance_period(moves);

    println!("By the way, they repeat after {} dances", period);

    do_de_dance(moves, 1000000000 % period).join("")
}