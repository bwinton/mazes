use crate::util::{
    draw_board, Algorithm, Direction, CELL_WIDTH, COLORS, COLUMNS, FIELD_COLOR, LINE_WIDTH, OFFSET,
    ROWS,
};
use maze_utils::From;
use enumset::EnumSet;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Graphics,
    log, Result,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Carving,
    Done,
}

#[derive(From)]
pub struct Exports {
    curr: (usize, usize),
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    harder: bool,
    rng: ThreadRng,
    run_start: usize,
    state: State,
}

impl Exports {
    pub fn new(variant: bool) -> Self {
        let curr = (0, 0);
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let harder = variant;

        let rng = thread_rng();
        let run_start = 0;
        let state = State::Setup;
        Self {
            curr,
            grid,
            harder,
            rng,
            run_start,
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
        if self.harder {
            String::from("Harder Sidewinder")
        } else {
            String::from("Sidewinder")
        }
    }
    fn re_init(&mut self, variant: String) {
        self.from(Exports::new(variant == "hard"));
    }
    fn get_variant(&self) -> String {
        if self.harder {
            "hard".to_owned()
        } else {
            "easy".to_owned()
        }
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                self.curr = (0, 0);
                self.run_start = 0;
                self.state = State::Running;
            }
            State::Done => {}
            State::Running => {
                let proportion = if self.harder {
                    0.4 + (self.curr.0 as f64 / COLUMNS as f64) * 0.4
                } else {
                    0.5
                };
                if (self.rng.gen_bool(proportion) || self.curr.1 == 0)
                    && self.curr.0 < COLUMNS as usize - 1
                {
                    // Carve a path to the east…
                    self.carve(self.curr, Direction::East);
                    self.curr.0 += 1;
                } else {
                    self.state = State::Carving;
                }
            }
            State::Carving => {
                self.curr.0 += 1;
                if self.curr.1 > 0 {
                    let north = self.rng.gen_range(self.run_start, self.curr.0);
                    self.carve((north, self.curr.1), Direction::North);
                    self.run_start = self.curr.0;
                }

                if self.curr.0 == COLUMNS as usize {
                    self.curr = (0, self.curr.1 + 1);
                    self.run_start = 0;
                }
                if self.curr.1 == ROWS as usize {
                    self.state = State::Done;
                    log::info!("Done!");
                    return;
                }

                self.state = State::Running;
            }
        }
    }

    fn draw(&self, gfx: &mut Graphics) -> Result<()> {
        let elements = draw_board(self.grid)?;
        gfx.draw_mesh(&elements);

        let curr_color = COLORS[1];
        let mut cell_color = COLORS[1];
        cell_color.a = 0.5;

        // Draw the field.
        let y = self.curr.1 as f32 + 1.0;
        let rect = Rectangle::new(
            Vector::new(0.0 * CELL_WIDTH + OFFSET, y * CELL_WIDTH + OFFSET),
            Vector::new(COLUMNS * CELL_WIDTH, (ROWS - y) * CELL_WIDTH),
        );
        gfx.fill_rect(&rect, FIELD_COLOR);

        let x = self.curr.0 as f32 + 1.0;
        let y = y - 1.0;
        let rect = Rectangle::new(
            Vector::new(x * CELL_WIDTH + OFFSET, y * CELL_WIDTH + OFFSET),
            Vector::new((COLUMNS - x) * CELL_WIDTH, CELL_WIDTH),
        );
        gfx.fill_rect(&rect, FIELD_COLOR);

        let start = self.run_start as f32;
        let rect = Rectangle::new(
            Vector::new(start * CELL_WIDTH + OFFSET, y * CELL_WIDTH + OFFSET),
            Vector::new((x - start) * CELL_WIDTH, CELL_WIDTH),
        );
        gfx.fill_rect(&rect, cell_color);

        let x = x - 1.0;
        let rect = Rectangle::new(
            Vector::new(
                x as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                y as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
            ),
            Vector::new(CELL_WIDTH - LINE_WIDTH * 2.0, CELL_WIDTH - LINE_WIDTH * 2.0),
        );
        gfx.fill_rect(&rect, curr_color);

        Ok(())
    }
}
