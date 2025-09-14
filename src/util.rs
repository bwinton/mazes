use enumset::EnumSet;
use itertools::Itertools;
use macroquad::{
    color::Color,
    math::vec2,
    prelude::{color_u8, draw_line, draw_rectangle, ImageFormat},
    rand::gen_range,
    shapes::draw_rectangle_lines,
    texture::{draw_texture_ex, DrawTextureParams, Texture2D},
};

#[cfg(not(target_arch = "wasm32"))]
pub use crate::desktop_util::Desktop as RealArgs;

#[cfg(target_arch = "wasm32")]
pub use crate::web_util::Web as RealArgs;

pub const LINE_WIDTH: f32 = 2.0;
pub const CELL_WIDTH: f32 = 20.0;
pub const COLUMNS: f32 = 40.0;
pub const ROWS: f32 = 30.0;
pub const OFFSET: f32 = 8.0;

pub type Grid = [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize];

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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum State {
    Setup,
    Running,
    Done,
}

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
    fn needs_reset(&self) -> bool;
}

pub trait Algorithm {
    fn name(&self) -> String;
    fn re_init(&mut self, variant: String);
    fn update(&mut self);
    fn draw(&self);
    fn get_variant(&self) -> String;
    fn get_state(&self) -> State;
    fn move_to(&mut self, cursor: (f32, f32));
}

pub trait Playable: Algorithm {
    fn get_grid(&self) -> Grid;
    fn get_path_mut(&mut self) -> &mut Vec<(usize, usize)>;
    fn cell_from_pos(&self, pos: (f32, f32)) -> Option<(usize, usize)> {
        cell_from_pos(pos)
    }
    fn move_to(&mut self, pos: (f32, f32)) {
        let cursor = self.cell_from_pos(pos);

        let grid = self.get_grid();
        let path = self.get_path_mut();
        if let Some(moves) = valid_move(path.last(), cursor, grid) {
            for cursor in moves {
                if let Some((index, _)) = path.iter().find_position(|&x| x == &cursor) {
                    path.truncate(index + 1);
                } else {
                    path.push(cursor);
                }
            }
        }
    }
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

    pub fn offset(self, start: (usize, usize)) -> Option<(usize, usize)> {
        let mut new_x = start.0 as isize;
        let mut new_y = start.1 as isize;

        match self {
            Direction::North => new_y -= 1,
            Direction::East => new_x += 1,
            Direction::South => new_y += 1,
            Direction::West => new_x -= 1,
        };

        if 0 <= new_x && new_x < COLUMNS as isize && 0 <= new_y && new_y < ROWS as isize {
            Some((new_x as usize, new_y as usize))
        } else {
            None
        }
    }
}

pub fn cell_from_pos(pos: (f32, f32)) -> Option<(usize, usize)> {
    let (x, y) = pos;
    if x < 0.0 || y < 0.0 {
        return None;
    }
    let x = ((x - OFFSET) / CELL_WIDTH) as usize;
    let y = ((y - OFFSET) / CELL_WIDTH) as usize;
    if x >= COLUMNS as usize || y >= ROWS as usize {
        return None;
    }
    Some((x, y))
}

