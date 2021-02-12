use enumset::EnumSet;
use macroquad::{
    color::Color,
    prelude::{color_u8, draw_line},
    rand::gen_range,
};

#[cfg(not(target_arch = "wasm32"))]
pub use crate::desktop_util::Desktop as RealArgs;

#[cfg(target_arch = "wasm32")]
pub use crate::web_util::Web as RealArgs;

pub const LINE_WIDTH: f32 = 2.0;
pub const CELL_WIDTH: f32 = 20.0;
pub const COLUMNS: f32 = 40.0;
pub const ROWS: f32 = 30.0;
pub const OFFSET: f32 = 2.0;

pub const EMPTY_COLOR: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 0.2,
};

pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};

pub const FIELD_COLOR: Color = Color {
    r: 0x4D as f32 / 255.0,
    g: 0xAF as f32 / 255.0,
    b: 0x4A as f32 / 255.0,
    a: 0.5,
};

lazy_static! {
    pub static ref COLORS: [Color; 61] = [
        color_u8!(0xB2, 0x18, 0x2B, 0xFF),
        color_u8!(0x37, 0x7E, 0xB8, 0xFF),
        color_u8!(0x4D, 0xAF, 0x4A, 0xFF),
        color_u8!(0x98, 0x4E, 0xA3, 0xFF),
        color_u8!(0xFF, 0x7F, 0x00, 0xFF),
        color_u8!(0xA6, 0x56, 0x28, 0xFF),
        color_u8!(0xF7, 0x81, 0xBF, 0xFF),
        color_u8!(0x99, 0x33, 0x00, 0xFF),
        color_u8!(0x33, 0x33, 0x00, 0xFF),
        color_u8!(0x00, 0x33, 0x00, 0xFF),
        color_u8!(0x00, 0x33, 0x66, 0xFF),
        color_u8!(0x00, 0x00, 0x80, 0xFF),
        color_u8!(0x33, 0x33, 0x99, 0xFF),
        color_u8!(0x33, 0x33, 0x33, 0xFF),
        color_u8!(0x80, 0x00, 0x00, 0xFF),
        color_u8!(0xFF, 0x66, 0x00, 0xFF),
        color_u8!(0x80, 0x80, 0x00, 0xFF),
        color_u8!(0x00, 0x80, 0x00, 0xFF),
        color_u8!(0x00, 0x80, 0x80, 0xFF),
        color_u8!(0x00, 0x00, 0xFF, 0xFF),
        color_u8!(0x66, 0x66, 0x99, 0xFF),
        color_u8!(0x80, 0x80, 0x80, 0xFF),
        color_u8!(0xFF, 0x00, 0x00, 0xFF),
        color_u8!(0xFF, 0x99, 0x00, 0xFF),
        color_u8!(0x99, 0xCC, 0x00, 0xFF),
        color_u8!(0x33, 0x99, 0x66, 0xFF),
        color_u8!(0x33, 0xCC, 0xCC, 0xFF),
        color_u8!(0x33, 0x66, 0xFF, 0xFF),
        color_u8!(0x80, 0x00, 0x80, 0xFF),
        color_u8!(0x96, 0x96, 0x96, 0xFF),
        color_u8!(0xFF, 0x00, 0xFF, 0xFF),
        color_u8!(0xFF, 0xCC, 0x00, 0xFF),
        color_u8!(0xFF, 0xFF, 0x00, 0xFF),
        color_u8!(0x00, 0xFF, 0x00, 0xFF),
        color_u8!(0x00, 0xFF, 0xFF, 0xFF),
        color_u8!(0x00, 0xCC, 0xFF, 0xFF),
        color_u8!(0x99, 0x33, 0x66, 0xFF),
        color_u8!(0xC0, 0xC0, 0xC0, 0xFF),
        color_u8!(0xFF, 0x99, 0xCC, 0xFF),
        color_u8!(0xFF, 0xCC, 0x99, 0xFF),
        color_u8!(0xFF, 0xFF, 0x99, 0xFF),
        color_u8!(0xCC, 0xFF, 0xCC, 0xFF),
        color_u8!(0xCC, 0xFF, 0xFF, 0xFF),
        color_u8!(0x99, 0xCC, 0xFF, 0xFF),
        color_u8!(0xCC, 0x99, 0xFF, 0xFF),
        color_u8!(0x99, 0x99, 0xFF, 0xFF),
        color_u8!(0x99, 0x33, 0x66, 0xFF),
        color_u8!(0xFF, 0xFF, 0xCC, 0xFF),
        color_u8!(0xCC, 0xFF, 0xFF, 0xFF),
        color_u8!(0x66, 0x00, 0x66, 0xFF),
        color_u8!(0xFF, 0x80, 0x80, 0xFF),
        color_u8!(0x00, 0x66, 0xCC, 0xFF),
        color_u8!(0xCC, 0xCC, 0xFF, 0xFF),
        color_u8!(0x00, 0x00, 0x80, 0xFF),
        color_u8!(0xFF, 0x00, 0xFF, 0xFF),
        color_u8!(0xFF, 0xFF, 0x00, 0xFF),
        color_u8!(0x00, 0xFF, 0xFF, 0xFF),
        color_u8!(0x80, 0x00, 0x80, 0xFF),
        color_u8!(0x80, 0x00, 0x00, 0xFF),
        color_u8!(0x00, 0x80, 0x80, 0xFF),
        color_u8!(0x00, 0x00, 0xFF, 0xFF),
    ];
}

