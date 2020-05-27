use ggez::{Context, GameResult};

pub const LINE_WIDTH: f32 = 4.0;
pub const CELL_WIDTH: f32 = 20.0;
pub const COLUMNS: f32 = 40.0;
pub const ROWS: f32 = 30.0;

pub const COLORS: [u32; 7] = [
    0xB2_18_2B_FF,
    0x37_7E_B8_FF,
    0x4D_AF_4A_FF,
    0x98_4E_A3_FF,
    0xFF_7F_00_FF,
    0xA6_56_28_FF,
    0xF7_81_BF_FF,
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
