use crate::util::{
    cell_from_pos, draw_board, Algorithm, ChooseRandom, Direction, State, CELL_WIDTH, COLORS,
    COLUMNS, FIELD_COLOR, LINE_WIDTH, OFFSET, ROWS,
};
use maze_utils::From;
use std::collections::VecDeque;

use enumset::EnumSet;
use macroquad::{logging as log, prelude::draw_rectangle, rand::gen_range};

#[derive(Debug)]
enum Variant {
    Newest,
    Middle,
    Oldest,
    Random,
}

#[derive(From)]
pub struct Exports {
    path: Vec<(usize, usize)>,
    curr: Option<(usize, usize)>,
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    stack: VecDeque<(usize, usize)>,
    state: State,
    variant: Variant,
}

impl Exports {
    pub fn new(variant: String) -> Self {
        let variant = match variant.as_str() {
            "newest" => Variant::Newest,
            "middle" => Variant::Middle,
            "oldest" => Variant::Oldest,
            "random" => Variant::Random,
            _ => panic!("Unknown Variant \"{}\"!", variant),
        };
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];

        Self {
            path: vec![],
            curr: None,
            grid,
            stack: VecDeque::new(),
            state: State::Setup,
            variant,
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        match self.variant {
            Variant::Newest => String::from("Newest Growing Tree"),
            Variant::Middle => String::from("Middle Growing Tree"),
            Variant::Oldest => String::from("Oldest Growing Tree"),
            Variant::Random => String::from("Random Growing Tree"),
        }
    }
    fn re_init(&mut self, variant: String) {
        self.from(Exports::new(variant));
    }
    fn get_variant(&self) -> String {
        match self.variant {
            Variant::Newest => "newest".to_owned(),
            Variant::Middle => "middle".to_owned(),
            Variant::Oldest => "oldest".to_owned(),
            Variant::Random => "random".to_owned(),
        }
    }
    fn update(&mut self) {
        // log::info!("Updating {}", self.name());
        match self.state {
            State::Setup => {
                self.stack
                    .push_front((gen_range(0, COLUMNS as usize), gen_range(0, ROWS as usize)));
                self.state = State::Running;
                return;
            }
            _ => {}
        }

        if self.stack.is_empty() {
            self.state = State::Done;
            log::info!("Done!");
            return;
        }

        // @todo: This is where I need to change stuff!!!
        let index = match self.variant {
            Variant::Newest => 0,
            Variant::Middle => (self.stack.len() - 1) / 2,
            Variant::Oldest => self.stack.len() - 1,
            Variant::Random => gen_range(0, self.stack.len()),
        };

        let (x, y) = self.stack[index];
        self.curr = Some((x, y));
        let directions = self.grid[y][x].complement();
        // log::info!("{:?}[{}] => ({},{}) {:?}", self.stack, self.index, x, y, directions);
        let potentials: Vec<(usize, usize, Direction)> = directions
            .iter()
            .filter_map(|direction| {
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
                        && !self.stack.contains(&(new_x, new_y))
                    {
                        Some((new_x, new_y, direction))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // log::info!("{:?} => ({},{}) Potentials: {:?}", self.index, x, y, potentials);
        if potentials.is_empty() {
            self.stack.remove(index);
            return;
        }
        let (new_x, new_y, direction) = potentials.choose().unwrap().to_owned();
        self.grid[y][x] |= direction;
        self.grid[new_y][new_x] |= direction.opposite();
        // log::info!("  pushing ({},{})", new_x, new_y);
        self.stack.push_front((new_x, new_y));
        self.curr = Some((new_x, new_y));
    }

    fn draw(&self) {
        // Draw code here...
        draw_board(self.grid);

        let curr_color = COLORS[1];
        let mut cell_color = COLORS[1];
        cell_color.a = 0.5;
        for x in 0..COLUMNS as usize {
            for y in 0..ROWS as usize {
                if self.grid[y][x] == EnumSet::new() {
                    draw_rectangle(
                        x as f32 * CELL_WIDTH + OFFSET,
                        y as f32 * CELL_WIDTH + OFFSET,
                        CELL_WIDTH,
                        CELL_WIDTH,
                        FIELD_COLOR,
                    );
                }
            }
        }
        for (x, y) in self.stack.iter() {
            if Some((*x, *y)) != self.curr {
                draw_rectangle(
                    *x as f32 * CELL_WIDTH + OFFSET,
                    *y as f32 * CELL_WIDTH + OFFSET,
                    CELL_WIDTH,
                    CELL_WIDTH,
                    cell_color,
                );
            }
        }
        if let Some((x, y)) = self.curr {
            draw_rectangle(
                x as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                y as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                CELL_WIDTH - LINE_WIDTH * 2.0,
                CELL_WIDTH - LINE_WIDTH * 2.0,
                curr_color,
            );
        }
    }

    fn get_state(&self) -> State {
        self.state
    }

    fn cell_from_pos(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        cell_from_pos(x, y)
    }
}
