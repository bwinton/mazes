use crate::util::{
    draw_board, draw_path, Algorithm, ChooseRandom, Direction, Grid, Playable, State, CELL_WIDTH,
    COLORS, COLUMNS, EMPTY_COLOR, LINE_WIDTH, OFFSET, ROWS,
};
use enumset::EnumSet;
use macroquad::{logging as log, prelude::draw_rectangle, rand::gen_range};
use maze_utils::From;

#[derive(From)]
pub struct Exports {
    path: Vec<(usize, usize)>,
    frontier: Vec<(usize, usize)>,
    grid: Grid,
    grid_state: [[bool; COLUMNS as usize]; ROWS as usize],
    state: State,
    debug: Vec<(usize, usize)>,
}

impl Exports {
    pub fn new() -> Self {
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let grid_state = [[false; COLUMNS as usize]; ROWS as usize];

        Self {
            path: vec![],
            frontier: vec![],
            grid,
            grid_state,
            state: State::Setup,
            debug: vec![],
        }
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
        if self.state == State::Setup {
            // Add an initial cell to the frontier…
            self.frontier
                .push((gen_range(0, COLUMNS as usize), gen_range(0, ROWS as usize)));
            self.state = State::Running;
            return;
        }

        if self.frontier.is_empty() {
            self.state = State::Done;
            self.path.push((0, 0));
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
        directions.shuffle();
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
        self.frontier.shuffle();
    }

    fn draw(&self) {
        draw_board(self.grid);

        if self.state == State::Running {
            let curr_color = COLORS[1];
            let mut cell_color = COLORS[1];
            cell_color.a = 0.5;

            for x in 0..COLUMNS as usize {
                for y in 0..ROWS as usize {
                    if self.grid[y][x] == EnumSet::empty() && !self.frontier.contains(&(x, y)) {
                        draw_rectangle(
                            x as f32 * CELL_WIDTH + OFFSET,
                            y as f32 * CELL_WIDTH + OFFSET,
                            CELL_WIDTH,
                            CELL_WIDTH,
                            EMPTY_COLOR,
                        );
                    };
                }
            }
            for (i, (x, y)) in self.frontier.iter().enumerate() {
                if i == self.frontier.len() - 1 {
                    draw_rectangle(
                        *x as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                        *y as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                        CELL_WIDTH - LINE_WIDTH * 2.0,
                        CELL_WIDTH - LINE_WIDTH * 2.0,
                        curr_color,
                    );
                }
                draw_rectangle(
                    *x as f32 * CELL_WIDTH + OFFSET,
                    *y as f32 * CELL_WIDTH + OFFSET,
                    CELL_WIDTH,
                    CELL_WIDTH,
                    cell_color,
                );
            }
        }
        draw_path(&self.path);
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
