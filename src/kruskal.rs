use crate::util::{draw_board, Algorithm, Direction, COLUMNS, ROWS, LINE_WIDTH};
use std::collections::HashSet;
use enumset::EnumSet;
use ggez::{graphics, Context, GameResult};
use ggez::graphics::DrawParam;

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

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mut builder = draw_board(self.grid)?;

        let mesh = builder.build(ctx)?;
        let dest = DrawParam::default().dest([LINE_WIDTH / 2.0, LINE_WIDTH / 2.0]);

        graphics::draw(ctx, &mesh, dest)?;

        Ok(())
    }
}