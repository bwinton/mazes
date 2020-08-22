use crate::util::{
    draw_board, Algorithm, Direction, CELL_WIDTH, COLORS, COLUMNS, LINE_WIDTH, OFFSET, ROWS,
};
use enumset::EnumSet;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Graphics,
    log, Result,
};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}

pub struct Exports {
    curr: (usize, usize),
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    prev: (usize, usize),
    remaining: usize,
    rng: ThreadRng,
    speedup: bool,
    state: State,
}

impl Exports {
    pub fn new(variant: bool) -> Self {
        let curr = (0, 0);
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let prev = (0, 0);
        let remaining = 0;
        let rng = thread_rng();
        let speedup = variant;
        let state = State::Setup;
        Self {
            curr,
            grid,
            prev,
            remaining,
            rng,
            speedup,
            state,
        }
    }
    pub fn filled(&self) -> f32 {
        1.0 - ((self.remaining as f32) / (COLUMNS * ROWS))
    }
    pub fn get_grid(&self) -> [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize] {
        self.grid
    }
    fn from(&mut self, other: Self) {
        self.curr = other.curr;
        self.grid = other.grid;
        self.prev = other.prev;
        self.remaining = other.remaining;
        self.rng = other.rng;
        self.speedup = other.speedup;
        self.state = other.state;
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        if self.speedup {
            String::from("Faster Aldous-Broderish")
        } else {
            String::from("Aldous-Broder")
        }
    }
    fn re_init(&mut self) {
        self.from(Exports::new(self.speedup));
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                self.curr = (
                    self.rng.gen_range(0, COLUMNS as usize),
                    self.rng.gen_range(0, ROWS as usize),
                );
                self.prev = self.curr;
                self.remaining = (ROWS * COLUMNS) as usize - 1;
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
            if self.remaining == 0 {
                self.state = State::Done;
                log::info!("Done!");
                return;
            }

            let (x, y) = self.curr;
            let mut potentials: Vec<Direction> = EnumSet::all().iter().collect();
            potentials.shuffle(&mut self.rng);
            for direction in potentials {
                let (new_x, new_y) = match direction {
                    Direction::North => (x as i32, y as i32 - 1),
                    Direction::East => (x as i32 + 1, y as i32),
                    Direction::South => (x as i32, y as i32 + 1),
                    Direction::West => (x as i32 - 1, y as i32),
                };
                if 0 <= new_x && new_x < COLUMNS as i32 && 0 <= new_y && new_y < ROWS as i32 {
                    // This isn't officially part of Aldous-Broder, but preventing the random walk
                    // from going back and forth a bunch seems to speed up the run by about 3x
                    // (from 30 minutes to 10 minutes)…
                    if self.speedup && (new_x as usize, new_y as usize) == self.prev {
                        continue;
                    }
                    let (new_x, new_y) = (new_x as usize, new_y as usize);
                    if self.grid[new_y][new_x] == EnumSet::new() {
                        self.grid[y][x] |= direction;
                        self.grid[new_y][new_x] |= direction.opposite();
                        self.remaining -= 1;
                    }
                    self.prev = self.curr;
                    self.curr = (new_x, new_y);
                    found = true;
                    break;
                }
            }
        }
    }

    fn draw(&self, gfx: &mut Graphics) -> Result<()> {
        let elements = draw_board(self.grid)?;
        gfx.draw_mesh(&elements);

        if self.state == State::Running {
            let curr_color = COLORS[1];
            let mut cell_color = COLORS[2];
            cell_color.a = 0.5;
            for x in 0..COLUMNS as usize {
                for y in 0..ROWS as usize {
                    if self.grid[y][x] == EnumSet::new() {
                        let rect = Rectangle::new(
                            Vector::new(
                                x as f32 * CELL_WIDTH + OFFSET,
                                y as f32 * CELL_WIDTH + OFFSET,
                            ),
                            Vector::new(CELL_WIDTH, CELL_WIDTH),
                        );
                        gfx.fill_rect(&rect, cell_color);
                    }
                }
            }
            let rect = Rectangle::new(
                Vector::new(
                    self.curr.0 as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                    self.curr.1 as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                ),
                Vector::new(CELL_WIDTH - LINE_WIDTH * 2.0, CELL_WIDTH - LINE_WIDTH * 2.0),
            );
            gfx.fill_rect(&rect, curr_color);
        }
        Ok(())
    }
}
