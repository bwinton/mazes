use crate::util::{draw_board, Algorithm, Direction, CELL_WIDTH, COLORS, COLUMNS, LINE_WIDTH, ROWS};

use std::collections::{HashSet, VecDeque};

use array_init::array_init;
use enumset::EnumSet;
use ggez::graphics::{
    Color, DrawMode, DrawParam, FillOptions, Rect,
};
use ggez::{graphics, Context, GameResult};

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

const SEEDS: usize = 5;

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
    stack: [VecDeque<(usize, usize, EnumSet<Direction>)>; SEEDS],
    sets: [HashSet<usize>; SEEDS],
    state: State,
}

impl Exports {
    pub fn new() -> Self {
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let grid_seeds = [[None; COLUMNS as usize]; ROWS as usize];
        let rng = thread_rng();
        let stack = array_init(|_| VecDeque::new());
        let sets = array_init(|_| HashSet::new());
        let state = State::Setup;
        Self {
            grid,
            grid_seeds,
            rng,
            stack,
            sets,
            state,
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Parallel Backtrack")
    }
    fn update(&mut self) {
        // println!("Updating {}", self.name());
        match self.state {
            State::Setup => {
                for (i, stack) in self.stack.iter_mut().enumerate() {
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

        'outer: for (i, stack) in self.stack.iter_mut().enumerate() {
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
                    if self.grid[new_y][new_x] == EnumSet::new() && self.grid_seeds[new_y][new_x] == None {
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
            println!("Done!");
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // Draw code here...
        let mut builder = draw_board(self.grid)?;

        for i in 0..SEEDS {
            let curr_color = Color::from_rgba_u32(COLORS[i + 1]);
            let mut cell_color = Color::from_rgba_u32(COLORS[i + 1]);
            cell_color.a = 0.5;
            for (i, (x, y, _)) in self.stack[i].iter().enumerate() {
                if i == 0 {
                    builder.rectangle(
                        DrawMode::Fill(FillOptions::default()),
                        Rect::new(
                            *x as f32 * CELL_WIDTH + LINE_WIDTH,
                            *y as f32 * CELL_WIDTH + LINE_WIDTH,
                            CELL_WIDTH - LINE_WIDTH * 2.0,
                            CELL_WIDTH - LINE_WIDTH * 2.0,
                        ),
                        curr_color,
                    );
                } else {
                    builder.rectangle(
                        DrawMode::Fill(FillOptions::default()),
                        Rect::new(
                            *x as f32 * CELL_WIDTH,
                            *y as f32 * CELL_WIDTH,
                            CELL_WIDTH,
                            CELL_WIDTH,
                        ),
                        cell_color,
                    );
                }
            }
        }
        let mesh = builder.build(ctx)?;
        let dest = DrawParam::default().dest([LINE_WIDTH / 2.0, LINE_WIDTH / 2.0]);

        graphics::draw(ctx, &mesh, dest)?;

        Ok(())
    }
}
