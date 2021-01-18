use enumset::EnumSet;

use quicksilver::{
    geom::Vector,
    graphics::{Color, Element, Mesh},
    Graphics, Result,
};

use crate::util::{push_vertex, CELL_WIDTH, OFFSET};

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

fn pointy_hex_corner(x: f32, y: f32, i: usize, inset: f32) -> Vector {
    let angle = (60.0 * (i as f32) - 30.0).to_radians();
    Vector::new(
        x + (CELL_WIDTH - inset) * angle.cos(),
        y + (CELL_WIDTH - inset) * angle.sin(),
    )
}

pub fn draw_cell(i: usize, j: usize, inset: f32, gfx: &mut Graphics, color: Color) {
    let (x, y) = center_pixel(i, j);

    let mut points: Vec<Vector> = vec![];
    for i in 0..6 {
        points.push(pointy_hex_corner(x, y, i, inset));
    }
    gfx.fill_polygon(&points, color);
}

pub fn draw_board(
    grid: [[Option<EnumSet<Direction>>; COLUMNS as usize]; ROWS as usize],
) -> Result<Mesh> {
    let mut vertices = vec![];
    let mut elements = vec![];
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

            let n = push_vertex(n, &mut vertices);
            let ne = push_vertex(ne, &mut vertices);
            let nw = push_vertex(nw, &mut vertices);
            let s = push_vertex(s, &mut vertices);
            let se = push_vertex(se, &mut vertices);
            let sw = push_vertex(sw, &mut vertices);

            let skip_last = j + 1 == ROWS as usize && row[i + 1] == None;

            //Figure out which lines to draw.
            if !cell.contains(Direction::NorthEast) {
                elements.push(Element::Line([n, ne]));
            }
            if !cell.contains(Direction::East) && !skip_last {
                elements.push(Element::Line([ne, se]));
            }
            if !cell.contains(Direction::SouthEast) {
                elements.push(Element::Line([se, s]));
            }
            if !cell.contains(Direction::SouthWest) {
                elements.push(Element::Line([s, sw]));
            }
            if !cell.contains(Direction::West) && printed_first {
                elements.push(Element::Line([sw, nw]));
            }
            if !cell.contains(Direction::NorthWest) {
                elements.push(Element::Line([nw, n]));
            }
            printed_first = true;
        }
    }
    Ok(Mesh {
        vertices,
        elements,
        image: None,
    })
}
