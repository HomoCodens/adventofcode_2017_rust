enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    cells: Vec<Vec<bool>>,
    offset_x: usize,
    offset_y: usize
}

impl Grid {
    fn from_string(string: &str) -> Grid {
        let cells: Vec<Vec<bool>> = string.lines()
                    .map(|l| {
                        l.chars().map(|c| { c == '#' }).collect::<Vec<bool>>()
                    }).collect();
        let ox = &cells[0].len() / 2;
        let oy = &cells.len() / 2;
        Grid {
            cells: cells,
            offset_x: ox,
            offset_y: oy
        }
    }

    fn grow(&mut self, dir: Direction) {
        let xd = self.cells[0].len();
        let yd = self.cells.len();
        let grwth = vec![false; xd];
        match dir  {
            Direction::North => {
                let mut north = vec!();
                for _i in 0..yd {
                    north.push(grwth.clone());
                }
                north.append(&mut self.cells);
                self.cells = north;
                self.offset_y += yd;
            },
            Direction::South => {
                let mut south = vec!();
                for _i in 0..yd {
                    south.push(grwth.clone());
                }
                self.cells.append(&mut south);
            },
            Direction::East => {
                for i in 0..yd {
                    self.cells[i].append(&mut grwth.clone());
                }
            },
            Direction::West => {
                for i in 0..yd {
                    let mut nn = grwth.clone();
                    nn.append(&mut self.cells[i]);
                    self.cells[i] = nn;
                }
                self.offset_x += xd;
            }
        };
    }
}

enum Turn {
    Left,
    Right
}

#[derive(PartialEq, Eq, Debug)]
struct Bot {
    x: i32,
    y: i32,
    facing: (i32, i32)
}

impl Bot {
    fn turn(&mut self, dir: Turn) {
        self.facing = match dir {
            Turn::Left => (self.facing.1, -self.facing.0),
            Turn::Right => (-self.facing.1, self.facing.0)
        }
    }

    fn step(&mut self) {
        self.x += self.facing.0;
        self.y += self.facing.1;
    }

