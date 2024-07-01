use crate::util::{
    draw_board, Algorithm, ChooseRandom, Direction, CELL_WIDTH, COLORS, COLUMNS, LINE_WIDTH,
    OFFSET, ROWS,
};
use enumset::EnumSet;
use macroquad::shapes::draw_rectangle;
use maze_utils::From;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}

#[derive(From)]
pub struct Exports {
    curr: (usize, usize),
    grid: [[Option<Direction>; COLUMNS as usize]; ROWS as usize],
    remaining: usize,
    iterations: usize,
    state: State,
}

impl Exports {
    pub fn new(iterations: usize) -> Self {
        let mut grid = [[Some(Direction::East); COLUMNS as usize]; ROWS as usize];

        for row in grid.iter_mut().take(ROWS as usize - 1) {
            row[COLUMNS as usize - 1] = Some(Direction::South);
        }
        grid[ROWS as usize - 1][COLUMNS as usize - 1] = None;

        Self {
            curr: (0, 0),
            grid,
            remaining: (ROWS * COLUMNS) as usize * 10 * iterations,
            iterations: (ROWS * COLUMNS) as usize * 10 * iterations,
            state: State::Setup,
        }
    }
    pub fn get_grid(&self) -> [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize] {
        let mut rv = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        for y in 0..ROWS as usize {
            for x in 0..COLUMNS as usize {
                if let Some(direction) = self.grid[y][x] {
                    rv[y][x] |= direction;
                    if let Some(offset) = direction.offset((x, y)) {
                        rv[offset.1][offset.0] |= direction.opposite();
                    } else {
                        println!("Pointing off the map!")
                    }
                }
            }
        }
        rv
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Origin Shift")
    }
    fn re_init(&mut self, variant: String) {
        // log::info!("Re-initing with {}", variant);
        self.from(Exports::new(variant.parse().unwrap()));
    }
    fn get_variant(&self) -> String {
        self.iterations.to_string()
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                self.curr = ((COLUMNS / 2.0) as usize - 1, (ROWS / 2.0) as usize - 1);
                self.remaining = self.iterations;
                self.state = State::Running;
                return;
            }
            State::Done => {
                return;
            }
            _ => {}
        }

        if self.remaining == 0 {
            self.state = State::Done;
        }
        self.remaining -= 1;

        // Have self.curr point to a random neighbouring node.
        let mut potentials: Vec<Direction> = Vec::from_iter(EnumSet::all());
        potentials.shuffle();

        for direction in potentials {
            if let Some(new) = direction.offset(self.curr) {
                self.grid[self.curr.1][self.curr.0] = Some(direction);
                // Set the new node to the origin.
                self.curr = new;
                // Remove the new node's pointer.
                self.grid[self.curr.1][self.curr.0] = None;
                break;
            }
        }

        // Performance optimization: relocate the origin to a new, random point by following the arrows from that new node and reversing them along the way, then setting curr to the new node.
    }

    fn draw(&self) {
        draw_board(self.get_grid());

        if self.state == State::Running {
            let curr_color = COLORS[1];
            draw_rectangle(
                self.curr.0 as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                self.curr.1 as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                CELL_WIDTH - LINE_WIDTH * 2.0,
                CELL_WIDTH - LINE_WIDTH * 2.0,
                curr_color,
            );
        }
    }
}
