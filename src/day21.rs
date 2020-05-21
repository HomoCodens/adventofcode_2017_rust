// Kids, today we are going to learn about Traits and Polymorphism.
// No Jimmy, that's not what YOU think...

trait Pattern: std::fmt::Debug {
    fn matches(&self, other: &Vec<bool>) -> bool;
}

#[derive(Eq)]
struct Pattern2 {
    pixels: Vec<bool>,
    n_lit: u8
}

impl Pattern2 {
    fn from_string(string: &str) -> Pattern2 {
        let pixels: Vec<bool> = string.chars().map(|c| c == '#').collect();
        Pattern2::from_bool(&pixels)
    }
    
    fn from_bool(pixels: &Vec<bool>) -> Pattern2 {
        let n_lit = pixels.iter().fold(0, |mut acc, x| { if *x { acc += 1 } acc });
        let pixels = vec!(pixels[0], pixels[1], pixels[3], pixels[2]);
        Pattern2 {
            pixels: pixels,
            n_lit: n_lit
        }
    }
}

impl Pattern for Pattern2 {
    fn matches(&self, other: &Vec<bool>) -> bool {
        other.len() == 4 && *self == Pattern2::from_bool(other)
    }
}

impl PartialEq for Pattern2 {
    fn eq(&self, other: &Pattern2) -> bool {
        if self.n_lit == other.n_lit && self.n_lit != 2 {
            return true;
        }
        
        // the good thing about 2x2 patterns is, that flipping don't matter
        for i in 0..4 {
            if self.pixels.iter().cycle().skip(i).take(4).eq(other.pixels.iter()) {
                return true;
            }
        }
        
        return false;
    }
}


impl PartialEq<Pattern3> for Pattern2 {
    fn eq(&self, _other: &Pattern3) -> bool {
        false
    }
}

impl std::fmt::Debug for Pattern2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.pixels.iter().map(|p| if *p { "#" } else { "." }).collect::<Vec<&str>>();
        let propr = vec!(s[0], s[1], s[3], s[2]).join("");
        write!(f, "{}", propr)
    }
}

#[derive(Eq)]
struct Pattern3 {
    center: bool,
    border: Vec<bool>,
    n_lit: u8
}

impl Pattern3 {
    fn from_string(string: &str) -> Pattern3 {
        let pixels: Vec<bool> = string.chars().map(|c| c == '#').collect();
        Pattern3::from_bool(&pixels)
    }

    fn from_bool(pixels: &Vec<bool>) -> Pattern3 {
        let n_lit = pixels.iter().fold(0, |mut acc, x| { if *x { acc += 1 } acc });
        let center = pixels[4];
        let border = vec!(pixels[0], pixels[1], pixels[2],
                            pixels[5], pixels[8],
                            pixels[7], pixels[6], pixels[3]);
        Pattern3 {
            center: center,
            border: border,
            n_lit: n_lit
        }
    }
}

impl Pattern for Pattern3 {
    fn matches(&self, other: &Vec<bool>) -> bool {
        other.len() == 9 && *self == Pattern3::from_bool(other)
    }
}

impl PartialEq for Pattern3 {
    fn eq(&self, other: &Pattern3) -> bool {
        if self.center != other.center || self.n_lit != other.n_lit {
            false
        } else {
            let border_flipped = vec!(self.border[6], self.border[5], self.border[4],
                                      self.border[3],
                                      self.border[2], self.border[1], self.border[0],
                                      self.border[7]);
            for i in 0..4 {
                if self.border.iter().cycle().skip(2*i).take(8).eq(other.border.iter()) {
                    return true;
                }
    
                if border_flipped.iter().cycle().skip(2*i).take(8).eq(other.border.iter()) {
                    return true;
                }
            }
    
            false
        }
    }
}

impl PartialEq<Pattern2> for Pattern3 {
    fn eq(&self, _other: &Pattern2) -> bool {
        false
    }
}

impl std::fmt::Debug for Pattern3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let border = self.border.iter().map(|p| if *p { "#" } else { "." }).collect::<Vec<&str>>();
        let center = if self.center { "#" } else { "." };
        let propr = vec!(border[0], border[1], border[2],
                            border[7], center, border[3],
                            border[6], border[5], border[4]).join("");
        write!(f, "{}", propr)
    }
}

#[derive(Debug)]
struct Rule {
    pattern: Box<dyn Pattern>,
    out: Vec<bool>
}

