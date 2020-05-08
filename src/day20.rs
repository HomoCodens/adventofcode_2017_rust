use regex::Regex;
use std::collections::HashMap;
use std::cmp::Ordering;

#[derive(Debug)]
struct Particle {
    p: Vec<i64>,
    v: Vec<i64>,
    a: Vec<i64>,
    id: usize
}

#[derive(Debug)]
struct Collision<'a> {
    p1: &'a Particle,
    p2: &'a Particle,
    t: f64
}

#[aoc_generator(day20)]
fn parthe_particles(input: &str) -> Vec<Particle> {
    let re = Regex::new(r"p=<(.+)>, v=<(.+)>, a=<(.+)>").unwrap();

    input.lines().enumerate().map(|(i, l)| {
        let caps = re.captures(l).unwrap();
        let p: Vec<i64> = caps.get(1).unwrap().as_str().split(",").map(|x| x.trim().parse().unwrap()).collect();
        let v: Vec<i64> = caps.get(2).unwrap().as_str().split(",").map(|x| x.trim().parse().unwrap()).collect();
        let a: Vec<i64> = caps.get(3).unwrap().as_str().split(",").map(|x| x.trim().parse().unwrap()).collect();

        Particle{
            p: p,
            v: v,
            a: a,
            id: i
        }
    }).collect()
}

#[aoc(day20, part1)]
fn day20_part1(particles: &Vec<Particle>) -> usize {
    
    let pmin = particles.iter().enumerate().min_by(|(_, p1), (_, p2)| {
        (p1.a[0].abs() + p1.a[1].abs() + p1.a[2].abs()).cmp(&(p2.a[0].abs() + p2.a[1].abs() + p2.a[2].abs()))
    }).unwrap();
    
    pmin.0
}

#[aoc(day20, part2)]
fn day20_part2(particles: &Vec<Particle>) -> usize {
    let mut collisions = vec!();
    let mut graveyard = HashMap::new(); // <id, time of death>

    for i in 0..(particles.len()-1) {
        for j in (i+1)..particles.len() {
            //println!("considering {} and {}", i, j);
            match do_particles_collide(&particles[i], &particles[j]) {
                None => {
                    //println!("note, you good");
                },
                Some(c) => {
                    collisions.push(c);
                    println!("bang {} {}", i, j);
                }
            }
        }
    }

    collisions.sort_by(|a, b| if a.t > b.t { Ordering::Greater } else { Ordering::Less });
    println!("{}", collisions.iter().map(|c| format!("{}, {}, {}", c.p1.id, c.p2.id, c.t)).collect::<Vec<String>>().join("\n"));

    for c in collisions {
        if ((!graveyard.contains_key(&c.p1.id)) || *graveyard.get(&c.p1.id).unwrap() == c.t) &&
            ((!graveyard.contains_key(&c.p2.id)) || *graveyard.get(&c.p2.id).unwrap() == c.t) {
                graveyard.insert(c.p1.id, c.t);
                graveyard.insert(c.p2.id, c.t);
            } else {
                //println!("oh, a double collision!");
            }
    }
    particles.len() - graveyard.len()
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    if a == 0.0 {
        //println!("solving {}t + {} = 0", b, c);
        return Some((-c/b, -c/b));
    } else {
        let d = b*b - 4.0*a*c;
        //println!("solving {}t^2 + {}t + {} = 0, d = {}", a, b, c, d);
        if d < 0.0 {
            return None;
        } else if d == 0.0 {
            //println!("double solushian!");
            return Some((-b/(2.0*a), -b/(2.0*a)));
        } else {
            //println!("Two solushians...");
            return Some(((-b + d.sqrt())/(2.0*a), (-b - d.sqrt())/(2.0*a)));
        }
    }
}

fn solve_for_dim(p1: &Particle, p2: &Particle, dim: usize) -> Option<(f64, f64)> {
    let a = (p1.a[dim] as f64 - p2.a[dim] as f64)/2.0;
    let b = p1.v[dim] as f64 - p2.v[dim] as f64;
    let c = p1.p[dim] as f64 - p2.p[dim] as f64;

    solve_quadratic(a, b, c)
}

fn do_particles_collide<'a>(p1: &'a Particle, p2: &'a Particle) -> Option<Collision<'a>> {
    match solve_for_dim(p1, p2, 0) {
        None => {
            //println!("No solushian found");
            return None;
        },
        Some((s1x, s2x)) => {
            //println!("got a solution for x: {}, {}", s1x, s2x);
            match solve_for_dim(p1, p2, 1) {
                None => {
                    return None;
                },
                Some((s1y, s2y)) => {
                    //println!("got a solution for y: {}, {}", s1y, s2y);
                    if s1x == s1y || s1x == s2y || s2x == s2y {
                        match solve_for_dim(p1, p2, 2) {
                            None => {
                                return None;
                            },
                            Some((s1z, s2z)) => {
                                println!("got a solution for all 3 dims");
                                let s1_valid = s1x > 0.0 &&
                                                s1y % 1.0 == 0.0 &&
                                                ((s1x == s1y && s1x == s1z) ||
                                                (s1x == s2y && s1x == s1z) ||
                                                (s1x == s1y && s1x == s2z) ||
                                                (s1x == s2y && s1x == s2z));
                                let s2_valid = s2x > 0.0 &&
                                                s2x % 1.0 == 0.0 &&
                                                ((s2x == s1y && s2x == s1z) ||
                                                (s2x == s2y && s2x == s1z) ||
                                                (s2x == s1y && s2x == s2z) ||
                                                (s2x == s2y && s2x == s2z));

                                if s1_valid && s2_valid {
                                    if s1x > s2x {
                                        return Some(Collision{p1: p1, p2: p2, t: s2x});
                                    } else {
                                        return Some(Collision{p1: p1, p2: p2, t: s1x});
                                    }
                                } else if s1_valid {
                                    return Some(Collision{p1: p1, p2: p2, t: s1x});
                                } else if s2_valid {
                                    return Some(Collision{p1: p1, p2: p2, t: s2x});
                                } else {
                                    return None;
                                }
                            }
                        }
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn day20ex1() {
        assert_eq!(day20_part1(&parthe_particles(&"p=< 3,0,0>, v=< 2,0,0>, a=<-1,0,0>
        p=< 4,0,0>, v=< 0,0,0>, a=<-2,0,0>")), 0);
    }

    #[test]
    fn day20ex2() {
        assert_eq!(day20_part2(&parthe_particles(&"p=<-6,0,0>, v=< 3,0,0>, a=< 0,0,0>    
        p=<-4,0,0>, v=< 2,0,0>, a=< 0,0,0>
        p=<-2,0,0>, v=< 1,0,0>, a=< 0,0,0>
        p=< 3,0,0>, v=<-1,0,0>, a=< 0,0,0>")), 1);
    }
}