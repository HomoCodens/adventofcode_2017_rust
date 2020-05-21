enum Direction {
    North,
    East,
    South,
    West
}

// Jahjah, it's modulo math
#[derive(Debug, Eq, PartialEq, Clone)]
enum CellState {
    Clear,  // Say hello to my friend Tom C.
    Weakened,
    Infected,
    Flagged
}

impl std::fmt::Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let oot = match self {
            CellState::Clear => " . ",
            CellState::Weakened => " W ",
            CellState::Infected => " # ",
            CellState::Flagged => " F "
        };
        write!(f, "{}", oot)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Grid {
    cells: Vec<Vec<CellState>>,
    offset_x: usize,
    offset_y: usize
}

impl Grid {
    fn from_string(string: &str) -> Grid {
        let cells: Vec<Vec<CellState>> = string.lines()
                    .map(|l| {
                        l.chars().map(|c| if c == '#' { CellState::Infected } else { CellState::Clear }).collect::<Vec<CellState>>()
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
        let grwth = vec![CellState::Clear; xd];
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
    Right,
    Around
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
            Turn::Right => (-self.facing.1, self.facing.0),
            Turn::Around => (-self.facing.0, -self.facing.1)
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

impl std::fmt::Display for Bot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let me = if self.facing.0 == 1 {
            String::from(">")
        } else if self.facing.0 == -1 {
            String::from("<")
        } else if self.facing.1 == 1 {
            String::from("v")
        } else {
            String::from("^")
        };

        write!(f, "{}", me)
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

    fn burst(&mut self, advanced_virus: bool) -> bool {
        let (bot_x, bot_y) = self.bot_pos();
        let cell_state = &self.grid.cells[bot_y][bot_x]; // If you're so smart, why don't you "add `;` here" yourself?
        match cell_state {
            CellState::Clear => self.bot.turn(Turn::Left),
            CellState::Weakened => {},
            CellState::Infected => self.bot.turn(Turn::Right),
            CellState::Flagged => self.bot.turn(Turn::Around)
        }

        let next_state = if advanced_virus {
            match cell_state {
                CellState::Clear => CellState::Weakened,
                CellState::Weakened => CellState::Infected,
                CellState::Infected => CellState::Flagged,
                CellState::Flagged => CellState::Clear
            }
        } else {
            match cell_state {
                CellState::Clear => CellState::Infected,
                CellState::Weakened => CellState::Clear,
                CellState::Infected => CellState::Clear,
                CellState::Flagged => CellState::Clear
            }
        };

        let infection_occurred = next_state == CellState::Infected;
        self.grid.cells[bot_y][bot_x] = next_state;

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

        infection_occurred
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = self.grid.cells.iter().map(|r| {
            r.iter().map(|c| c.to_string()).collect::<Vec<String>>()
        }).collect::<Vec<Vec<String>>>();

        let (bx, by) = self.bot_pos();
        let bot = self.bot.to_string();

        let bot_cell = match self.grid.cells[by][bx] {
            CellState::Clear => format!(" {} ", bot),
            CellState::Weakened => format!("({})", bot),
            CellState::Infected => format!("[{}]", bot),
            CellState::Flagged => format!("{{{}}}", bot) // ya, sure
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
        cnt += state.burst(false) as i32;
    }

    cnt
}

#[aoc(day22, part2)]
fn day22_part2(input: &str) -> i32 {
    let mut state = State::from_string(input);
    let mut cnt = 0;

    for _i in 0..10000000 {
        cnt += state.burst(true) as i32;
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
                vec!(CellState::Clear, CellState::Clear, CellState::Infected),
                vec!(CellState::Infected, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear)),
            offset_x: 1,
            offset_y: 1
        });
    }

    #[test]
    fn day22_grid_grow_north() {
        let mut g = Grid {
            cells: vec!(
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Infected, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear)
            ),
            offset_x: 1,
            offset_y: 1
        };

        g.grow(Direction::North);

        assert_eq!(g, Grid {
            cells: vec!(
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Infected, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
            ),
            offset_x: 1,
            offset_y: 4
        });
    }

    #[test]
    fn day22_grid_grow_south() {
        let mut g = Grid {
            cells: vec!(
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Infected, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear)
            ),
            offset_x: 1,
            offset_y: 1
        };

        g.grow(Direction::South);

        assert_eq!(g, Grid {
            cells: vec!(
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Infected, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
            ),
            offset_x: 1,
            offset_y: 1
        });
    }

    #[test]
    fn day22_grid_grow_east() {
        let mut g = Grid {
            cells: vec!(
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Infected, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear)
            ),
            offset_x: 1,
            offset_y: 1
        };

        g.grow(Direction::East);

        assert_eq!(g, Grid {
            cells: vec!(
                vec!(CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Infected, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear)
            ),
            offset_x: 1,
            offset_y: 1
        });
    }

    #[test]
    fn day22_grid_grow_west() {
        let mut g = Grid {
            cells: vec!(
                vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Infected, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear)
            ),
            offset_x: 1,
            offset_y: 1
        };

        g.grow(Direction::West);

        assert_eq!(g, Grid {
            cells: vec!(
                vec!(CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Infected, CellState::Clear),
                vec!(CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear)
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
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Infected, CellState::Clear, CellState::Infected),
                    vec!(CellState::Clear, CellState::Infected, CellState::Clear),
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
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear),
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

        let out = s.burst(false);

        assert_eq!(s, State {
            grid: Grid {
                cells: vec!(
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Infected, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear),
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
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Infected, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear),
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

        let out = s.burst(false);

        assert_eq!(s, State {
            grid: Grid {
                cells: vec!(
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear),
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
                    vec!(CellState::Clear, CellState::Clear, CellState::Infected),
                    vec!(CellState::Infected, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear)
                ),
                offset_x: 1,
                offset_y: 1
            },
            bot: Bot { x: 0, y: 0, facing: (0, -1) }
        };

        for _i in 0..70 {
            s.burst(false);
        }

        assert_eq!(s, State {
            grid: Grid {
                cells: vec!(
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Infected, CellState::Infected, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Infected, CellState::Clear, CellState::Clear, CellState::Infected, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear, CellState::Infected, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Infected, CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Infected, CellState::Clear, CellState::Infected, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Infected, CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Infected, CellState::Clear, CellState::Infected, CellState::Clear, CellState::Clear, CellState::Infected, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Infected, CellState::Infected, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear, CellState::Clear)
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

    #[test]
    fn day22_ex1_part_2_count() {
        let mut s = State {
            grid: Grid {
                cells: vec!(
                    vec!(CellState::Clear, CellState::Clear, CellState::Infected),
                    vec!(CellState::Infected, CellState::Clear, CellState::Clear),
                    vec!(CellState::Clear, CellState::Clear, CellState::Clear)
                ),
                offset_x: 1,
                offset_y: 1
            },
            bot: Bot { x: 0, y: 0, facing: (0, -1) }
        };


        let mut cnt = 0;
        for _i in 0..100{
            cnt += s.burst(true) as usize;
        }

        assert_eq!(cnt, 26);
    }

    #[test]
    fn day22_ex1_p2_solution() {
        assert_eq!(day22_part2(&"..#\n#..\n..."), 2511944);
    }
}