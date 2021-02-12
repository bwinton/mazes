use crate::util::{Algorithm, ChooseRandom, COLORS};
use crate::{
    hex_util::{draw_board, draw_cell, Direction, COLUMNS, ROWS},
    util::LINE_WIDTH,
};

use macroquad::{logging as log, rand::gen_range};

use maze_utils::From;
use std::collections::{HashSet, VecDeque};

use array_init::array_init;

use enumset::EnumSet;

const MAX_SEEDS: usize = 6;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}

#[derive(From)]
pub struct Exports {
    grid: [[Option<EnumSet<Direction>>; COLUMNS as usize]; ROWS as usize],
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
        let mut grid = [[Some(EnumSet::new()); COLUMNS as usize]; ROWS as usize];
        for (j, row) in grid.iter_mut().enumerate() {
            for (i, cell) in row.iter_mut().enumerate() {
                let x = i as f32;
                let y = j as f32;
                if (x < (ROWS - 1.0 - y) / 2.0) || (x > COLUMNS - (ROWS + y) / 2.0) {
                    *cell = None;
                }
            }
        }
        let grid_seeds = [[None; COLUMNS as usize]; ROWS as usize];
        let sets = array_init(|_| HashSet::new());
        let stack = array_init(|_| VecDeque::new());
        let state = State::Setup;
        Self {
            grid,
            grid_seeds,
            seeds,
            sets,
            stack,
            state,
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
        // println!("Updating {}", self.name());
        match self.state {
            State::Setup => {
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
            State::Done => {
                return;
            }
            _ => {}
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
                // println!("{}: ({},{}) -> {:?}", i, x, y, direction);
                stack.push_front((x, y, directions ^ direction));

                let (new_x, new_y) = match direction {
                    Direction::NorthEast => (x as i32 + 1, y as i32 - 1),
                    Direction::NorthWest => (x as i32, y as i32 - 1),
                    Direction::East => (x as i32 + 1, y as i32),
                    Direction::West => (x as i32 - 1, y as i32),
                    Direction::SouthEast => (x as i32, y as i32 + 1),
                    Direction::SouthWest => (x as i32 - 1, y as i32 + 1),
                };
                // println!("{}: ({},{}) / {:?} -> {:?}", i, x,y, direction, (new_x, new_y));
                if 0 <= new_x && new_x < COLUMNS as i32 && 0 <= new_y && new_y < ROWS as i32 {
                    let (new_x, new_y) = (new_x as usize, new_y as usize);
                    if self.grid[new_y][new_x] == Some(EnumSet::new())
                        && self.grid_seeds[new_y][new_x] == None
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
    }
}
