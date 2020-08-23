use crate::util::{
    draw_board, Algorithm, Direction, CELL_WIDTH, COLORS, COLUMNS, EMPTY_COLOR, LINE_WIDTH, OFFSET,
    ROWS,
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
    frontier: Vec<(usize, usize)>,
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    grid_state: [[bool; COLUMNS as usize]; ROWS as usize],
    rng: ThreadRng,
    state: State,
    debug: Vec<(usize, usize)>,
}

impl Exports {
    pub fn new() -> Self {
        let frontier = vec![];
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let grid_state = [[false; COLUMNS as usize]; ROWS as usize];
        let rng = thread_rng();
        let state = State::Setup;
        Self {
            frontier,
            grid,
            grid_state,
            rng,
            state,
            debug: vec![],
        }
    }
    fn from(&mut self, other: Self) {
        self.frontier = other.frontier;
        self.grid = other.grid;
        self.grid_state = other.grid_state;
        self.rng = other.rng;
        self.state = other.state;
        self.debug = other.debug;
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Prim")
    }
    fn re_init(&mut self, _variant: String) {
        self.from(Exports::new());
    }
    fn get_variant(&self) -> String {
        "unused".to_owned()
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                // Add an initial cell to the frontier…
                self.frontier.push((
                    self.rng.gen_range(0, COLUMNS as usize),
                    self.rng.gen_range(0, ROWS as usize),
                ));
                self.state = State::Running;
                return;
            }
            State::Done => {
                return;
            }
            _ => {}
        }

        if self.frontier.is_empty() {
            self.state = State::Done;
            log::info!("Done!");
            return;
        }

        let (x, y) = self.frontier.pop().unwrap();
        // log::info!("Popped ({},{}) from {:?}", x, y, self.frontier);
        // Add the cell to the in set!
        self.grid_state[y][x] = true;

        let mut carved = false;
        let mut directions = EnumSet::all();
        if x == 0 {
            directions ^= Direction::West;
        } else if x == COLUMNS as usize - 1 {
            directions ^= Direction::East;
        }
        if y == 0 {
            directions ^= Direction::North;
        } else if y == ROWS as usize - 1 {
            directions ^= Direction::South;
        }
        let mut directions: Vec<Direction> = directions.iter().collect();
        directions.shuffle(&mut self.rng);
        for direction in directions {
            let (new_x, new_y) = match direction {
                Direction::North => (x, y - 1),
                Direction::East => (x + 1, y),
                Direction::South => (x, y + 1),
                Direction::West => (x - 1, y),
            };
            match (self.grid_state[new_y][new_x], carved) {
                (true, false) => {
                    // Find another in cell to carve a path to.
                    self.grid[y][x] |= direction;
                    self.grid[new_y][new_x] |= direction.opposite();
                    carved = true;
                }
                (true, true) => {}
                (false, _) => {
                    // Add the other out cells to the frontier.
                    if !self.frontier.contains(&(new_x, new_y)) {
                        self.frontier.push((new_x, new_y));
                    }
                }
            }
        }
        self.frontier.shuffle(&mut self.rng);
    }

    fn draw(&self, gfx: &mut Graphics) -> Result<()> {
        let elements = draw_board(self.grid)?;
        gfx.draw_mesh(&elements);

        if self.state == State::Running {
            let curr_color = COLORS[1];
            let mut cell_color = COLORS[1];
            cell_color.a = 0.5;

            for x in 0..COLUMNS as usize {
                for y in 0..ROWS as usize {
                    if self.grid[y][x] == EnumSet::empty() && !self.frontier.contains(&(x, y)) {
                        let rect = Rectangle::new(
                            Vector::new(
                                x as f32 * CELL_WIDTH + OFFSET,
                                y as f32 * CELL_WIDTH + OFFSET,
                            ),
                            Vector::new(CELL_WIDTH, CELL_WIDTH),
                        );
                        gfx.fill_rect(&rect, EMPTY_COLOR);
                    };
                }
            }
            for (i, (x, y)) in self.frontier.iter().enumerate() {
                if i == self.frontier.len() - 1 {
                    let rect = Rectangle::new(
                        Vector::new(
                            *x as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                            *y as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                        ),
                        Vector::new(CELL_WIDTH - LINE_WIDTH * 2.0, CELL_WIDTH - LINE_WIDTH * 2.0),
                    );

                    gfx.fill_rect(&rect, curr_color);
                }
                let rect = Rectangle::new(
                    Vector::new(
                        *x as f32 * CELL_WIDTH + OFFSET,
                        *y as f32 * CELL_WIDTH + OFFSET,
                    ),
                    Vector::new(CELL_WIDTH, CELL_WIDTH),
                );
                gfx.fill_rect(&rect, cell_color);
            }
        }

        Ok(())
    }
}
