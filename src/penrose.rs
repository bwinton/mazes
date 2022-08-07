use crate::util::{draw_board, Algorithm, Direction, COLUMNS, ROWS};
use maze_utils::From;

use enumset::EnumSet;
use macroquad::logging as log;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}
#[derive(From)]
pub struct Exports {
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    grid_seeds: [[Option<usize>; COLUMNS as usize]; ROWS as usize],
    state: State,
}

impl Exports {
    pub fn new() -> Self {
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let grid_seeds = [[None; COLUMNS as usize]; ROWS as usize];
        Self {
            grid,
            grid_seeds,
            state: State::Setup,
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Penrose")
    }
    fn re_init(&mut self, _variant: String) {
        self.from(Exports::new());
    }
    fn get_variant(&self) -> String {
        "unused".to_owned()
    }
    fn update(&mut self) {
        // println!("Updating {}", self.name());
        match self.state {
            State::Setup => {
                self.state = State::Running;
                return;
            }
            State::Done => {
                return;
            }
            _ => {}
        }

        let done = true;

        if done {
            self.state = State::Done;
            log::info!("Done!");
        }
    }

    fn draw(&self) {
        draw_board(self.grid);

        if self.state == State::Running {}
    }
}
