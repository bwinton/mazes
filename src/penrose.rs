use std::fmt::Display;

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

#[derive(Debug)]
enum Variant {
    Sun,
    Star,
    Ace,
    Deuce,
    Jack,
    Queen,
    King,
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::Sun => f.write_str("Sun"),
            Variant::Star => f.write_str("Star"),
            Variant::Ace => f.write_str("Ace"),
            Variant::Deuce => f.write_str("Deuce"),
            Variant::Jack => f.write_str("Jack"),
            Variant::Queen => f.write_str("Queen"),
            Variant::King => f.write_str("King"),
        }
    }
}

#[derive(From)]
pub struct Exports {
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    grid_seeds: [[Option<usize>; COLUMNS as usize]; ROWS as usize],
    state: State,
    variant: Variant,
}

impl Exports {
    pub fn new(variant: String) -> Self {
        let variant = match variant.as_str() {
            "sun" => Variant::Sun,
            "star" => Variant::Star,
            "ace" => Variant::Ace,
            "deuce" => Variant::Deuce,
            "jack" => Variant::Jack,
            "queen" => Variant::Queen,
            "king" => Variant::King,
            _ => panic!("Unknown Variant \"{}\"!", variant),
        };
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let grid_seeds = [[None; COLUMNS as usize]; ROWS as usize];
        Self {
            grid,
            grid_seeds,
            state: State::Setup,
            variant,
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        format!("Penrose {}", self.variant)
    }
    fn re_init(&mut self, variant: String) {
        self.from(Exports::new(variant));
    }
    fn get_variant(&self) -> String {
        self.variant.to_string().to_lowercase()
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
