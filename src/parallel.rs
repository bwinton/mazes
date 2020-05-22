use crate::{Algorithm, CELL_WIDTH, COLUMNS, LINE_WIDTH, ROWS};

use std::collections::VecDeque;
use enumset::EnumSet;
use ggez::graphics::{Color, DrawMode, Rect, DrawParam, LineCap, MeshBuilder, StrokeOptions, FillOptions};
use ggez::{graphics, Context, GameResult};

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};


#[derive(EnumSetType, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done
}

pub struct Exports {
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    rng: ThreadRng,
    stack: VecDeque<(usize, usize, EnumSet<Direction>)>,
    state: State,
}

impl Exports {
    pub fn new() -> impl Algorithm {
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let mut rng = thread_rng();
        let mut stack = VecDeque::new();
        stack.push_front((
            rng.gen_range(0, COLUMNS as usize),
            rng.gen_range(0, ROWS as usize),
            EnumSet::all()
        ));
        let state = State::Setup;
        Self {
            grid,
            rng,
            stack,
            state,
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Backtrack")
    }
    fn update(&mut self) {
        // println!("Updating {}", self.name());
        if self.state != State::Running {
            if self.state == State::Setup {
                self.state = State::Running;
            }
            return;
        }

        let mut found = false;
        // let (first_x, first_y, _) = self.stack.front().unwrap().clone();

        while !found {
            if self.stack.is_empty() {
                self.state = State::Done;
                println!("Done!");
                return;
            }

            let (x,y, directions) = self.stack.pop_front().unwrap();
            let mut potentials: Vec<Direction> = directions.iter().collect();
            if potentials.is_empty() {
                return;
            }
            potentials.shuffle(&mut self.rng);
            let direction = potentials.pop().unwrap();
            // println!("({},{}) -> {:?}", x, y, direction);
            self.stack.push_front((x, y, directions ^ direction));
    
            let (new_x, new_y) = match direction {
                Direction::North => (x as i32, y as i32 - 1),
                Direction::East => (x as i32+ 1, y as i32),
                Direction::South => (x as i32, y as i32 + 1),
                Direction::West => (x as i32 - 1, y as i32)
            };
            // println!("{:?} / {:?} -> {:?}", (x,y), direction, (new_x, new_y));
            if 0 <= new_x && new_x < COLUMNS as i32 && 0 <= new_y && new_y < ROWS as i32{
                let (new_x, new_y) = (new_x as usize, new_y as usize);
                if self.grid[new_y][new_x] == EnumSet::new() {
                    self.grid[y][x] |= direction;
                    self.grid[new_y][new_x] |= direction.opposite();
                    self.stack.push_front((new_x, new_y, EnumSet::all() ^ direction.opposite()));
                    found = true;
                }
                // Otherwise, loop again and see what we can get.
            }
            // if potentials.is_empty() {
            //     return;
            // }

        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        // Draw code here...
        let mut builder = MeshBuilder::new();
        let options = StrokeOptions::default()
            .with_line_width(LINE_WIDTH)
            .with_line_cap(LineCap::Round);
        let color = Color::from_rgba_u32(0x88_00_88_FF);
        let color_2 = Color::from_rgba_u32(0x00_00_00_88);
        for (j, row) in self.grid.iter().enumerate() {
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
                        color,
                    )?;
                }
                if !cell.contains(Direction::East) {
                    builder.polyline(
                        DrawMode::Stroke(options),
                        &[
                            [(x + 1.0) * CELL_WIDTH, y * CELL_WIDTH],
                            [(x + 1.0) * CELL_WIDTH, (y + 1.0) * CELL_WIDTH],
                        ],
                        color,
                    )?;
                }
                if !cell.contains(Direction::South) {
                    builder.polyline(
                        DrawMode::Stroke(options),
                        &[
                            [x * CELL_WIDTH, (y + 1.0) * CELL_WIDTH],
                            [(x + 1.0) * CELL_WIDTH, (y + 1.0) * CELL_WIDTH],
                        ],
                        color,
                    )?;
                }
                if !cell.contains(Direction::West) {
                    builder.polyline(
                        DrawMode::Stroke(options),
                        &[
                            [x * CELL_WIDTH, y * CELL_WIDTH],
                            [x * CELL_WIDTH, (y + 1.0) * CELL_WIDTH],
                        ],
                        color,
                    )?;
                }
            }
        }
        if let Some((x, y, _)) = self.stack.front() {
            builder.rectangle(
                DrawMode::Fill(FillOptions::default()),
                Rect::new(*x as f32 * CELL_WIDTH + LINE_WIDTH, *y as f32 * CELL_WIDTH + LINE_WIDTH,
                    CELL_WIDTH - LINE_WIDTH * 2.0, CELL_WIDTH - LINE_WIDTH * 2.0),
                color_2
            );
        }
        let mesh = builder.build(ctx)?;
        let dest = DrawParam::default().dest([LINE_WIDTH / 2.0, LINE_WIDTH / 2.0]);

        graphics::draw(ctx, &mesh, dest)?;

        Ok(())
    }
}
