// Er. Mah. Gerd. This took me like 2 years to solve... ;P
// Also: Coming back into a typed language is harder than I thought.
// What's up with not being able to subtract integers from floats though?!

use std::collections::HashMap;

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> f64 {
    input.trim()
    .parse()
    .expect("Oh noes, please check input!")
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn get_coordinates(number: &f64) -> Point {

    if *number == 1.0 {
        return Point {
            x: 0,
            y: 0
        };
    }

    let layer = number.sqrt().ceil();

    // How far along the layer we are (1 based)
    let offset = 2.0*(layer - 1.0) + 1.0 + (number - layer*layer);

    //println!("n: {}, layer: {}, offset: {}", number, layer, offset);

    let x_offset;
    let y_offset;

    if (layer as u32 % 2) == 0 {
        // We are on the right or top edge
        if offset <= layer {
            // right (including corner)
            x_offset = layer/2.0;
            y_offset = -layer/2.0 + offset;
        } else {
            // top excluding corner
            y_offset = layer/2.0;
            x_offset = layer/2.0 - (offset - layer);
        }
    } else {
        // We are on the left or bottom edge
        if offset < layer {
            // left (w/o corner)
            x_offset = -(layer/2.0).floor();
            y_offset = (layer/2.0).floor() - (offset - 1.0);
        } else {
            // bottom (including corner)
            y_offset = -(layer/2.0).floor();
            x_offset = -(layer/2.0).floor() + (offset - layer);
        }
    }

    Point {
        x: x_offset as i32,
        y: y_offset as i32
    }
}

#[aoc(day3, part1)]
pub fn day3_part1(number: &f64) -> i32 {
    let p = get_coordinates(number);

    p.x.abs() + p.y.abs()
}

#[aoc(day3, part2)]
pub fn day3_part2(number: &f64) -> i32 {
    let mut points_visited = HashMap::new();

    let number = *number as i32;

    let mut cnt = 2.0;
    let mut current = 0;

    points_visited.insert(0, 1);

    while current < number {
        let p = get_coordinates(&cnt);

        current = 0;

        for dx in -1..2 {
            for dy in -1..2 {
                if dx == dy && dx == 0 {
                    continue;
                }

                let coord = 10000*(p.x + dx) + (p.y + dy);
                if points_visited.contains_key(&coord) {
                    current += points_visited.get(&coord).unwrap();
                }
            }
        }

        //println!("x: {}, y: {}, value: {}", p.x, p.y, current);
        
        points_visited.insert(10000*p.x + p.y, current);

        cnt += 1.0;
    }
    
    //println!("{:?}", points_visited);
    println!("It took us {} steps to get to {}.", cnt, number);
    current
}

#[cfg(test)]
mod tests {
    use super::{day3_part1, get_coordinates, day3_part2};

    #[test]
    fn example1() {
        assert_eq!(day3_part1(&1.0), 0)
    }

    #[test]
    fn example2() {
        assert_eq!(day3_part1(&12.0), 3)
    }
    
    #[test]
    fn example3() {
        assert_eq!(day3_part1(&23.0), 2);
    }

    #[test]
    fn example4() {
        assert_eq!(day3_part1(&1024.0), 31);
    }

    #[test]
    fn sanity3() {
        let aa = get_coordinates(&3.0);
        assert_eq!(aa.x, 1, "x matches");
        assert_eq!(aa.y, 1, "y matches");
    }

    #[test]
    fn sanity4() {
        let aa = get_coordinates(&4.0);
        assert_eq!(aa.x, 0, "x matches");
        assert_eq!(aa.y, 1, "y matches");
    }

    #[test]
    fn sanity5() {
        let aa = get_coordinates(&5.0);
        assert_eq!(aa.x, -1, "x matches");
        assert_eq!(aa.y, 1, "y matches");
    }

    #[test]
    fn sanity7() {
        let aa = get_coordinates(&7.0);
        assert_eq!(aa.x, -1, "x matches");
        assert_eq!(aa.y, -1, "y matches");
    }

    #[test]
    fn sanity8() {
        let aa = get_coordinates(&8.0);
        assert_eq!(aa.x, 0, "x matches");
        assert_eq!(aa.y, -1, "y matches");
    }

    #[test]
    fn sanity9() {
        let aa = get_coordinates(&9.0);
        assert_eq!(aa.x, 1, "x matches");
        assert_eq!(aa.y, -1, "y matches");
    }

    #[test]
    fn sanity16() {
        let aa = get_coordinates(&16.0);
        assert_eq!(aa.x, -1, "x matches");
        assert_eq!(aa.y, 2, "y matches");
    }

    #[test]
    fn sanity20() {
        let aa = get_coordinates(&20.0);
        assert_eq!(aa.x, -2, "x matches");
        assert_eq!(aa.y, -1, "y matches");
    }

    #[test]
    fn sanity21() {
        let aa = get_coordinates(&21.0);
        assert_eq!(aa.x, -2, "x matches");
        assert_eq!(aa.y, -2, "y matches");
    }

    #[test]
    fn part2_1() {
        assert_eq!(day3_part2(&58.0), 59)
    }
}