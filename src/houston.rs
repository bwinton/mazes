use crate::aldous_broder::Exports as aldous_broder;
use crate::util::{cell_from_pos, Algorithm, State as BaseState};
use crate::wilson::Exports as wilson;
use macroquad::logging as log;
use maze_utils::From;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    RunningAldousBroder,
    RunningWilson,
    Done,
}

#[derive(From)]
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
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Houston")
    }
    fn re_init(&mut self, _variant: String) {
        self.from(Exports::new());
    }
    fn get_variant(&self) -> String {
        "unused".to_owned()
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
            _ => {}
        }
    }

    fn draw(&self) {
        match self.state {
            State::RunningAldousBroder => self.aldous_broder.draw(),
            State::RunningWilson | State::Done => self.wilson.draw(),
            _ => {}
        }
    }

    fn get_state(&self) -> BaseState {
        match &self.state {
            State::Setup => BaseState::Setup,
            State::Done => BaseState::Done,
            _ => BaseState::Running,
        }
    }

    fn cell_from_pos(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        cell_from_pos(x, y)
    }
}
