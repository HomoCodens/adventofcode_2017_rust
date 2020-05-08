use regex::Regex;

#[derive(Debug)]
struct Particle {
    p: Vec<i64>,
    v: Vec<i64>,
    a: Vec<i64>,
    id: usize,
    tod: i32
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
            id: i,
            tod: -1
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
    let mut particles: Vec<Particle> = particles.iter().map(|p| {
        Particle{
            p: p.p.clone(),
            v: p.v.clone(),
            a: p.a.clone(),
            id: p.id,
            tod: -1
        }
    }).collect();

    for step in 0..4000 {
        for p in 0..particles.len() {
            if particles[p].tod == -1 {
                for i in 0..3 {
                    particles[p].v[i] += particles[p].a[i];
                    particles[p].p[i] += particles[p].v[i];
                }
            }
        }


        for p1 in 0..(particles.len() - 1) {
            if particles[p1].tod == -1 || particles[p1].tod == step {
                for p2 in (p1+1)..particles.len() {
                    if (particles[p2].tod == -1 || particles[p2].tod == step) &&
                    particles[p1].p[0] == particles[p2].p[0] &&
                    particles[p1].p[1] == particles[p2].p[1] && 
                    particles[p1].p[2] == particles[p2].p[2] {
                        particles[p1].tod = step;
                        particles[p2].tod = step;
                    }
                }
            }
        }
    }

    particles.iter().filter(|p| p.tod == -1).count()
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