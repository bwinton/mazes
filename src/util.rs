use enumset::EnumSet;

// use ggez::{Context, GameResult};
// use ggez::graphics::{
//     Color, DrawMode, LineCap, MeshBuilder, StrokeOptions,
// };

use quicksilver::{
    geom::Vector,
    graphics::{Color, Element, FontRenderer, Graphics, Mesh, Vertex},
    Result,
};

pub const LINE_WIDTH: f32 = 4.0;
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

pub const FIELD_COLOR: Color = Color {
    r: 0x4D as f32 / 255.0,
    g: 0xAF as f32 / 255.0,
    b: 0x4A as f32 / 255.0,
    a: 0.5,
};

lazy_static! {
    pub static ref COLORS: [Color; 61] = [
        Color::from_rgba(0xB2, 0x18, 0x2B, 1.0),
        Color::from_rgba(0x37, 0x7E, 0xB8, 1.0),
        Color::from_rgba(0x4D, 0xAF, 0x4A, 1.0),
        Color::from_rgba(0x98, 0x4E, 0xA3, 1.0),
        Color::from_rgba(0xFF, 0x7F, 0x00, 1.0),
        Color::from_rgba(0xA6, 0x56, 0x28, 1.0),
        Color::from_rgba(0xF7, 0x81, 0xBF, 1.0),
        Color::from_rgba(0x99, 0x33, 0x00, 1.0),
        Color::from_rgba(0x33, 0x33, 0x00, 1.0),
        Color::from_rgba(0x00, 0x33, 0x00, 1.0),
        Color::from_rgba(0x00, 0x33, 0x66, 1.0),
        Color::from_rgba(0x00, 0x00, 0x80, 1.0),
        Color::from_rgba(0x33, 0x33, 0x99, 1.0),
        Color::from_rgba(0x33, 0x33, 0x33, 1.0),
        Color::from_rgba(0x80, 0x00, 0x00, 1.0),
        Color::from_rgba(0xFF, 0x66, 0x00, 1.0),
        Color::from_rgba(0x80, 0x80, 0x00, 1.0),
        Color::from_rgba(0x00, 0x80, 0x00, 1.0),
        Color::from_rgba(0x00, 0x80, 0x80, 1.0),
        Color::from_rgba(0x00, 0x00, 0xFF, 1.0),
        Color::from_rgba(0x66, 0x66, 0x99, 1.0),
        Color::from_rgba(0x80, 0x80, 0x80, 1.0),
        Color::from_rgba(0xFF, 0x00, 0x00, 1.0),
        Color::from_rgba(0xFF, 0x99, 0x00, 1.0),
        Color::from_rgba(0x99, 0xCC, 0x00, 1.0),
        Color::from_rgba(0x33, 0x99, 0x66, 1.0),
        Color::from_rgba(0x33, 0xCC, 0xCC, 1.0),
        Color::from_rgba(0x33, 0x66, 0xFF, 1.0),
        Color::from_rgba(0x80, 0x00, 0x80, 1.0),
        Color::from_rgba(0x96, 0x96, 0x96, 1.0),
        Color::from_rgba(0xFF, 0x00, 0xFF, 1.0),
        Color::from_rgba(0xFF, 0xCC, 0x00, 1.0),
        Color::from_rgba(0xFF, 0xFF, 0x00, 1.0),
        Color::from_rgba(0x00, 0xFF, 0x00, 1.0),
        Color::from_rgba(0x00, 0xFF, 0xFF, 1.0),
        Color::from_rgba(0x00, 0xCC, 0xFF, 1.0),
        Color::from_rgba(0x99, 0x33, 0x66, 1.0),
        Color::from_rgba(0xC0, 0xC0, 0xC0, 1.0),
        Color::from_rgba(0xFF, 0x99, 0xCC, 1.0),
        Color::from_rgba(0xFF, 0xCC, 0x99, 1.0),
        Color::from_rgba(0xFF, 0xFF, 0x99, 1.0),
        Color::from_rgba(0xCC, 0xFF, 0xCC, 1.0),
        Color::from_rgba(0xCC, 0xFF, 0xFF, 1.0),
        Color::from_rgba(0x99, 0xCC, 0xFF, 1.0),
        Color::from_rgba(0xCC, 0x99, 0xFF, 1.0),
        Color::from_rgba(0x99, 0x99, 0xFF, 1.0),
        Color::from_rgba(0x99, 0x33, 0x66, 1.0),
        Color::from_rgba(0xFF, 0xFF, 0xCC, 1.0),
        Color::from_rgba(0xCC, 0xFF, 0xFF, 1.0),
        Color::from_rgba(0x66, 0x00, 0x66, 1.0),
        Color::from_rgba(0xFF, 0x80, 0x80, 1.0),
        Color::from_rgba(0x00, 0x66, 0xCC, 1.0),
        Color::from_rgba(0xCC, 0xCC, 0xFF, 1.0),
        Color::from_rgba(0x00, 0x00, 0x80, 1.0),
        Color::from_rgba(0xFF, 0x00, 0xFF, 1.0),
        Color::from_rgba(0xFF, 0xFF, 0x00, 1.0),
        Color::from_rgba(0x00, 0xFF, 0xFF, 1.0),
        Color::from_rgba(0x80, 0x00, 0x80, 1.0),
        Color::from_rgba(0x80, 0x00, 0x00, 1.0),
        Color::from_rgba(0x00, 0x80, 0x80, 1.0),
        Color::from_rgba(0x00, 0x00, 0xFF, 1.0),
    ];
}

