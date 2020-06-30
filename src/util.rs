use enumset::EnumSet;
use ggez::{Context, GameResult};
use ggez::graphics::{
    Color, DrawMode, LineCap, MeshBuilder, StrokeOptions,
};

pub const LINE_WIDTH: f32 = 4.0;
pub const CELL_WIDTH: f32 = 20.0;
pub const COLUMNS: f32 = 40.0;
pub const ROWS: f32 = 30.0;

pub const COLORS: [u32; 61] = [
    0xB2_18_2B_FF,
    0x37_7E_B8_FF,
    0x4D_AF_4A_FF,
    0x98_4E_A3_FF,
    0xFF_7F_00_FF,
    0xA6_56_28_FF,
    0xF7_81_BF_FF,
    0x99_33_00_FF,
    0x33_33_00_FF,
    0x00_33_00_FF,
    0x00_33_66_FF,
    0x00_00_80_FF,
    0x33_33_99_FF,
    0x33_33_33_FF,
    0x80_00_00_FF,
    0xFF_66_00_FF,
    0x80_80_00_FF,
    0x00_80_00_FF,
    0x00_80_80_FF,
    0x00_00_FF_FF,
    0x66_66_99_FF,
    0x80_80_80_FF,
    0xFF_00_00_FF,
    0xFF_99_00_FF,
    0x99_CC_00_FF,
    0x33_99_66_FF,
    0x33_CC_CC_FF,
    0x33_66_FF_FF,
    0x80_00_80_FF,
    0x96_96_96_FF,
    0xFF_00_FF_FF,
    0xFF_CC_00_FF,
    0xFF_FF_00_FF,
    0x00_FF_00_FF,
    0x00_FF_FF_FF,
    0x00_CC_FF_FF,
    0x99_33_66_FF,
    0xC0_C0_C0_FF,
    0xFF_99_CC_FF,
    0xFF_CC_99_FF,
    0xFF_FF_99_FF,
    0xCC_FF_CC_FF,
    0xCC_FF_FF_FF,
    0x99_CC_FF_FF,
    0xCC_99_FF_FF,
    0x99_99_FF_FF,
    0x99_33_66_FF,
    0xFF_FF_CC_FF,
    0xCC_FF_FF_FF,
    0x66_00_66_FF,
    0xFF_80_80_FF,
    0x00_66_CC_FF,
    0xCC_CC_FF_FF,
    0x00_00_80_FF,
    0xFF_00_FF_FF,
    0xFF_FF_00_FF,
    0x00_FF_FF_FF,
    0x80_00_80_FF,
    0x80_00_00_FF,
    0x00_80_80_FF,
    0x00_00_FF_FF,
];

pub trait Algorithm {
    fn name(&self) -> String;
    fn update(&mut self);
    fn draw(&self, ctx: &mut Context) -> GameResult<()>;
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

pub fn draw_board(grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize]) -> GameResult<MeshBuilder> {
    let mut builder = MeshBuilder::new();
    let options = StrokeOptions::default()
        .with_line_width(LINE_WIDTH)
        .with_line_cap(LineCap::Round);
    let line_color = Color::from_rgba_u32(COLORS[0]);
    for (j, row) in grid.iter().enumerate() {
        for (i, cell) in row.iter().enumerate() {
            let x = i as f32;
            let y = j as f32;
            //Figure out which lines to draw.
            if !cell.contains(Direction::North) {
                builder.polyline(
                    DrawMode::Stroke(options),
                    &[
                        [x * CELL_WIDTH, y * CELL_WIDTH],
                        [(x + 1.0) * CELL_WIDTH, y * CELL_WIDTH],
                    ],
                    line_color,
                )?;
            }
            if !cell.contains(Direction::East) {
                builder.polyline(
                    DrawMode::Stroke(options),
                    &[
                        [(x + 1.0) * CELL_WIDTH, y * CELL_WIDTH],
                        [(x + 1.0) * CELL_WIDTH, (y + 1.0) * CELL_WIDTH],
                    ],
                    line_color,
                )?;
            }
            if !cell.contains(Direction::South) {
                builder.polyline(
                    DrawMode::Stroke(options),
                    &[
                        [x * CELL_WIDTH, (y + 1.0) * CELL_WIDTH],
                        [(x + 1.0) * CELL_WIDTH, (y + 1.0) * CELL_WIDTH],
                    ],
                    line_color,
                )?;
            }
            if !cell.contains(Direction::West) {
                builder.polyline(
                    DrawMode::Stroke(options),
                    &[
                        [x * CELL_WIDTH, y * CELL_WIDTH],
                        [x * CELL_WIDTH, (y + 1.0) * CELL_WIDTH],
                    ],
                    line_color,
                )?;
            }
        }
    }
    Ok(builder)
}