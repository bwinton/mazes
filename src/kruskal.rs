use crate::util::{Algorithm, Direction, COLUMNS, ROWS};
use std::collections::HashSet;
use enumset::EnumSet;
use ggez::{Context, GameResult};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}

pub struct Exports<'a> {
    sets: Vec<HashSet<usize>>,
    grid: [[(Option<&'a HashSet<usize>>, EnumSet<Direction>); COLUMNS as usize]; ROWS as usize],
    state: State,
}

impl<'a> Exports<'a> {
    pub fn new() -> Self {
        let sets = vec![];
        let grid = [[(None, EnumSet::new()); COLUMNS as usize]; ROWS as usize];
        let state = State::Setup;
        Self {
            sets,
            grid,
            state,
        }
    }
}

impl<'a> Algorithm for Exports<'a> {
    fn name(&self) -> String {
        String::from("Kruskal")
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                self.state = State::Running;
            }
            State::Done => {}
            State::Running => {
                self.state = State::Done;
            }
        }
    }

    fn draw(&self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}