impl Rule {
    fn from_string(string: &str) -> Rule {
        let mut chunks = string.split(" => ");
        let rule = chunks.next().unwrap().replace("/", "");
        let output: Vec<bool> = chunks.next().unwrap().replace("/", "").chars().map(|c| c == '#').collect();

        // TODO: How to pattern::from_string. Want to finally get going tho
        if rule.chars().count() == 4 {
            Rule {
                pattern: Box::new(Pattern2::from_string(&rule[..])),
                out: output
            }
        } else {
            Rule {
                pattern: Box::new(Pattern3::from_string(&rule[..])),
                out: output
            }
        }
    }
}

// Guess who I'm reading at the moment
#[derive(Debug)]
struct Patternmaster {
    grid: Vec<bool>,
    rules: Vec<Rule>
}

impl Patternmaster {
    fn new(input: &str) -> Patternmaster {
        Patternmaster {
            grid: vec!(false, true, false,
                        false, false, true,
                        true, true, true),
            rules: input.lines().map(|l| Rule::from_string(l)).collect()
        }
    }

    fn evolve(&mut self) {
        let prt = self.partition();
        //println!("{:?}", prt);
        let res = prt.iter().map(|p| {
            // TODO: Get rid of this clone
            for r in &self.rules {
                if r.pattern.matches(p) {
                    //println!("Matched rule {:?}!", r);
                    return (*r).out.clone();
                }
            }
            // panic!?
            return vec!(false);
        }).collect();
        //println!("{:?}", res);
        self.assemble(res);
    }

    fn partition(&self) -> Vec<Vec<bool>> {
        let dim = (self.grid.len() as f32).sqrt() as usize;
        if self.grid.len() % 2 == 0 {
            //println!("Partitioning into chunks of 2");
            let griddim = dim / 2;
            return (0..griddim*griddim).map(|i| {
                let col = i % griddim;
                let row = i / griddim;
                vec!(self.grid[2*row*dim +       2*col], self.grid[2*row*dim +       2*col + 1],
                     self.grid[2*row*dim + dim + 2*col], self.grid[2*row*dim + dim + 2*col + 1])
            }).collect();
        } else {
            //println!("Partitioning into chunks of tweee");
            let griddim = dim / 3;
            // so dear rustc, why is v this v fine all of a sudden?
            return (0..griddim*griddim).map(|i| {
                let col = i % griddim;
                let row = i / griddim;
                vec!(self.grid[3*row*dim +         3*col], self.grid[3*row*dim +         3*col + 1], self.grid[3*row*dim +         3*col + 2],
                     self.grid[3*row*dim +   dim + 3*col], self.grid[3*row*dim +   dim + 3*col + 1], self.grid[3*row*dim +   dim + 3*col + 2],
                     self.grid[3*row*dim + 2*dim + 3*col], self.grid[3*row*dim + 2*dim + 3*col + 1], self.grid[3*row*dim + 2*dim + 3*col + 2])
            }).collect();
        }
    }

    fn assemble(&mut self, parts: Vec<Vec<bool>>) {
        let size = parts[0].len();
        let n: usize = parts.len() * size;
        let dim = (n as f32).sqrt() as usize;
        let griddim = (parts.len() as f32).sqrt() as usize;

        let mut new_grid = vec![false; n];
        //println!("assembling into an grid of size {}", n);

        for i in 0..parts.len() {
            let p = &parts[i];
            let col = i % griddim;
            let row = i / griddim;
            if size == 9 {
                // I. am. COMPILER! ;P
                new_grid[3*row*dim +         3*col    ] = p[0];
                new_grid[3*row*dim +         3*col + 1] = p[1];
                new_grid[3*row*dim +         3*col + 2] = p[2];
                new_grid[3*row*dim +   dim + 3*col    ] = p[3];
                new_grid[3*row*dim +   dim + 3*col + 1] = p[4];
                new_grid[3*row*dim +   dim + 3*col + 2] = p[5];
                new_grid[3*row*dim + 2*dim + 3*col    ] = p[6];
                new_grid[3*row*dim + 2*dim + 3*col + 1] = p[7];
                new_grid[3*row*dim + 2*dim + 3*col + 2] = p[8];
            } else {
                new_grid[4*row*dim         + 4*col    ] = p[0];
                new_grid[4*row*dim         + 4*col + 1] = p[1];
                new_grid[4*row*dim         + 4*col + 2] = p[2];
                new_grid[4*row*dim         + 4*col + 3] = p[3];
                new_grid[4*row*dim +   dim + 4*col    ] = p[4];
                new_grid[4*row*dim +   dim + 4*col + 1] = p[5];
                new_grid[4*row*dim +   dim + 4*col + 2] = p[6];
                new_grid[4*row*dim +   dim + 4*col + 3] = p[7];
                new_grid[4*row*dim + 2*dim + 4*col    ] = p[8];
                new_grid[4*row*dim + 2*dim + 4*col + 1] = p[9];
                new_grid[4*row*dim + 2*dim + 4*col + 2] = p[10];
                new_grid[4*row*dim + 2*dim + 4*col + 3] = p[11];
                new_grid[4*row*dim + 3*dim + 4*col    ] = p[12];
                new_grid[4*row*dim + 3*dim + 4*col + 1] = p[13];
                new_grid[4*row*dim + 3*dim + 4*col + 2] = p[14];
                new_grid[4*row*dim + 3*dim + 4*col + 3] = p[15];
            }
        }

        self.grid = new_grid;
    }