    fn new() -> Bot {
        Bot {
            x: 0,
            y: 0,
            facing: (0, -1)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    bot: Bot,
    grid: Grid
}

impl State {
    fn from_string(string: &str) -> State {
        State {
            bot: Bot::new(),
            grid: Grid::from_string(string)
        }
    }

    fn bot_pos(&self) -> (usize, usize) {
        let bot_x = (self.bot.x + (self.grid.offset_x as i32)) as usize;
        let bot_y = (self.bot.y + (self.grid.offset_y as i32)) as usize;
        (bot_x, bot_y)
    }

    fn burst(&mut self) -> bool {
        let (bot_x, bot_y) = self.bot_pos();
        let is_infected = self.grid.cells[bot_y][bot_x]; // If you're so smart, why don't you "add `;` here" yourself?
        if  is_infected {
            self.bot.turn(Turn::Right);
        } else {
            self.bot.turn(Turn::Left);
        }

        self.grid.cells[bot_y][bot_x] = !is_infected;

        if bot_y == 0 && self.bot.facing.1 == -1 {
            self.grid.grow(Direction::North);
        } else if bot_y == self.grid.cells.len() - 1 && self.bot.facing.1 == 1 {
            self.grid.grow(Direction::South);
        } else if bot_x == 0 && self.bot.facing.0 == -1 {
            self.grid.grow(Direction::West);
        } else if bot_x == self.grid.cells[0].len() - 1 && self.bot.facing.0 == 1 {
            self.grid.grow(Direction::East);
        }

        self.bot.step();

        !is_infected
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = self.grid.cells.iter().map(|r| {
            r.iter().map(|c| if *c { String::from(" # ") } else { String::from(" . ") }).collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>();

        let (bx, by) = self.bot_pos();
        let bot = if self.bot.facing.0 == 1 {
            String::from(">")
        } else if self.bot.facing.0 == -1 {
            String::from("<")
        } else if self.bot.facing.1 == 1 {
            String::from("v")
        } else {
            String::from("^")
        };

        let bot_cell = if self.grid.cells[by][bx] {
            format!("[{}]", bot)
        } else {
            format!(" {} ", bot)
        };

        lines[by][bx] = bot_cell;

        write!(f, "{}", lines.iter().map(|l| l.join("")).collect::<Vec<String>>().join("\n"))
    }
}

#[aoc(day22, part1)]
fn day22_part1(input: &str) -> i32 {
    let mut state = State::from_string(input);
    let mut cnt = 0;

    for _i in 0..10000 {
        cnt += state.burst() as i32;
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn day22_bot_new() {
        let b = Bot::new();
        assert_eq!(b, Bot{x: 0, y: 0, facing: (0, -1)})
    }

    #[test]
    fn day22_bot_turl_left() {
        let mut b = Bot{
            x: 0, y: 0, facing: (0, -1)
        };

        b.turn(Turn::Left);
        assert_eq!(b.facing, (-1, 0));

        b.turn(Turn::Left);
        assert_eq!(b.facing, (0, 1));

        b.turn(Turn::Left);
        assert_eq!(b.facing, (1, 0));
    }

    #[test]
    fn day22_bot_turl_right() {
        let mut b = Bot{
            x: 0, y: 0, facing: (0, -1)
        };

        b.turn(Turn::Right);
        assert_eq!(b.facing, (1, 0));

        b.turn(Turn::Right);
        assert_eq!(b.facing, (0, 1));

        b.turn(Turn::Right);
        assert_eq!(b.facing, (-1, 0));
    }

    #[test]
    fn day22_bot_step() {
        let mut b = Bot {
            x: 0,
            y: 0,
            facing: (0, -1)
        };

        b.step();

        assert_eq!(b, Bot{x: 0, y: -1, facing: (0, -1)});
    }

    #[test]
    fn day22_grid_from_string() {
        let g = Grid::from_string("..#\n#..\n...");

        assert_eq!(g, Grid{ 
            cells: vec!(
                vec!(false, false, true),
                vec!(true, false, false),
                vec!(false, false, false)),
            offset_x: 1,
            offset_y: 1
        });
    }

    #[test]
    fn day22_grid_grow_north() {
        let mut g = Grid {
            cells: vec!(
                vec!(false, false, false),
                vec!(false, true, false),
                vec!(false, false, false)
            ),
            offset_x: 1,
            offset_y: 1
        };

        g.grow(Direction::North);

        assert_eq!(g, Grid {
            cells: vec!(
                vec!(false, false, false),
                vec!(false, false, false),
                vec!(false, false, false),
                vec!(false, false, false),
                vec!(false, true, false),
                vec!(false, false, false),
            ),
            offset_x: 1,
            offset_y: 4
        });
    }

    #[test]
    fn day22_grid_grow_south() {
        let mut g = Grid {
            cells: vec!(
                vec!(false, false, false),
                vec!(false, true, false),
                vec!(false, false, false)
            ),
            offset_x: 1,
            offset_y: 1
        };

        g.grow(Direction::South);

        assert_eq!(g, Grid {
            cells: vec!(
                vec!(false, false, false),
                vec!(false, true, false),
                vec!(false, false, false),
                vec!(false, false, false),
                vec!(false, false, false),
                vec!(false, false, false),
            ),
            offset_x: 1,
            offset_y: 1
        });
    }

    #[test]
    fn day22_grid_grow_east() {
        let mut g = Grid {
            cells: vec!(
                vec!(false, false, false),
                vec!(false, true, false),
                vec!(false, false, false)
            ),
            offset_x: 1,
            offset_y: 1
        };

        g.grow(Direction::East);

        assert_eq!(g, Grid {
            cells: vec!(
                vec!(false, false, false, false, false, false),
                vec!(false, true, false, false, false, false),
                vec!(false, false, false, false, false, false)
            ),
            offset_x: 1,
            offset_y: 1
        });
    }

    #[test]
    fn day22_grid_grow_west() {
        let mut g = Grid {
            cells: vec!(
                vec!(false, false, false),
                vec!(false, true, false),
                vec!(false, false, false)
            ),
            offset_x: 1,
            offset_y: 1
        };

        g.grow(Direction::West);

        assert_eq!(g, Grid {
            cells: vec!(
                vec!(false, false, false, false, false, false),
                vec!(false, false, false, false, true, false),
                vec!(false, false, false, false, false, false)
            ),
            offset_x: 4,
            offset_y: 1
        });
    }

    #[test]
    fn day22_state_from_string() {
        let s = State::from_string("...\n#.#\n.#.");

        assert_eq!(s, State {
            bot: Bot { x: 0, y: 0, facing: (0, -1)},
            grid: Grid {
                cells: vec!(
                    vec!(false, false, false),
                    vec!(true, false, true),
                    vec!(false, true, false),
                ),
                offset_x: 1,
                offset_y: 1
            }
        });
    }

    #[test]
    fn state_burst_clean() {
        let mut s = State {
            grid: Grid {
                cells: vec!(
                    vec!(false, false, false),
                    vec!(false, false, false),
                    vec!(false, false, false),
                ),
                offset_x: 1,
                offset_y: 1
            },
            bot: Bot {
                x: 0,
                y: 0,
                facing: (0, -1)
            }
        };

        let out = s.burst();

        assert_eq!(s, State {
            grid: Grid {
                cells: vec!(
                    vec!(false, false, false),
                    vec!(false, true, false),
                    vec!(false, false, false),
                ),
                offset_x: 1,
                offset_y: 1
            },
            bot: Bot {
                x: -1,
                y: 0,
                facing: (-1, 0)
            }
        });

        assert_eq!(out, true);
    }

    #[test]
    fn state_burst_infected() {
        let mut s = State {
            grid: Grid {
                cells: vec!(
                    vec!(false, false, false),
                    vec!(false, true, false),
                    vec!(false, false, false),
                ),
                offset_x: 1,
                offset_y: 1
            },
            bot: Bot {
                x: 0,
                y: 0,
                facing: (0, -1)
            }
        };

        let out = s.burst();

        assert_eq!(s, State {
            grid: Grid {
                cells: vec!(
                    vec!(false, false, false),
                    vec!(false, false, false),
                    vec!(false, false, false),
                ),
                offset_x: 1,
                offset_y: 1
            },
            bot: Bot {
                x: 1,
                y: 0,
                facing: (1, 0)
            }
        });

        assert_eq!(out, false);
    }

    #[test]
    fn day22_ex1_evolution() {
        let mut s = State {
            grid: Grid {
                cells: vec!(
                    vec!(false, false, true),
                    vec!(true, false, false),
                    vec!(false, false, false)
                ),
                offset_x: 1,
                offset_y: 1
            },
            bot: Bot { x: 0, y: 0, facing: (0, -1) }
        };

        for _i in 0..70 {
            s.burst();
        }

        assert_eq!(s, State {
            grid: Grid {
                cells: vec!(
                    vec!(false, false, false, false, false, true, true, false, false, false, false, false),
                    vec!(false, false, false, false, true, false, false, true, false, false, false, false),
                    vec!(false, false, false, true, false, false, false, false, true, false, false, false),
                    vec!(false, false, true, false, true, false, false, false, true, false, false, false),
                    vec!(false, false, true, false, true, false, false, true, false, false, false, false),
                    vec!(false, false, false, false, false, true, true, false, false, false, false, false)
                ),
                offset_x: 4,
                offset_y: 4
            },
            bot: Bot {x: 1, y: -1, facing: (0, -1)}
        });
    }

    #[test]
    fn day22_ex1_solution() {
        assert_eq!(day22_part1(&"..#\n#..\n..."), 5587);
    }
}