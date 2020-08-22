use crate::util::{
    draw_board, Algorithm, Direction, CELL_WIDTH, COLORS, COLUMNS, LINE_WIDTH, OFFSET,
    ROWS,
};
use enumset::EnumSet;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Color, Graphics},
    log, Result,
};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

const UNITS: f32 = CELL_WIDTH / 12.0;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Finding,
    Following,
    Done,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
    Direction(Direction),
    In,
    Out,
}

pub struct Exports {
    current: Option<(usize, usize)>,
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    previous: Option<(usize, usize)>,
    processing: [[Cell; COLUMNS as usize]; ROWS as usize],
    remaining: usize,
    rng: ThreadRng,
    slowdown: bool,
    start: Option<(usize, usize)>,
    state: State,
}

impl Exports {
    pub fn new(variant: bool) -> Self {
        let current = None;
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let previous = None;
        let processing = [[Cell::Out; COLUMNS as usize]; ROWS as usize];
        let remaining = 0;
        let rng = thread_rng();
        let slowdown = variant;
        let start = None;
        let state = State::Setup;
        Self {
            current,
            grid,
            previous,
            processing,
            remaining,
            rng,
            slowdown,
            start,
            state,
        }
    }
    fn from(&mut self, other: Self) {
        self.current = other.current;
        self.grid = other.grid;
        self.processing = other.processing;
        self.remaining = other.remaining;
        self.rng = other.rng;
        self.start = other.start;
        self.state = other.state;
    }
    fn draw_arrow(&self, x: f32, y: f32, direction: Direction, color: Color, gfx: &mut Graphics) {
        let x = x * CELL_WIDTH + OFFSET;
        let y = y * CELL_WIDTH + OFFSET;
        let mut points = vec![];
        match direction {
            Direction::North => {
                points.push(Vector::new(x + 3.0 * UNITS, y + 5.0 * UNITS));
                points.push(Vector::new(x + 6.0 * UNITS, y + 2.0 * UNITS));
                points.push(Vector::new(x + 6.0 * UNITS, y + 10.0 * UNITS));
                points.push(Vector::new(x + 6.0 * UNITS, y + 2.0 * UNITS));
                points.push(Vector::new(x + 9.0 * UNITS, y + 5.0 * UNITS));
            }
            Direction::East => {
                points.push(Vector::new(x + 7.0 * UNITS, y + 3.0 * UNITS));
                points.push(Vector::new(x + 10.0 * UNITS, y + 6.0 * UNITS));
                points.push(Vector::new(x + 2.0 * UNITS, y + 6.0 * UNITS));
                points.push(Vector::new(x + 10.0 * UNITS, y + 6.0 * UNITS));
                points.push(Vector::new(x + 7.0 * UNITS, y + 9.0 * UNITS));
            }
            Direction::South => {
                points.push(Vector::new(x + 3.0 * UNITS, y + 7.0 * UNITS));
                points.push(Vector::new(x + 6.0 * UNITS, y + 10.0 * UNITS));
                points.push(Vector::new(x + 6.0 * UNITS, y + 2.0 * UNITS));
                points.push(Vector::new(x + 6.0 * UNITS, y + 10.0 * UNITS));
                points.push(Vector::new(x + 9.0 * UNITS, y + 7.0 * UNITS));
            }
            Direction::West => {
                points.push(Vector::new(x + 5.0 * UNITS, y + 3.0 * UNITS));
                points.push(Vector::new(x + 2.0 * UNITS, y + 6.0 * UNITS));
                points.push(Vector::new(x + 10.0 * UNITS, y + 6.0 * UNITS));
                points.push(Vector::new(x + 2.0 * UNITS, y + 6.0 * UNITS));
                points.push(Vector::new(x + 5.0 * UNITS, y + 9.0 * UNITS));
            }
        }
        gfx.stroke_path(&points, color);
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        if self.slowdown {
            String::from("Slower Wilsonish")
        } else {
            String::from("Wilson")
        }
    }
    fn re_init(&mut self) {
        self.from(Exports::new(self.slowdown));
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                let x = self.rng.gen_range(0, COLUMNS as usize);
                let y = self.rng.gen_range(0, ROWS as usize);
                self.processing[y][x] = Cell::In;
                self.remaining = (ROWS * COLUMNS - 1.0) as usize;

                self.state = State::Finding;
                return;
            }
            State::Done => {
                return;
            }
            _ => {}
        }

        if self.remaining == 0 {
            self.start = None;
            self.current = None;
            self.state = State::Done;
            log::info!("Done!");
            return;
        }

        match self.state {
            State::Finding => {
                // log::info!("Finding, start={:?}", self.start);
                if self.start.is_none() {
                    let mut potentials = vec![];
                    for x in 0..COLUMNS as usize {
                        for y in 0..ROWS as usize {
                            if self.processing[y][x] == Cell::Out {
                                potentials.push((x, y));
                            }
                        }
                    }
                    self.start = potentials.choose(&mut self.rng).map(|x| x.clone());
                    if self.start.is_none() {
                        panic!("Couldn't find a random element, but we think we need one!");
                    }
                    self.current = self.start;
                }
                let (x, y) = self.current.unwrap();

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
                        // For some reason the checking-previous trick that sped up Aldous-Broder
                        // seems to slow down Wilson… ¯\_(ツ)_/¯
                        let (new_x, new_y) = (new_x as usize, new_y as usize);
                        if self.slowdown && Some((new_x, new_y)) == self.previous {
                            continue;
                        }
                        self.processing[y][x] = Cell::Direction(direction);
                        self.previous = self.current;
                        self.current = Some((new_x, new_y));
                        if self.processing[new_y][new_x] == Cell::In {
                            // We found it!!!
                            self.current = self.start;
                            self.start = None;
                            self.state = State::Following;
                            // log::info!("Switching to Following!");
                        }
                        break;
                    }
                }
            }
            State::Following => {
                let (x, y) = self.current.unwrap();
                match self.processing[y][x] {
                    Cell::Direction(direction) => {
                        self.processing[y][x] = Cell::In;
                        self.remaining -= 1;
                        let (new_x, new_y) = match direction {
                            Direction::North => (x, y - 1),
                            Direction::East => (x + 1, y),
                            Direction::South => (x, y + 1),
                            Direction::West => (x - 1, y),
                        };
                        self.current = Some((new_x, new_y));
                        self.grid[y][x] |= direction;
                        self.grid[new_y][new_x] |= direction.opposite();

                        // log::info!("Moving from ({},{})/{:?} to  ({},{})/{:?}", x, y, direction, new_x, new_y, self.processing[new_y][new_x]);
                    }
                    Cell::In => {
                        // We found it!
                        for x in 0..COLUMNS as usize {
                            for y in 0..ROWS as usize {
                                if let Cell::Direction(_) = self.processing[y][x] {
                                    self.processing[y][x] = Cell::Out;
                                }
                            }
                        }
                        self.state = State::Finding;
                        // log::info!("Switching to Finding!");
                    }
                    _ => {
                        panic!(
                            "Should be unable to hit cell {:?} at ({},{})!",
                            self.processing[y][x], x, y
                        );
                    }
                }
            }
            _ => {
                panic!(
                    "Should be unable to hit state {:?} in this match!",
                    self.state
                );
            }
        }
    }

    fn draw(&self, gfx: &mut Graphics) -> Result<()> {
        let elements = draw_board(self.grid)?;
        gfx.draw_mesh(&elements);

        let mut start_color = COLORS[1];
        start_color.a = 0.3;
        let arrow_color = COLORS[1];
        let curr_color = COLORS[3];
        let mut empty_color = COLORS[2];
        empty_color.a = 0.3;


        if let Some((x, y)) = self.current {
            let rect = Rectangle::new(
                Vector::new(
                    x as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                    y as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                ),
                Vector::new(CELL_WIDTH - LINE_WIDTH * 2.0, CELL_WIDTH - LINE_WIDTH * 2.0),
            );
            gfx.fill_rect(&rect, curr_color);
        }

        if let Some((x, y)) = self.start {
            let rect = Rectangle::new(
                Vector::new(
                    x as f32 * CELL_WIDTH + OFFSET,
                    y as f32 * CELL_WIDTH + OFFSET,
                ),
                Vector::new(CELL_WIDTH, CELL_WIDTH),
            );
            gfx.fill_rect(&rect, start_color);
        }

        for x in 0..COLUMNS as usize {
            for y in 0..ROWS as usize {
                match self.processing[y][x] {
                    Cell::Out => {
                        let rect = Rectangle::new(
                            Vector::new(
                                x as f32 * CELL_WIDTH + OFFSET,
                                y as f32 * CELL_WIDTH + OFFSET,
                            ),
                            Vector::new(CELL_WIDTH, CELL_WIDTH),
                        );
                        gfx.fill_rect(&rect, empty_color);
                    },
                    Cell::Direction(direction) => {
                        let rect = Rectangle::new(
                            Vector::new(
                                x as f32 * CELL_WIDTH + OFFSET,
                                y as f32 * CELL_WIDTH + OFFSET,
                            ),
                            Vector::new(CELL_WIDTH, CELL_WIDTH),
                        );
                        gfx.fill_rect(&rect, start_color);
                        self.draw_arrow(x as f32, y as f32, direction, arrow_color, gfx);
                    },
                    _ => {},
                }
            }
        }

        Ok(())
    }
}
