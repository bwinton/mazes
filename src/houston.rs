use crate::aldous_broder::Exports as aldous_broder;
use crate::util::Algorithm;
use crate::wilson::Exports as wilson;
use quicksilver::{graphics::Graphics, log, Result};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    RunningAldousBroder,
    RunningWilson,
    Done,
}

pub struct Exports {
    aldous_broder: aldous_broder,
    state: State,
    wilson: wilson,
}

impl Exports {
    pub fn new() -> Self {
        let aldous_broder = aldous_broder::new(true);
        let wilson = wilson::new(false);
        let state = State::Setup;
        Self {
            aldous_broder,
            state,
            wilson,
        }
    }
    fn from(&mut self, other: Self) {
        self.aldous_broder = other.aldous_broder;
        self.state = other.state;
        self.wilson = other.wilson;
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Houston")
    }
    fn re_init(&mut self) {
        self.from(Exports::new());
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                self.state = State::RunningAldousBroder;
                log::info!("Starting with Aldous-Broder!");
            }
            State::RunningAldousBroder => {
                self.aldous_broder.update();
                if self.aldous_broder.filled() > 0.3 {
                    log::info!("Switching to Wilson!");
                    self.wilson.init_from_grid(self.aldous_broder.get_grid());
                    self.state = State::RunningWilson;
                }
            }
            State::RunningWilson => {
                self.wilson.update();
                if self.wilson.is_done() {
                    log::info!("Done!");
                    self.state = State::Done;
                }
            }
            State::Done => {}
        }
    }

    fn draw(&self, gfx: &mut Graphics) -> Result<()> {
        match self.state {
            State::RunningAldousBroder => self.aldous_broder.draw(gfx),
            State::RunningWilson | State::Done => self.wilson.draw(gfx),
            _ => Ok(()),
        }
    }
}