pub trait Args {
    fn get_algorithm(&self) -> String;
    fn get_variant(&self) -> String;
}

pub trait Algorithm {
    fn name(&self) -> String;
    fn re_init(&mut self, variant: String);
    fn update(&mut self);
    fn draw(&self);
    fn get_variant(&self) -> String;
}

#[derive(EnumSetType, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

pub fn draw_board(grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize]) {
    for (j, row) in grid.iter().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            let x = i as f32;
            let y = j as f32;
            let north = y * CELL_WIDTH + OFFSET;
            let east = (x + 1.0) * CELL_WIDTH + OFFSET;
            let south = (y + 1.0) * CELL_WIDTH + OFFSET;
            let west = x * CELL_WIDTH + OFFSET;

            //Figure out which lines to draw.
            if !cell.contains(Direction::North) {
                draw_line(east, north, west, north, LINE_WIDTH, COLORS[0]);
            }
            if !cell.contains(Direction::East) && (x, y) != (COLUMNS - 1.0, ROWS - 1.0) {
                draw_line(east, north, east, south, LINE_WIDTH, COLORS[0]);
            }
            if !cell.contains(Direction::South) {
                draw_line(east, south, west, south, LINE_WIDTH, COLORS[0]);
            }
            if !cell.contains(Direction::West) && (x, y) != (0.0, 0.0) {
                draw_line(west, north, west, south, LINE_WIDTH, COLORS[0]);
            }
        }
    }
}

pub struct VecChooseIter<'a, T> {
    source: &'a Vec<T>,
    indices: std::vec::IntoIter<usize>,
}

impl<'a, T> Iterator for VecChooseIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.indices.next().map(|ix| &self.source[ix])
    }
}
pub trait ChooseRandom<T> {
    fn shuffle(&mut self);
    fn choose(&self) -> Option<T>;
    fn choose_multiple(&self, amount: usize) -> VecChooseIter<T>;
}

impl<T: Copy> ChooseRandom<T> for Vec<T> {
    fn shuffle(&mut self) {
        for i in (1..self.len()).rev() {
            let j = gen_range(0, i + 1);
            self.swap(i, j);
        }
    }

    fn choose(&self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let mut indices = (0..self.len()).collect::<Vec<usize>>();
        indices.shuffle();
        Some(self[indices[0]])
    }

    fn choose_multiple(&self, amount: usize) -> VecChooseIter<T> {
        let mut indices = (0..self.len())
            .enumerate()
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        indices.shuffle();
        indices.resize(amount, 0);

        VecChooseIter {
            source: self,
            indices: indices.into_iter(),
        }
    }
}
