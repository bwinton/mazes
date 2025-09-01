use enumset::EnumSet;
use itertools::Itertools;
use macroquad::prelude::{draw_line, draw_poly, Color};

use crate::util::{COLORS, LINE_WIDTH, OFFSET};

pub use crate::util::Algorithm;

pub type Grid = [[Option<EnumSet<Direction>>; COLUMNS as usize]; ROWS as usize];

pub const CELL_WIDTH: f32 = 12.0;
pub const ROWS: f32 = 32.0;
pub const COLUMNS: f32 = 68.0;

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

    pub fn next(self, x: i32, y: i32) -> (i32, i32) {
        match self {
            Direction::NorthEast => (x + 1, y - 1),
            Direction::NorthWest => (x, y - 1),
            Direction::East => (x + 1, y),
            Direction::West => (x - 1, y),
            Direction::SouthEast => (x, y + 1),
            Direction::SouthWest => (x - 1, y + 1),
        }
    }
}

pub trait Playable: Algorithm {
    fn get_grid(&self) -> [[Option<EnumSet<Direction>>; COLUMNS as usize]; ROWS as usize];
    fn get_path_mut(&mut self) -> &mut Vec<(usize, usize)>;
    fn cell_from_pos(&self, pos: (f32, f32)) -> Option<(usize, usize)> {
        let (x, y) = pos;
        cell_from_pos(x, y, self.get_grid())
    }
    fn move_to(&mut self, pos: (f32, f32)) {
        let cursor = self.cell_from_pos(pos);

        let grid = self.get_grid();
        let path = self.get_path_mut();
        if valid_move(path.last(), cursor, grid) {
            let cursor = cursor.unwrap();
            if let Some((index, _)) = path.iter().find_position(|&x| x == &cursor) {
                path.truncate(index + 1);
            } else {
                path.push(cursor);
            }
        }
    }
}

pub fn init_grid<T: Copy>(value: T) -> [[Option<T>; COLUMNS as usize]; ROWS as usize] {
    let mut grid = [[Some(value); COLUMNS as usize]; ROWS as usize];

    for (j, row) in grid.iter_mut().enumerate() {
        for (i, cell) in row.iter_mut().enumerate() {
            let x = i as f32;
            let y = j as f32;
            if (x < (ROWS - 1.0 - y) / 2.0) || (x > COLUMNS - (ROWS + y) / 2.0) {
                *cell = None;
            }
        }
    }

    grid
}

pub fn set_border(grid: &mut [[Option<EnumSet<Direction>>; COLUMNS as usize]; ROWS as usize]) {
    // Make sure the border is filled in.
    for (j, row) in grid.iter_mut().enumerate() {
        for (i, cell) in row.iter_mut().enumerate() {
            if let Some(cell) = cell {
                if j == 0 {
                    cell.remove(Direction::NorthEast);
                    cell.remove(Direction::NorthWest);
                } else if j == ROWS as usize - 1 {
                    cell.remove(Direction::SouthEast);
                    cell.remove(Direction::SouthWest);
                }
                if i >= COLUMNS as usize - (ROWS as usize + j).div_ceil(2) {
                    cell.remove(Direction::East);
                    if j % 2 == 0 {
                        cell.remove(Direction::NorthEast);
                        cell.remove(Direction::SouthEast);
                    }
                } else if i <= (ROWS as usize - j) / 2 {
                    cell.remove(Direction::West);
                    if j % 2 == 1 {
                        cell.remove(Direction::NorthWest);
                        cell.remove(Direction::SouthWest);
                    }
                }
            }
        }
    }
}

pub fn center_pixel(i: usize, j: usize) -> (f32, f32) {
    let i = i as f32;
    let j = j as f32;
    let sqrt_3 = f32::sqrt(3.0);
    let mut x = CELL_WIDTH * (sqrt_3 * i + sqrt_3 / 2.0 * j);
    x -= 292.0;

    let mut y = CELL_WIDTH * 3.0 / 2.0 * j;
    y += 12.0;
    (x - CELL_WIDTH + OFFSET, y + CELL_WIDTH + OFFSET)
}

pub fn pointy_hex_corner(x: f32, y: f32, i: usize, inset: f32) -> (f32, f32) {
    let angle = (60.0 * (i as f32) - 30.0).to_radians();
    (
        x + (CELL_WIDTH - inset) * angle.cos(),
        y + (CELL_WIDTH - inset) * angle.sin(),
    )
}

fn hex_round(x: f32, y: f32) -> (usize, usize) {
    let z = -x - y;
    let mut rx = f32::round(x);
    let mut ry = f32::round(y);
    let rz = f32::round(z);

    let x_diff = f32::abs(rx - x);
    let y_diff = f32::abs(ry - y);
    let z_diff = f32::abs(rz - z);

    if x_diff > y_diff && x_diff > z_diff {
        rx = -ry - rz;
    } else if y_diff > z_diff {
        ry = -rx - rz;
    }

    (rx as usize, ry as usize)
}

pub fn cell_from_pos(x: f32, y: f32, grid: Grid) -> Option<(usize, usize)> {
    if x < 0.0 || y < 0.0 {
        return None;
    }

    let mut x = x;
    x += CELL_WIDTH + OFFSET;
    x += 292.0;

    let mut y = y;
    y -= CELL_WIDTH + OFFSET;
    y -= 12.0;

    let sqrt_3 = f32::sqrt(3.0);
    let q = (sqrt_3 / 3.0 * x - 1.0 / 3.0 * y) / CELL_WIDTH;
    let r = (2.0 / 3.0 * y) / CELL_WIDTH;
    let (x, y) = hex_round(q, r);

    if x >= COLUMNS as usize || y >= ROWS as usize {
        return None;
    }
    grid[y][x]?;
    Some((x, y))
}

pub fn draw_path(path: &[(usize, usize)]) {
    let mut color = COLORS[10];
    if let Some((&(x, y), rest)) = path.split_last() {
        draw_cell(x, y, 3.0, color);
        color.a = 0.5;
        for &(x, y) in rest {
            draw_cell(x, y, 0.0, color)
        }
    }
}

pub fn valid_move(
    start: Option<&(usize, usize)>,
    next: Option<(usize, usize)>,
    grid: [[Option<EnumSet<Direction>>; COLUMNS as usize]; ROWS as usize],
) -> bool {
    if let Some(&(x1, y1)) = start {
        if let Some((x2, y2)) = next {
            if let Some(cell) = grid[y1][x1] {
                for direction in cell {
                    if direction.next(x1 as i32, y1 as i32) == (x2 as i32, y2 as i32) {
                        return true;
                    }
                }
            }
        }
    }
    false
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

            let skip_last = j + 1 == ROWS as usize && row[i + 1].is_none();

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

#[test]
fn a() {
    let grid = init_grid(EnumSet::new());
    let (i, j) = (16, 0);
    let (x, y) = center_pixel(i, j);

    assert_eq!(cell_from_pos(x, y, grid), Some((i, j)));

    let (i, j) = (15, 0);
    let (x, y) = center_pixel(i, j);

    assert_eq!(cell_from_pos(x, y, grid), None);
}
