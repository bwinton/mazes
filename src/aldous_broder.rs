use crate::util::{
    draw_board, Algorithm, ChooseRandom, Direction, Grid, Playable, State, CELL_WIDTH, COLORS,
    COLUMNS, FIELD_COLOR, LINE_WIDTH, OFFSET, ROWS,
};
use enumset::EnumSet;
use macroquad::{logging as log, prelude::draw_rectangle, rand::gen_range};
use maze_utils::From;

#[derive(From)]
pub struct Exports {
    path: Vec<(usize, usize)>,
    curr: (usize, usize),
    grid: Grid,
    prev: (usize, usize),
    remaining: usize,
    speedup: bool,
    state: State,
}

impl Exports {
    pub fn new(variant: bool) -> Self {
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];

        Self {
            path: vec![],
            curr: (0, 0),
            grid,
            prev: (0, 0),
            remaining: 0,
            speedup: variant,
            state: State::Setup,
        }
    }
    pub fn filled(&self) -> f32 {
        1.0 - ((self.remaining as f32) / (COLUMNS * ROWS))
    }
    pub fn get_grid(&self) -> Grid {
        self.grid
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
    fn re_init(&mut self, variant: String) {
        self.from(Exports::new(variant == "fast"));
    }
    fn get_variant(&self) -> String {
        if self.speedup {
            "fast".to_owned()
        } else {
            "slow".to_owned()
        }
    }
    fn update(&mut self) {
        if self.state == State::Setup {
            self.curr = (gen_range(0, COLUMNS as usize), gen_range(0, ROWS as usize));
            self.prev = self.curr;
            self.remaining = (ROWS * COLUMNS) as usize - 1;
            self.state = State::Running;
            return;
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
            potentials.shuffle();
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

    fn draw(&self) {
        draw_board(self.grid);

        if self.state == State::Running {
            let curr_color = COLORS[1];
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
                    }
                }
            }
            draw_rectangle(
                self.curr.0 as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                self.curr.1 as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                CELL_WIDTH - LINE_WIDTH * 2.0,
                CELL_WIDTH - LINE_WIDTH * 2.0,
                curr_color,
            );
        }
    }

    fn get_state(&self) -> State {
        self.state
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
