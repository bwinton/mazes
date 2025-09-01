use crate::util::{
    draw_board, Algorithm, Direction, Grid, Playable, State as BaseState, CELL_WIDTH, COLORS,
    COLUMNS, FIELD_COLOR, LINE_WIDTH, OFFSET, ROWS,
};
use enumset::EnumSet;
use macroquad::{logging as log, prelude::draw_rectangle, rand::gen_range};
use maze_utils::From;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Carving,
    Done,
}

#[derive(From)]
pub struct Exports {
    path: Vec<(usize, usize)>,
    curr: (usize, usize),
    grid: Grid,
    harder: bool,
    run_start: usize,
    state: State,
}

impl Exports {
    pub fn new(variant: bool) -> Self {
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];

        Self {
            path: vec![],
            curr: (0, 0),
            grid,
            harder: variant,
            run_start: 0,
            state: State::Setup,
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
            State::Running => {
                let proportion = if self.harder {
                    (0.4 + (self.curr.0 as f64 / COLUMNS as f64) * 0.4) * 100.0
                } else {
                    50.0
                } as usize;
                if (gen_range(0, 100) < proportion || self.curr.1 == 0)
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
                    let north = gen_range(self.run_start, self.curr.0);
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
            _ => {}
        }
    }

    fn draw(&self) {
        draw_board(self.grid);

        let curr_color = COLORS[1];
        let mut cell_color = COLORS[1];
        cell_color.a = 0.5;

        // Draw the field.
        let y = self.curr.1 as f32 + 1.0;
        draw_rectangle(
            0.0 * CELL_WIDTH + OFFSET,
            y * CELL_WIDTH + OFFSET,
            COLUMNS * CELL_WIDTH,
            (ROWS - y) * CELL_WIDTH,
            FIELD_COLOR,
        );

        let x = self.curr.0 as f32 + 1.0;
        let y = y - 1.0;
        draw_rectangle(
            x * CELL_WIDTH + OFFSET,
            y * CELL_WIDTH + OFFSET,
            (COLUMNS - x) * CELL_WIDTH,
            CELL_WIDTH,
            FIELD_COLOR,
        );

        let start = self.run_start as f32;
        draw_rectangle(
            start * CELL_WIDTH + OFFSET,
            y * CELL_WIDTH + OFFSET,
            (x - start) * CELL_WIDTH,
            CELL_WIDTH,
            cell_color,
        );

        let x = x - 1.0;
        draw_rectangle(
            x * CELL_WIDTH + LINE_WIDTH + OFFSET,
            y * CELL_WIDTH + LINE_WIDTH + OFFSET,
            CELL_WIDTH - LINE_WIDTH * 2.0,
            CELL_WIDTH - LINE_WIDTH * 2.0,
            curr_color,
        );
    }

    fn get_state(&self) -> BaseState {
        match &self.state {
            State::Setup => BaseState::Setup,
            State::Done => BaseState::Done,
            _ => BaseState::Running,
        }
    }

    fn move_to(&mut self, pos: (f32, f32)) {
        Playable::move_to(self, pos);
    }
}

impl Playable for Exports {
    fn get_grid(&self) -> Grid {
        self.grid
    }

    fn get_path_mut(&mut self) -> &mut Vec<(usize, usize)> {
        &mut self.path
    }
}
