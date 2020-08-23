use crate::util::{draw_board, Algorithm, Direction, COLUMNS, ROWS};
use enumset::EnumSet;
use quicksilver::{graphics::Graphics, log, Result};
use rand::{rngs::ThreadRng, thread_rng};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}

pub struct Exports {
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    rng: ThreadRng,
    state: State,
    variant: String,
}

impl Exports {
    pub fn new(variant: String) -> Self {
        println!("variant: {}", variant);
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let rng = thread_rng();
        let state = State::Setup;
        Self {
            grid,
            rng,
            state,
            variant,
        }
    }
    fn from(&mut self, other: Self) {
        self.grid = other.grid;
        self.rng = other.rng;
        self.state = other.state;
        self.variant = other.variant;
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Growing Tree")
    }
    fn re_init(&mut self, variant: String) {
        self.from(Exports::new(variant));
    }
    fn get_variant(&self) -> String {
        self.variant.clone()
    }
    fn update(&mut self) {
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

        let mut found = false;

        while !found {
            if true {
                self.state = State::Done;
                log::info!("Done!");
                return;
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