pub fn valid_move(
    start: Option<&(usize, usize)>,
    next: Option<(usize, usize)>,
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
) -> Option<Vec<(usize, usize)>> {
    if let Some(&(x1, y1)) = start {
        if let Some((x2, y2)) = next {
            print!(
                "Moving ({}, {})",
                x2 as i32 - x1 as i32,
                y2 as i32 - y1 as i32
            );
            let delta = (x2 as i32 - x1 as i32, y2 as i32 - y1 as i32);
            let direction = match delta {
                (-1, y) if y < 0 => (Some(Direction::North), Some(Direction::West)),
                (0, y) if y < 0 => (Some(Direction::North), None),
                (1, y) if y < 0 => (Some(Direction::North), Some(Direction::East)),

                (-1, y) if y > 0 => (Some(Direction::South), Some(Direction::West)),
                (0, y) if y > 0 => (Some(Direction::South), None),
                (1, y) if y > 0 => (Some(Direction::South), Some(Direction::East)),

                (x, -1) if x < 0 => (Some(Direction::West), Some(Direction::North)),
                (x, 0) if x < 0 => (Some(Direction::West), None),
                (x, 1) if x < 0 => (Some(Direction::West), Some(Direction::South)),

                (x, -1) if x > 0 => (Some(Direction::East), Some(Direction::North)),
                (x, 0) if x > 0 => (Some(Direction::East), None),
                (x, 1) if x > 0 => (Some(Direction::East), Some(Direction::South)),

                _ => (None, None),
            };
            println!(" => {:?}", direction);
            let sideways = direction.1;
            if let Some(direction) = direction.0 {
                let mut start = *start.unwrap();
                let next = next.unwrap();
                let mut rv = vec![];
                while start != next {
                    if let Some(sideways) = sideways {
                        if grid[start.1][start.0].contains(sideways)
                            && sideways.offset(start).unwrap() == next
                        {
                            break;
                        }
                    }
                    if grid[start.1][start.0].contains(direction) {
                        start = direction.offset(start).unwrap();
                        rv.push(start);
                    } else {
                        return None;
                    }
                }
                rv.push(next);
                return Some(rv);
            }
            // If we haven't found it, maybe we went the wrong way at the start…
            let direction = match delta {
                (-1, -1) => Some((Direction::East, Direction::North)),
                (-1, 1) => Some((Direction::East, Direction::South)),
                (1, -1) => Some((Direction::West, Direction::North)),
                (1, 1) => Some((Direction::West, Direction::South)),
                _ => None,
            };
            if let Some((first, second)) = direction {
                let start = *start.unwrap();
                let mut rv = vec![];

                if grid[start.1][start.0].contains(first) {
                    rv.push(start);
                    let start = first.offset(start).unwrap();
                    if grid[start.1][start.0].contains(second) {
                        rv.push(start);
                        return Some(rv);
                    }
                }
            }
        }
    }
    None
}

fn draw_little_robot(x: usize, y: usize, color: Color) {
    let x = x as f32 * CELL_WIDTH + OFFSET;
    let y = y as f32 * CELL_WIDTH + OFFSET;
    draw_rectangle_lines(x, y, CELL_WIDTH, CELL_WIDTH, 4.0, color);

    let inset = 2.0;
    let x = x + inset;
    let y = y + inset;
    let w = CELL_WIDTH - inset * 2.0;
    let h = CELL_WIDTH - inset * 2.0;

    let image = Texture2D::from_file_with_format(
        include_bytes!("../static/little_guy.png"),
        Some(ImageFormat::Png),
    );
    draw_texture_ex(
        &image,
        x,
        y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(w, h)),
            ..Default::default()
        },
    );
}

pub fn draw_cell(x: usize, y: usize, inset: f32, color: Color) {
    draw_rectangle(
        x as f32 * CELL_WIDTH + inset + OFFSET,
        y as f32 * CELL_WIDTH + inset + OFFSET,
        CELL_WIDTH - inset * 2.0,
        CELL_WIDTH - inset * 2.0,
        color,
    );
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

pub fn draw_path(path: &[(usize, usize)]) {
    let mut color = COLORS[10];
    if let Some((&(x, y), rest)) = path.split_last() {
        color.a = 0.6;
        draw_little_robot(x, y, color);
        color.a = 0.3;
        for &(x, y) in rest {
            draw_cell(x, y, 0.0, color)
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
    fn choose_multiple(&'_ self, amount: usize) -> VecChooseIter<'_, T>;
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

    fn choose_multiple(&'_ self, amount: usize) -> VecChooseIter<'_, T> {
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
