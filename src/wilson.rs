use crate::util::{
    draw_board, Algorithm, ChooseRandom, Direction, CELL_WIDTH, COLORS, COLUMNS, FIELD_COLOR,
    LINE_WIDTH, OFFSET, ROWS,
};
use enumset::EnumSet;
use macroquad::{
    logging as log,
    prelude::{draw_line, draw_rectangle, Color},
    rand::gen_range,
};
use maze_utils::From;

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

#[derive(From)]
pub struct Exports {
    path: Vec<(usize, usize)>,
    current: Option<(usize, usize)>,
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    previous: Option<(usize, usize)>,
    processing: [[Cell; COLUMNS as usize]; ROWS as usize],
    remaining: usize,
    slowdown: bool,
    start: Option<(usize, usize)>,
    state: State,
}

impl Exports {
    pub fn new(variant: bool) -> Self {
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let processing = [[Cell::Out; COLUMNS as usize]; ROWS as usize];

        Self {
            path: vec![],
            current: None,
            grid,
            previous: None,
            processing,
            remaining: 0,
            slowdown: variant,
            start: None,
            state: State::Setup,
        }
    }
    pub fn is_done(&self) -> bool {
        self.state == State::Done
    }
    pub fn init_from_grid(
        &mut self,
        incoming: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    ) {
        self.state = State::Finding;
        self.remaining = (ROWS * COLUMNS) as usize;
        self.grid = incoming;
        for (y, row) in incoming.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == EnumSet::new() {
                    self.processing[y][x] = Cell::Out;
                } else {
                    self.processing[y][x] = Cell::In;
                    self.remaining -= 1;
                }
            }
        }
    }
    fn draw_arrow(&self, x: f32, y: f32, direction: Direction, color: Color) {
        let x = x * CELL_WIDTH + OFFSET;
        let y = y * CELL_WIDTH + OFFSET;
        let mut points = vec![];
        match direction {
            Direction::North => {
                points.push((x + 3.0 * UNITS, y + 5.0 * UNITS));
                points.push((x + 6.0 * UNITS, y + 2.0 * UNITS));
                points.push((x + 6.0 * UNITS, y + 10.0 * UNITS));
                points.push((x + 6.0 * UNITS, y + 2.0 * UNITS));
                points.push((x + 9.0 * UNITS, y + 5.0 * UNITS));
            }
            Direction::East => {
                points.push((x + 7.0 * UNITS, y + 3.0 * UNITS));
                points.push((x + 10.0 * UNITS, y + 6.0 * UNITS));
                points.push((x + 2.0 * UNITS, y + 6.0 * UNITS));
                points.push((x + 10.0 * UNITS, y + 6.0 * UNITS));
                points.push((x + 7.0 * UNITS, y + 9.0 * UNITS));
            }
            Direction::South => {
                points.push((x + 3.0 * UNITS, y + 7.0 * UNITS));
                points.push((x + 6.0 * UNITS, y + 10.0 * UNITS));
                points.push((x + 6.0 * UNITS, y + 2.0 * UNITS));
                points.push((x + 6.0 * UNITS, y + 10.0 * UNITS));
                points.push((x + 9.0 * UNITS, y + 7.0 * UNITS));
            }
            Direction::West => {
                points.push((x + 5.0 * UNITS, y + 3.0 * UNITS));
                points.push((x + 2.0 * UNITS, y + 6.0 * UNITS));
                points.push((x + 10.0 * UNITS, y + 6.0 * UNITS));
                points.push((x + 2.0 * UNITS, y + 6.0 * UNITS));
                points.push((x + 5.0 * UNITS, y + 9.0 * UNITS));
            }
        }
        for (first, second) in points.iter().zip(points.iter().skip(1)) {
            draw_line(first.0, first.1, second.0, second.1, LINE_WIDTH, color);
        }
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
    fn re_init(&mut self, variant: String) {
        log::info!(
            "REiniting from {}/{} with {}",
            self.slowdown,
            self.get_variant(),
            variant
        );
        self.from(Exports::new(variant == "slow"));
        log::info!("  to {}/{}", self.slowdown, self.get_variant());
    }
    fn get_variant(&self) -> String {
        if self.slowdown {
            "slow".to_owned()
        } else {
            "fast".to_owned()
        }
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                let x = gen_range(0, COLUMNS as usize);
                let y = gen_range(0, ROWS as usize);
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
                    self.start = potentials.choose();
                    if self.start.is_none() {
                        panic!("Couldn't find a random element, but we think we need one!");
                    }
                    self.current = self.start;
                }
                let (x, y) = self.current.unwrap();

                let mut potentials: Vec<Direction> = EnumSet::all().iter().collect();
                potentials.shuffle();
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

    fn draw(&self) {
        draw_board(self.grid);

        let mut start_color = COLORS[1];
        start_color.a = 0.5;
        let arrow_color = COLORS[1];
        let curr_color = COLORS[1];

        if let Some((x, y)) = self.current {
            draw_rectangle(
                x as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                y as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                CELL_WIDTH - LINE_WIDTH * 2.0,
                CELL_WIDTH - LINE_WIDTH * 2.0,
                curr_color,
            );
        }

        if let Some((x, y)) = self.start {
            draw_rectangle(
                x as f32 * CELL_WIDTH + OFFSET,
                y as f32 * CELL_WIDTH + OFFSET,
                CELL_WIDTH,
                CELL_WIDTH,
                start_color,
            );
        }

        for x in 0..COLUMNS as usize {
            for y in 0..ROWS as usize {
                if Some((x, y)) == self.current {
                    continue;
                }
                match self.processing[y][x] {
                    Cell::Out => {
                        if Some((x, y)) == self.start {
                            continue;
                        }
                        draw_rectangle(
                            x as f32 * CELL_WIDTH + OFFSET,
                            y as f32 * CELL_WIDTH + OFFSET,
                            CELL_WIDTH,
                            CELL_WIDTH,
                            FIELD_COLOR,
                        );
                    }
                    Cell::Direction(direction) => {
                        draw_rectangle(
                            x as f32 * CELL_WIDTH + OFFSET,
                            y as f32 * CELL_WIDTH + OFFSET,
                            CELL_WIDTH,
                            CELL_WIDTH,
                            start_color,
                        );
                        self.draw_arrow(x as f32, y as f32, direction, arrow_color);
                    }
                    _ => {}
                }
            }
        }
    }
}
