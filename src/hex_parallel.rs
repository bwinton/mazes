use crate::{
    hex_util::{
        draw_board, draw_cell, draw_path, init_grid, Direction, Grid, Playable, COLUMNS, ROWS,
    },
    util::{Algorithm, ChooseRandom, State, COLORS, LINE_WIDTH},
};

use itertools::Itertools;
use macroquad::{logging as log, rand::gen_range};

use maze_utils::From;
use std::collections::{HashSet, VecDeque};

use array_init::array_init;

use enumset::EnumSet;

const MAX_SEEDS: usize = 6;

#[derive(From)]
pub struct Exports {
    path: Vec<(usize, usize)>,
    grid: Grid,
    grid_seeds: [[Option<usize>; COLUMNS as usize]; ROWS as usize],
    seeds: usize,
    sets: [HashSet<usize>; MAX_SEEDS],
    stack: [VecDeque<(usize, usize, EnumSet<Direction>)>; MAX_SEEDS],
    state: State,
}

impl Exports {
    pub fn new(seeds: usize) -> Self {
        if !(1..=MAX_SEEDS).contains(&seeds) {
            panic!("Seeds {} must be between {} and {}", seeds, 1, MAX_SEEDS);
        }
        let grid = init_grid(EnumSet::new());
        let grid_seeds = [[None; COLUMNS as usize]; ROWS as usize];
        let sets = array_init(|_| HashSet::new());
        let stack = array_init(|_| VecDeque::new());
        Self {
            path: vec![],
            grid,
            grid_seeds,
            seeds,
            sets,
            stack,
            state: State::Setup,
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        if self.seeds == 1 {
            String::from("Hex Backtrack")
        } else {
            String::from("Parallel Hex Backtrack")
        }
    }
    fn re_init(&mut self, variant: String) {
        self.from(Exports::new(variant.parse().unwrap()));
    }
    fn get_variant(&self) -> String {
        self.seeds.to_string()
    }
    fn update(&mut self) {
        // log::info!("Updating {}", self.name());
        if self.state == State::Setup {
            for (i, stack) in self.stack.iter_mut().take(self.seeds).enumerate() {
                let mut pushed = false;
                while !pushed {
                    let x = gen_range(0, COLUMNS as usize);
                    let y = gen_range(0, ROWS as usize);
                    if self.grid[y][x].is_none() {
                        continue;
                    }
                    stack.push_front((x, y, EnumSet::all()));
                    pushed = true;
                }
                self.sets[i].insert(i);
            }

            self.state = State::Running;
            return;
        }

        let mut done = true;

        'outer: for (i, stack) in self.stack.iter_mut().take(self.seeds).enumerate() {
            let mut found = false;

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
                potentials.shuffle();
                let direction = potentials.pop().unwrap();
                // log::info!("{}: ({},{}) -> {:?}", i, x, y, direction);
                stack.push_front((x, y, directions ^ direction));

                let (new_x, new_y) = direction.next(x as i32, y as i32);
                // log::info!("{}: ({},{}) / {:?} -> {:?}", i, x,y, direction, (new_x, new_y));
                if 0 <= new_x && new_x < COLUMNS as i32 && 0 <= new_y && new_y < ROWS as i32 {
                    let (new_x, new_y) = (new_x as usize, new_y as usize);
                    if self.grid[new_y][new_x] == Some(EnumSet::new())
                        && self.grid_seeds[new_y][new_x].is_none()
                    {
                        self.grid_seeds[y][x] = Some(i);
                        self.grid[y][x] = self.grid[y][x].map(|cell| cell | direction);
                        self.grid_seeds[new_y][new_x] = Some(i);
                        self.grid[new_y][new_x] =
                            self.grid[new_y][new_x].map(|cell| cell | direction.opposite());
                        stack.push_front((new_x, new_y, EnumSet::all() ^ direction.opposite()));
                        found = true;
                    } else if let Some(set) = self.grid_seeds[new_y][new_x] {
                        if !self.sets[i].contains(&set) {
                            let both_sets: HashSet<usize> =
                                self.sets[i].union(&self.sets[set]).cloned().collect();
                            for i in &both_sets {
                                self.sets[*i] = both_sets.clone();
                            }
                            self.grid[y][x] = self.grid[y][x].map(|cell| cell | direction);
                            self.grid[new_y][new_x] =
                                self.grid[new_y][new_x].map(|cell| cell | direction.opposite());
                        }
                    }
                    // Otherwise, loop again and see what we can get.
                }
            }
        }
        if done {
            let (first, _) = self.grid[0]
                .iter()
                .find_position(|&&x| x.is_some())
                .unwrap();
            self.path.push((first, 0));
            self.state = State::Done;
            log::info!("Done!");
        }
    }

    fn draw(&self) {
        draw_board(self.grid);

        for i in 0..self.seeds {
            let curr_color = COLORS[i + 1];
            let mut cell_color = COLORS[i + 1];
            cell_color.a = 0.5;
            for (i, (x, y, _)) in self.stack[i].iter().enumerate() {
                if i == 0 {
                    draw_cell(*x, *y, LINE_WIDTH * 1.5, curr_color);
                } else {
                    draw_cell(*x, *y, 0.0, cell_color);
                }
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
