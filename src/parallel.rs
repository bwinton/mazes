use crate::util::{
    cell_from_pos, draw_board, draw_cell, draw_path, valid_move, Algorithm, ChooseRandom,
    Direction, State, COLORS, COLUMNS, LINE_WIDTH, ROWS,
};
use itertools::Itertools;
use maze_utils::From;
use std::collections::{HashSet, VecDeque};

use array_init::array_init;
use enumset::EnumSet;
use macroquad::{logging as log, rand::gen_range};

const MAX_SEEDS: usize = 6;

#[derive(From)]
pub struct Exports {
    path: Vec<(usize, usize)>,
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
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
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
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
            String::from("Backtrack")
        } else {
            String::from("Parallel Backtrack")
        }
    }
    fn re_init(&mut self, variant: String) {
        // log::info!("Re-initing with {}", variant);
        self.from(Exports::new(variant.parse().unwrap()));
    }
    fn get_variant(&self) -> String {
        self.seeds.to_string()
    }
    fn update(&mut self) {
        // log::info!("Updating {}", self.name());
        match self.state {
            State::Setup => {
                for (i, stack) in self.stack.iter_mut().take(self.seeds).enumerate() {
                    let x = gen_range(0, COLUMNS as usize);
                    let y = gen_range(0, ROWS as usize);
                    stack.push_front((x, y, EnumSet::all()));
                    self.sets[i].insert(i);
                }

                self.state = State::Running;
                return;
            }
            // State::Done => {}
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
                potentials.shuffle();
                let direction = potentials.pop().unwrap();
                // log::info!("({},{}) -> {:?}", x, y, direction);
                stack.push_front((x, y, directions ^ direction));

                let (new_x, new_y) = match direction {
                    Direction::North => (x as i32, y as i32 - 1),
                    Direction::East => (x as i32 + 1, y as i32),
                    Direction::South => (x as i32, y as i32 + 1),
                    Direction::West => (x as i32 - 1, y as i32),
                };
                // log::info!("{:?} / {:?} -> {:?}", (x,y), direction, (new_x, new_y));
                if 0 <= new_x && new_x < COLUMNS as i32 && 0 <= new_y && new_y < ROWS as i32 {
                    let (new_x, new_y) = (new_x as usize, new_y as usize);
                    if self.grid[new_y][new_x] == EnumSet::new()
                        && self.grid_seeds[new_y][new_x].is_none()
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
            self.path.push((0, 0));
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
                    draw_cell(*x, *y, LINE_WIDTH, curr_color);
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

    fn cell_from_pos(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        cell_from_pos(x, y)
    }

    fn move_to(&mut self, cursor: Option<(usize, usize)>) {
        if valid_move(self.path.last(), cursor, self.grid) {
            let cursor = cursor.unwrap();
            if let Some((index, _)) = self.path.iter().find_position(|&x| x == &cursor) {
                self.path.truncate(index + 1);
            } else {
                self.path.push(cursor);
            }
        }
    }
}
