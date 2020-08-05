use crate::util::{draw_board, Algorithm, Direction, CELL_WIDTH, COLORS, COLUMNS, OFFSET, ROWS};
use enumset::EnumSet;
use quicksilver::{
    // geom::{Rectangle, Vector},
    graphics::Graphics,
    log, Result,
};
use rand::{rngs::ThreadRng, thread_rng, Rng};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}

#[derive(PartialEq, Eq, Debug)]
enum Orientation {
    HORIZONTAL,
    VERTICAL,
}

pub struct Exports {
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    rng: ThreadRng,
    stack: Vec<(usize, usize, usize, usize)>,
    state: State,
}

impl Exports {
    pub fn new() -> Self {
        let mut grid = [[EnumSet::all(); COLUMNS as usize]; ROWS as usize];
        for x in 0..COLUMNS as usize {
            grid[0][x].remove(Direction::North);
            grid[ROWS as usize - 1][x].remove(Direction::South);
        }
        for y in 0..ROWS as usize {
            grid[y][0].remove(Direction::West);
            grid[y][COLUMNS as usize - 1].remove(Direction::East);
        }
        let rng = thread_rng();
        let stack = vec![];
        let state = State::Setup;
        Self {
            grid,
            rng,
            stack,
            state,
        }
    }
    fn from(&mut self, other: Self) {
        self.grid = other.grid;
        self.rng = other.rng;
        self.stack = other.stack;
        self.state = other.state;
    }

    fn choose_orientation(&mut self, width: usize, height: usize) -> Orientation {
        if width < height {
            Orientation::HORIZONTAL
        } else if height < width {
            Orientation::VERTICAL
        } else {
            if self.rng.gen() {
                Orientation::HORIZONTAL
            } else {
                Orientation::VERTICAL
            }
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Recursive Division")
    }
    fn re_init(&mut self) {
        self.from(Exports::new());
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                self.stack.push((0, 0, COLUMNS as usize, ROWS as usize));
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
            if self.stack.is_empty() {
                self.state = State::Done;
                log::info!("Done!");
                return;
            }

            let (x, y, width, height) = self.stack.pop().unwrap();
            let orientation = self.choose_orientation(width, height);
            log::info!("Cutting ({},{})x({},{}) in {:?}", x, y, width, height, orientation);

            match orientation {
                Orientation::HORIZONTAL => {
                    // log::info!("GenRange 1 {}-{}", y, y + height);
                    let wall_y = self.rng.gen_range(y, y + height - 1);
                    // log::info!("GenRange 2 {}-{}", x, x + width);
                    let passage_x = self.rng.gen_range(x, x + width);
                    log::info!("  H-At ({},{})", wall_y, passage_x);
                    for i in x..x+width {
                        self.grid[wall_y][i].remove(Direction::South);
                    }
                    self.grid[wall_y][passage_x].insert(Direction::South);

                    let new_height = wall_y - y + 1;
                    if width >= 2 && new_height >= 2 {
                        log::info!("  1 - Adding {},{}x{},{}", x, y, width, new_height);
                        self.stack.push((x, y, width, new_height));
                    }

                    let new_height = height - new_height;
                    if width >= 2 && new_height >= 2 {
                        log::info!("  2 - Adding {},{}x{},{}", x, wall_y + 1, width, new_height);
                        self.stack.push((x, wall_y + 1, width, new_height));
                    }
                }
                Orientation::VERTICAL => {
                    // log::info!("GenRange 3 {}-{}", x, x + width);
                    let wall_x = self.rng.gen_range(x, x + width - 1);
                    // log::info!("GenRange 4 {}-{}", y, y + height);
                    let passage_y = self.rng.gen_range(y, y + height);
                    log::info!("  V-At ({},{})", wall_x, passage_y);
                    for j in y..y+height {
                        self.grid[j][wall_x].remove(Direction::East);
                    }
                    self.grid[passage_y][wall_x].insert(Direction::East);

                    let new_width  = wall_x - x + 1;
                    if new_width >= 2 && height >= 2 {
                        log::info!("  3 - Adding {},{}x{},{}", x, y, new_width, height);
                        self.stack.push((x, y, new_width, height));
                    }

                    let new_width = width - new_width;
                    if new_width >= 2 && height >= 2 {
                        log::info!("  4 - Adding {},{}x{},{}", wall_x + 1, y, new_width, height);
                        self.stack.push((wall_x + 1, y, new_width, height));
                    }
                },
            }
            found = true;
        }
    }

    fn draw(&self, gfx: &mut Graphics) -> Result<()> {
        let elements = draw_board(self.grid)?;
        gfx.draw_mesh(&elements);

        Ok(())
    }
}
