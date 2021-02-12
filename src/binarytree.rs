use crate::util::{
    draw_board, Algorithm, ChooseRandom, Direction, CELL_WIDTH, COLORS, COLUMNS, FIELD_COLOR,
    OFFSET, ROWS,
};
use derive_more::Display;
use enumset::EnumSet;
use itertools::Itertools;
use macroquad::{logging as log, prelude::draw_rectangle, rand::gen_range};
use maze_utils::From;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}

#[derive(Display)]
enum Bias {
    NorthEast,
    SouthEast,
    SouthWest,
    NorthWest,
}

#[derive(From)]
pub struct Exports {
    bias: Bias,
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    random: bool,
    remaining: Vec<(usize, usize)>,
    state: State,
}

impl Exports {
    pub fn new(variant: String) -> Self {
        let mut args = variant.splitn(2, ':');
        let random = args.next().unwrap_or("random") == "random";
        let bias = args.next().unwrap_or("NorthWest");

        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let mut remaining: Vec<(usize, usize)> = (0..ROWS as usize)
            .cartesian_product(0..COLUMNS as usize)
            .map(|(y, x)| (x, y))
            .collect();
        if random {
            remaining.shuffle();
        } else {
            remaining.reverse();
        }
        let state = State::Setup;
        let bias = match bias {
            "NorthEast" => Bias::NorthEast,
            "SouthEast" => Bias::SouthEast,
            "SouthWest" => Bias::SouthWest,
            "NorthWest" => Bias::NorthWest,
            _ => panic!("Unknown bias {}", bias),
        };
        Self {
            bias,
            grid,
            random,
            remaining,
            state,
        }
    }
    fn carve(&mut self, cell: (usize, usize), direction: Direction) {
        let (x, y) = cell;
        let (new_x, new_y) = match direction {
            Direction::North => (x as i32, y as i32 - 1),
            Direction::East => (x as i32 + 1, y as i32),
            Direction::South => (x as i32, y as i32 + 1),
            Direction::West => (x as i32 - 1, y as i32),
        };
        self.grid[y][x] |= direction;
        self.grid[new_y as usize][new_x as usize] |= direction.opposite();
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Binary Tree")
    }
    fn re_init(&mut self, variant: String) {
        self.from(Exports::new(variant));
    }
    fn get_variant(&self) -> String {
        let rv = if self.random { "random" } else { "ordered" };
        format!("{}:{}", rv, self.bias)
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                self.state = State::Running;
                return;
            }
            State::Done => {
                return;
            }
            _ => {}
        }

        let mut found = false;

        while !found {
            if self.remaining.is_empty() {
                self.state = State::Done;
                log::info!("Done!");
                return;
            }
            let curr = self.remaining.pop().unwrap();
            // log::info!("{:?}", curr);
            match self.bias {
                Bias::NorthEast => {
                    if curr.0 == COLUMNS as usize - 1 && curr.1 == 0 {
                        return;
                    }
                    let direction = if curr.0 == COLUMNS as usize - 1 {
                        Direction::North
                    } else if curr.1 == 0 {
                        Direction::East
                    } else if gen_range(0, 2) == 0 {
                        Direction::North
                    } else {
                        Direction::East
                    };
                    self.carve(curr, direction);
                }
                Bias::SouthEast => {
                    if curr.0 == COLUMNS as usize - 1 && curr.1 == ROWS as usize - 1 {
                        return;
                    }
                    let direction = if curr.0 == COLUMNS as usize - 1 {
                        Direction::South
                    } else if curr.1 == ROWS as usize - 1 {
                        Direction::East
                    } else if gen_range(0, 2) == 0 {
                        Direction::South
                    } else {
                        Direction::East
                    };
                    self.carve(curr, direction);
                }
                Bias::SouthWest => {
                    if curr.0 == 0 && curr.1 == ROWS as usize - 1 {
                        return;
                    }
                    let direction = if curr.0 == 0 {
                        Direction::South
                    } else if curr.1 == ROWS as usize - 1 {
                        Direction::West
                    } else if gen_range(0, 2) == 0 {
                        Direction::South
                    } else {
                        Direction::West
                    };
                    self.carve(curr, direction);
                }
                Bias::NorthWest => {
                    if curr.0 == 0 && curr.1 == 0 {
                        return;
                    }
                    let direction = if curr.0 == 0 {
                        Direction::North
                    } else if curr.1 == 0 {
                        Direction::West
                    } else if gen_range(0, 2) == 0 {
                        Direction::North
                    } else {
                        Direction::West
                    };
                    self.carve(curr, direction);
                }
            }

            found = true;
        }
    }

    fn draw(&self) {
        draw_board(self.grid);

        if self.state == State::Running {
            let mut curr_color = COLORS[1];
            curr_color.a = 0.3;
            for x in 0..COLUMNS as usize {
                for y in 0..ROWS as usize {
                    if self.grid[y][x] == EnumSet::new() {
                        draw_rectangle(
                            x as f32 * CELL_WIDTH + OFFSET,
                            y as f32 * CELL_WIDTH + OFFSET,
                            CELL_WIDTH,
                            CELL_WIDTH,
                            FIELD_COLOR,
                        );
                    } else if self.remaining.contains(&(x, y)) {
                        draw_rectangle(
                            x as f32 * CELL_WIDTH + OFFSET,
                            y as f32 * CELL_WIDTH + OFFSET,
                            CELL_WIDTH,
                            CELL_WIDTH,
                            curr_color,
                        );
                    }
                }
            }
        }
    }
}
