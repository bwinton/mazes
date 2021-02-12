use enumset::EnumSet;
use macroquad::prelude::{draw_line, draw_poly, Color};

use crate::util::{CELL_WIDTH, COLORS, LINE_WIDTH, OFFSET};

pub const ROWS: f32 = 19.0;
pub const COLUMNS: f32 = 40.0;

#[derive(EnumSetType, Debug)]
pub enum Direction {
    NorthEast,
    NorthWest,
    East,
    West,
    SouthEast,
    SouthWest,
}

impl Direction {
    #[allow(dead_code)]
    pub fn opposite(self) -> Self {
        match self {
            Direction::NorthEast => Direction::SouthWest,
            Direction::NorthWest => Direction::SouthEast,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
        }
    }
}

pub fn center_pixel(i: usize, j: usize) -> (f32, f32) {
    let i = i as f32;
    let j = j as f32;
    let mut x = f32::sqrt(3.0) * i + f32::sqrt(3.0) / 2.0 * j;
    x -= ROWS / 1.5;
    x *= CELL_WIDTH;
    x -= 10.0;
    let mut y = CELL_WIDTH * 3.0 / 2.0 * j;
    y += 12.0;
    (x - CELL_WIDTH + OFFSET, y + CELL_WIDTH + OFFSET)
}

fn pointy_hex_corner(x: f32, y: f32, i: usize, inset: f32) -> (f32, f32) {
    let angle = (60.0 * (i as f32) - 30.0).to_radians();
    (
        x + (CELL_WIDTH - inset) * angle.cos(),
        y + (CELL_WIDTH - inset) * angle.sin(),
    )
}

pub fn draw_cell(i: usize, j: usize, inset: f32, color: Color) {
    let (x, y) = center_pixel(i, j);
    // This totally feels like cheating…
    draw_poly(x, y, 6, CELL_WIDTH - inset, 90.0, color);
}

pub fn draw_board(grid: [[Option<EnumSet<Direction>>; COLUMNS as usize]; ROWS as usize]) {
    let mut printed_first = false;
    for (j, row) in grid.iter().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            if cell.is_none() {
                continue;
            }

            let cell = cell.unwrap();
            let (x, y) = center_pixel(i, j);

            let nw = pointy_hex_corner(x, y, 4, 0.0);
            let n = pointy_hex_corner(x, y, 5, 0.0);
            let ne = pointy_hex_corner(x, y, 0, 0.0);

            let se = pointy_hex_corner(x, y, 1, 0.0);
            let s = pointy_hex_corner(x, y, 2, 0.0);
            let sw = pointy_hex_corner(x, y, 3, 0.0);

            let skip_last = j + 1 == ROWS as usize && row[i + 1] == None;

            //Figure out which lines to draw.
            if !cell.contains(Direction::NorthEast) {
                draw_line(n.0, n.1, ne.0, ne.1, LINE_WIDTH, COLORS[0]);
            }
            if !cell.contains(Direction::East) && !skip_last {
                draw_line(ne.0, ne.1, se.0, se.1, LINE_WIDTH, COLORS[0]);
            }
            if !cell.contains(Direction::SouthEast) {
                draw_line(se.0, se.1, s.0, s.1, LINE_WIDTH, COLORS[0]);
            }
            if !cell.contains(Direction::SouthWest) {
                draw_line(s.0, s.1, sw.0, sw.1, LINE_WIDTH, COLORS[0]);
            }
            if !cell.contains(Direction::West) && printed_first {
                draw_line(sw.0, sw.1, nw.0, nw.1, LINE_WIDTH, COLORS[0]);
            }
            if !cell.contains(Direction::NorthWest) {
                draw_line(nw.0, nw.1, n.0, n.1, LINE_WIDTH, COLORS[0]);
            }
            printed_first = true;
        }
    }
}