pub trait Args {
    fn get_args(&self) -> Result<String>;
    fn get_variant(&self) -> String;
}

pub trait Algorithm {
    fn name(&self) -> String;
    fn re_init(&mut self, variant: String);
    fn update(&mut self);
    fn draw(&self, gfx: &mut Graphics, font: &mut FontRenderer) -> Result<()>;
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

pub fn push_vertex(pos: Vector, vertices: &mut Vec<Vertex>) -> u32 {
    let rv = vertices.len() as u32;
    vertices.push(Vertex {
        pos,
        uv: None,
        color: COLORS[0],
    });
    rv
}

pub fn draw_board(grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize]) -> Result<Mesh> {
    let mut vertices = vec![];
    let mut elements = vec![];
    for (j, row) in grid.iter().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            let x = i as f32;
            let y = j as f32;
            let ne = Vector::new((x + 1.0) * CELL_WIDTH + OFFSET, y * CELL_WIDTH + OFFSET);
            let nw = Vector::new(x * CELL_WIDTH + OFFSET, y * CELL_WIDTH + OFFSET);
            let se = Vector::new(
                (x + 1.0) * CELL_WIDTH + OFFSET,
                (y + 1.0) * CELL_WIDTH + OFFSET,
            );
            let sw = Vector::new(x * CELL_WIDTH + OFFSET, (y + 1.0) * CELL_WIDTH + OFFSET);

            let ne = push_vertex(ne, &mut vertices);
            let nw = push_vertex(nw, &mut vertices);
            let se = push_vertex(se, &mut vertices);
            let sw = push_vertex(sw, &mut vertices);

            //Figure out which lines to draw.
            if !cell.contains(Direction::North) {
                elements.push(Element::Line([ne, nw]));
            }
            if !cell.contains(Direction::East) && (x, y) != (COLUMNS - 1.0, ROWS - 1.0) {
                elements.push(Element::Line([ne, se]));
            }
            if !cell.contains(Direction::South) {
                elements.push(Element::Line([se, sw]));
            }
            if !cell.contains(Direction::West) && (x, y) != (0.0, 0.0) {
                elements.push(Element::Line([nw, sw]));
            }
        }
    }
    Ok(Mesh {
        vertices,
        elements,
        image: None,
    })
}
