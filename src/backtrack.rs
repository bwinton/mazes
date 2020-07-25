use crate::util::{
    draw_board, Algorithm, Direction, CELL_WIDTH, COLORS, COLUMNS, LINE_WIDTH, ROWS,
};

use std::collections::VecDeque;

use enumset::EnumSet;
use quicksilver::{log, Result,
    geom::{Rectangle, Vector},
    graphics::Graphics,
};
use rand::{thread_rng, Rng,
    rngs::ThreadRng,
    seq::SliceRandom,
};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}

pub struct Exports {
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    rng: ThreadRng,
    stack: VecDeque<(usize, usize, EnumSet<Direction>)>,
    state: State,
}

impl Exports {
    pub fn new() -> Self {
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let rng = thread_rng();
        let stack = VecDeque::new();
        let state = State::Setup;
        Self {
            grid,
            rng,
            stack,
            state,
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Backtrack")
    }
    fn update(&mut self) {
        // println!("Updating {}", self.name());
        match self.state {
            State::Setup => {
                self.stack.push_front((
                    self.rng.gen_range(0, COLUMNS as usize),
                    self.rng.gen_range(0, ROWS as usize),
                    EnumSet::all(),
                ));
                self.state = State::Running;
                return;
            }
            State::Done => {
                return;
            }
            _ => {}
        }

        let mut found = false;
        // let (first_x, first_y, _) = self.stack.front().unwrap().clone();

        while !found {
            if self.stack.is_empty() {
                self.state = State::Done;
                log::info!("Done!");
                return;
            }

            let (x, y, directions) = self.stack.pop_front().unwrap();
            let mut potentials: Vec<Direction> = directions.iter().collect();
            if potentials.is_empty() {
                return;
            }
            potentials.shuffle(&mut self.rng);
            let direction = potentials.pop().unwrap();
            // println!("({},{}) -> {:?}", x, y, direction);
            self.stack.push_front((x, y, directions ^ direction));

            let (new_x, new_y) = match direction {
                Direction::North => (x as i32, y as i32 - 1),
                Direction::East => (x as i32 + 1, y as i32),
                Direction::South => (x as i32, y as i32 + 1),
                Direction::West => (x as i32 - 1, y as i32),
            };
            // println!("{:?} / {:?} -> {:?}", (x,y), direction, (new_x, new_y));
            if 0 <= new_x && new_x < COLUMNS as i32 && 0 <= new_y && new_y < ROWS as i32 {
                let (new_x, new_y) = (new_x as usize, new_y as usize);
                if self.grid[new_y][new_x] == EnumSet::new() {
                    self.grid[y][x] |= direction;
                    self.grid[new_y][new_x] |= direction.opposite();
                    self.stack
                        .push_front((new_x, new_y, EnumSet::all() ^ direction.opposite()));
                    found = true;
                }
                // Otherwise, loop again and see what we can get.
            }
            // if potentials.is_empty() {
            //     return;
            // }
        }
    }

    fn draw(&self, gfx: &mut Graphics) -> Result<()> {
        // Draw code here...
        let elements = draw_board(self.grid)?;
        gfx.draw_mesh(&elements);

        let curr_color = COLORS[1];
        let mut cell_color = COLORS[1];
        cell_color.a = 0.5;
        for (i, (x, y, _)) in self.stack.iter().enumerate() {
            if i == 0 {
                let rect = Rectangle::new(Vector::new(*x as f32 * CELL_WIDTH + LINE_WIDTH, *y as f32 * CELL_WIDTH + LINE_WIDTH),
                Vector::new(CELL_WIDTH - LINE_WIDTH * 2.0, CELL_WIDTH - LINE_WIDTH * 2.0));

                gfx.fill_rect(&rect, curr_color);
            } else {
                let rect = Rectangle::new(Vector::new(*x as f32 * CELL_WIDTH, *y as f32 * CELL_WIDTH),
                 Vector::new(CELL_WIDTH, CELL_WIDTH));
                gfx.fill_rect(&rect, cell_color);
            }
        }

        Ok(())
    }
}
