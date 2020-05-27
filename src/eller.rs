use crate::util::Algorithm;
use ggez::{Context, GameResult};

pub struct Exports {}

impl Exports {
    pub fn new() -> Self {
        Self {}
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Eller")
    }
    fn update(&mut self) {}

    fn draw(&self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
}