    fn get_n_lit(&self) -> u64 {
        self.grid.iter().fold(0, |acc, x| acc + (*x as u64))
    }
}

impl std::fmt::Display for Patternmaster {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dim = (self.grid.len() as f64).sqrt() as usize;
        let st = self.grid.chunks(dim).map(|c| {
            c.iter().map(|cell| {
                if *cell { "#" } else { "." }
            })
            .collect::<Vec<&str>>()
            .join("")
        })
        .collect::<Vec<String>>()
        .join("\n");

        write!(f, "{}", st)
    }
}

#[aoc(day21, part1)]
fn day21_part1(input: &str) -> u64 {
    let mut rayal = Patternmaster::new(input);
    println!("{}", rayal);

    for _i in 0..5 {
        rayal.evolve();
        println!("{}", rayal);
    }

    rayal.get_n_lit()

}

#[aoc(day21, part2)]
fn day21_part2(input: &str) -> u64 {
    let mut teray /* spoilers! */ = Patternmaster::new(input);
    println!("{}", teray);

    for _i in 0..18 {
        teray.evolve();
        //println!("{}", teray);
    }

    teray.get_n_lit()

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn day21_pattern2_1() {
        let a = Pattern2 {
            pixels: vec!(false, false, false, false),
            n_lit: 0
        };

        let b = Pattern2 {
            pixels: vec!(false, false, false, false),
            n_lit: 0
        };

        assert_eq!(a, b);
    }

    #[test]
    fn day21_pattern2_2() {
        let a = Pattern2 {
            pixels: vec!(true, false, false, false),
            n_lit: 1
        };

        let b = Pattern2 {
            pixels: vec!(false, false, false, false),
            n_lit: 0
        };

        assert_ne!(a, b);
    }

    #[test]
    fn day21_pattern2_3() {
        let a = Pattern2 {
            pixels: vec!(true, true, false, false),
            n_lit: 2
        };

        let b = Pattern2 {
            pixels: vec!(false, true, true, false),
            n_lit: 2
        };

        assert_eq!(a, b);
    }

    #[test]
    fn day21_pattern2_4() {
        let a = Pattern2 {
            pixels: vec!(true, false, true, false),
            n_lit: 2
        };

        let b = Pattern2 {
            pixels: vec!(false, true, true, false),
            n_lit: 2
        };

        assert_ne!(a, b);
    }

    #[test]
    fn day21_pattern2_fromstring() {
        let x = Pattern2 {
            pixels: vec!(false, true, false, true),
            n_lit: 2
        };

        let y = Pattern2::from_string(".##.");

        assert_eq!(x, y);
    }

    #[test]
    fn day21_pattern3_rot() {
        let a = Pattern3 {
            center: true,
            border: vec!(true, true, false, true, true, true, false, true),
            n_lit: 6
        };

        let b = Pattern3 {
            center: true,
            border: vec!(false, true, true, true, false, true, true, true),
            n_lit: 6
        };

        assert_eq!(a, b);
    }

    #[test]
    fn day21_pattern3_flip() {
        
        let a = Pattern3 {
            center: true,
            border: vec!(true, false, false, true, true, false, false, true),
            n_lit: 6
        };

        let b = Pattern3 {
            center: true,
            border: vec!(false, false, true, true, false, false, true, true),
            n_lit: 6
        };

        assert_eq!(a, b);
    }

    #[test]
    fn day21_pattern3_fromstring() {
        let a = Pattern3 {
            center: true,
            border: vec!(true, true, false, true, false, true, false, false),
            n_lit: 5
        };

        let b = Pattern3::from_string("##..##.#.");

        assert_eq!(a, b);
    }
}