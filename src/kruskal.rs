use crate::util::{draw_board, Algorithm, Direction, COLUMNS, LINE_WIDTH, ROWS};
use enumset::EnumSet;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Graphics,
    log, Result,
};
use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}

pub struct Exports<'a> {
    sets: Vec<HashSet<usize>>,
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    grid_sets: [[Option<&'a HashSet<usize>>; COLUMNS as usize]; ROWS as usize],
    state: State,
}

impl<'a> Exports<'a> {
    pub fn new() -> Self {
        let sets = vec![];
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let grid_sets = [[None; COLUMNS as usize]; ROWS as usize];
        let state = State::Setup;
        Self {
            sets,
            grid,
            grid_sets,
            state,
        }
    }
    fn from(&mut self, other: Self) {
        self.sets = other.sets;
        self.grid = other.grid;
        self.grid_sets = other.grid_sets;
        self.state = other.state;
    }
}

impl<'a> Algorithm for Exports<'a> {
    fn name(&self) -> String {
        String::from("Kruskal")
    }
    fn re_init(&mut self) {
        self.from(Exports::new());
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                self.state = State::Running;
            }
            State::Done => {}
            State::Running => {
                self.state = State::Done;
                log::info!("Done!");
            }
        }
    }

    fn draw(&self, gfx: &mut Graphics) -> Result<()> {
        let elements = draw_board(self.grid)?;
        gfx.draw_mesh(&elements);

        Ok(())
    }
}
