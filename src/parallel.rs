use crate::util::{
    draw_board, Algorithm, Direction, CELL_WIDTH, COLORS, COLUMNS, LINE_WIDTH, OFFSET, ROWS,
};

use std::collections::{HashSet, VecDeque};

use array_init::array_init;
use enumset::EnumSet;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Graphics,
    log, Result,
};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng, Rng};

const MAX_SEEDS: usize = 6;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}
pub struct Exports {
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    grid_seeds: [[Option<usize>; COLUMNS as usize]; ROWS as usize],
    rng: ThreadRng,
    seeds: usize,
    sets: [HashSet<usize>; MAX_SEEDS],
    stack: [VecDeque<(usize, usize, EnumSet<Direction>)>; MAX_SEEDS],
    state: State,
}

impl Exports {
    pub fn new(seeds: usize) -> Self {
        if seeds < 1 || seeds > MAX_SEEDS {
            panic!("Seeds {} must be between {} and {}", seeds, 1, MAX_SEEDS);
        }
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let grid_seeds = [[None; COLUMNS as usize]; ROWS as usize];
        let rng = thread_rng();
        let sets = array_init(|_| HashSet::new());
        let stack = array_init(|_| VecDeque::new());
        let state = State::Setup;
        Self {
            grid,
            grid_seeds,
            rng,
            seeds,
            sets,
            stack,
            state,
        }
    }
    fn from(&mut self, other: Self) {
        self.grid = other.grid;
        self.grid_seeds = other.grid_seeds;
        self.rng = other.rng;
        self.stack = other.stack;
        self.seeds = other.seeds;
        self.sets = other.sets;
        self.state = other.state;
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        if self.seeds == 1 {
            String::from("Backtrack")
        } else {
            String::from("Parallel Backtrack")
        }
    }
    fn re_init(&mut self, variant: String) {
        self.from(Exports::new(variant.parse().unwrap()));
    }
    fn get_variant(&self) -> String {
        self.seeds.to_string()
    }
    fn update(&mut self) {
        // println!("Updating {}", self.name());
        match self.state {
            State::Setup => {
                for (i, stack) in self.stack.iter_mut().take(self.seeds).enumerate() {
                    let x = self.rng.gen_range(0, COLUMNS as usize);
                    let y = self.rng.gen_range(0, ROWS as usize);
                    stack.push_front((x, y, EnumSet::all()));
                    self.sets[i].insert(i);
                }

                self.state = State::Running;
                return;
            }
            State::Done => {
                return;
            }
            _ => {}
        }

        let mut done = true;

        'outer: for (i, stack) in self.stack.iter_mut().take(self.seeds).enumerate() {
            let mut found = false;
            // let (first_x, first_y, _) = self.stack.front().unwrap().clone();

            while !found {
                if stack.is_empty() {
                    continue 'outer;
                }
                done = false;

                let (x, y, directions) = stack.pop_front().unwrap();
                let mut potentials: Vec<Direction> = directions.iter().collect();
                if potentials.is_empty() {
                    continue 'outer;
                }
                potentials.shuffle(&mut self.rng);
                let direction = potentials.pop().unwrap();
                // println!("({},{}) -> {:?}", x, y, direction);
                stack.push_front((x, y, directions ^ direction));

                let (new_x, new_y) = match direction {
                    Direction::North => (x as i32, y as i32 - 1),
                    Direction::East => (x as i32 + 1, y as i32),
                    Direction::South => (x as i32, y as i32 + 1),
                    Direction::West => (x as i32 - 1, y as i32),
                };
                // println!("{:?} / {:?} -> {:?}", (x,y), direction, (new_x, new_y));
                if 0 <= new_x && new_x < COLUMNS as i32 && 0 <= new_y && new_y < ROWS as i32 {
                    let (new_x, new_y) = (new_x as usize, new_y as usize);
                    if self.grid[new_y][new_x] == EnumSet::new()
                        && self.grid_seeds[new_y][new_x] == None
                    {
                        self.grid_seeds[y][x] = Some(i);
                        self.grid[y][x] |= direction;
                        self.grid_seeds[new_y][new_x] = Some(i);
                        self.grid[new_y][new_x] |= direction.opposite();
                        stack.push_front((new_x, new_y, EnumSet::all() ^ direction.opposite()));
                        found = true;
                    } else if let Some(set) = self.grid_seeds[new_y][new_x] {
                        if !self.sets[i].contains(&set) {
                            let both_sets: HashSet<usize> =
                                self.sets[i].union(&self.sets[set]).cloned().collect();
                            for i in &both_sets {
                                self.sets[*i] = both_sets.clone();
                            }
                            self.grid[y][x] |= direction;
                            self.grid[new_y][new_x] |= direction.opposite();
                        }
                    }
                    // Otherwise, loop again and see what we can get.
                }
            }
        }
        if done {
            self.state = State::Done;
            log::info!("Done!");
        }
    }

    fn draw(&self, gfx: &mut Graphics) -> Result<()> {
        let elements = draw_board(self.grid)?;
        gfx.draw_mesh(&elements);

        for i in 0..self.seeds {
            let curr_color = COLORS[i + 1];
            let mut cell_color = COLORS[i + 1];
            cell_color.a = 0.5;
            for (i, (x, y, _)) in self.stack[i].iter().enumerate() {
                if i == 0 {
                    let rect = Rectangle::new(
                        Vector::new(
                            *x as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                            *y as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                        ),
                        Vector::new(CELL_WIDTH - LINE_WIDTH * 2.0, CELL_WIDTH - LINE_WIDTH * 2.0),
                    );
                    gfx.fill_rect(&rect, curr_color);
                } else {
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
        }

        Ok(())
    }
}
