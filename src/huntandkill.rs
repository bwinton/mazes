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
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Walking,
    Finding,
    Done,
}

#[derive(From)]
pub struct Exports {
    curr: Option<(usize, usize)>,
    first_empty_line: usize,
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    rng: ThreadRng,
    scan_line: Option<usize>,
    state: State,
}

impl Exports {
    pub fn new() -> Self {
        let curr = None;
        let first_empty_line = 0;
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let rng = thread_rng();
        let scan_line = None;
        let state = State::Setup;
        Self {
            curr,
            first_empty_line,
            grid,
            rng,
            scan_line,
            state,
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Hunt and Kill")
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
                let x = self.rng.gen_range(0, COLUMNS as usize);
                let y = self.rng.gen_range(0, ROWS as usize);
                self.curr = Some((x, y));

                self.state = State::Walking;
            }

            State::Walking => {
                let (x, y) = self.curr.unwrap();
                let mut potentials: Vec<Direction> = self.grid[y][x].complement().iter().collect();
                // println!("({},{}) / {:?}", x, y, potentials);
                potentials.shuffle(&mut self.rng);
                while !potentials.is_empty() {
                    let direction = potentials.pop().unwrap();
                    let (new_x, new_y) = match direction {
                        Direction::North => (x as i32, y as i32 - 1),
                        Direction::East => (x as i32 + 1, y as i32),
                        Direction::South => (x as i32, y as i32 + 1),
                        Direction::West => (x as i32 - 1, y as i32),
                    };
                    // println!("{:?} / {:?} -> {:?}", (x,y), direction, (new_x, new_y));
                    if 0 <= new_x && new_x < COLUMNS as i32 && 0 <= new_y && new_y < ROWS as i32 {
                        let (new_x, new_y) = (new_x as usize, new_y as usize);
                        if self.grid[new_y][new_x] != EnumSet::new() {
                            continue;
                        }
                        self.grid[y][x] |= direction;
                        self.grid[new_y][new_x] |= direction.opposite();
                        self.curr = Some((new_x, new_y));
                        break;
                    }
                }
                if self.curr == Some((x, y)) {
                    // We didn't find a direction to go, so start the Finding!
                    self.curr = None;
                    self.scan_line = Some(self.first_empty_line);
                    self.state = State::Finding;
                    // log::info!("Switching to Finding!");
                }
            }

            State::Finding => {
                let mut potentials = vec![];
                // log::info!("Starting from {:?}", self.scan_line);
                let y = self.scan_line.unwrap();
                let mut found_empty_cell = false;
                for x in 0..COLUMNS as usize {
                    if self.grid[y][x] == EnumSet::empty() {
                        found_empty_cell = true;
                        let mut neighbours = vec![];
                        if y > 0 && self.grid[y - 1][x] != EnumSet::empty() {
                            neighbours.push(Direction::North);
                        }
                        if x < COLUMNS as usize - 1 && self.grid[y][x + 1] != EnumSet::empty() {
                            neighbours.push(Direction::East);
                        }
                        if y < ROWS as usize - 1 && self.grid[y + 1][x] != EnumSet::empty() {
                            neighbours.push(Direction::South);
                        }
                        if x > 0 && self.grid[y][x - 1] != EnumSet::empty() {
                            neighbours.push(Direction::West);
                        }

                        if let Some(direction) = neighbours.choose(&mut self.rng) {
                            potentials.push((x, *direction));
                        }
                    }
                }

                if potentials.is_empty() {
                    if y < ROWS as usize - 1 {
                        // Move to the next line…
                        self.scan_line = Some(y + 1);
                        if !found_empty_cell {
                            self.first_empty_line = y + 1;
                        }
                    // log::info!("Moving to {:?}", self.scan_line);
                    } else {
                        // We're done!
                        self.scan_line = None;
                        self.state = State::Done;
                        log::info!("Done!");
                    }
                    return;
                }

                // Otherwise, pick one of the potentials, and go from there!
                let (x, direction) = *potentials.choose(&mut self.rng).unwrap();

                let (new_x, new_y) = match direction {
                    Direction::North => (x as i32, y as i32 - 1),
                    Direction::East => (x as i32 + 1, y as i32),
                    Direction::South => (x as i32, y as i32 + 1),
                    Direction::West => (x as i32 - 1, y as i32),
                };

                self.grid[y][x] |= direction;
                self.grid[new_y as usize][new_x as usize] |= direction.opposite();
                self.curr = Some((x, y));
                self.scan_line = None;
                self.state = State::Walking;
                // log::info!("Switching to Walking from ({},{})!", x, y);
            }
            State::Done => {}
        }
    }

    fn draw(&self, gfx: &mut Graphics) -> Result<()> {
        let elements = draw_board(self.grid)?;
        gfx.draw_mesh(&elements);

        let curr_color = COLORS[1];
        let mut cell_color = COLORS[1];
        cell_color.a = 0.3;

        for x in 0..COLUMNS as usize {
            for y in 0..ROWS as usize {
                if self.grid[y][x] == EnumSet::empty() {
                    let rect = Rectangle::new(
                        Vector::new(
                            x as f32 * CELL_WIDTH + OFFSET,
                            y as f32 * CELL_WIDTH + OFFSET,
                        ),
                        Vector::new(CELL_WIDTH, CELL_WIDTH),
                    );
                    gfx.fill_rect(&rect, FIELD_COLOR);
                };
            }
        }
        if let Some((x, y)) = self.curr {
            let rect = Rectangle::new(
                Vector::new(
                    x as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                    y as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                ),
                Vector::new(CELL_WIDTH - LINE_WIDTH * 2.0, CELL_WIDTH - LINE_WIDTH * 2.0),
            );
            gfx.fill_rect(&rect, curr_color);
        }
        if let Some(line) = self.scan_line {
            let rect = Rectangle::new(
                Vector::new(OFFSET, line as f32 * CELL_WIDTH + OFFSET),
                Vector::new(COLUMNS * CELL_WIDTH, CELL_WIDTH),
            );
            gfx.fill_rect(&rect, cell_color);
        }

        if self.state != State::Done {
            let rect = Rectangle::new(
                Vector::new(OFFSET, self.first_empty_line as f32 * CELL_WIDTH + OFFSET),
                Vector::new(COLUMNS * CELL_WIDTH, CELL_WIDTH),
            );
            gfx.fill_rect(&rect, cell_color);
        }

        Ok(())
    }
}
