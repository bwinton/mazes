use crate::util::{
    draw_board, Algorithm, Direction, Grid, Playable, State, CELL_WIDTH, COLORS, COLUMNS, OFFSET,
    ROWS,
};
use enumset::EnumSet;
use macroquad::{logging as log, prelude::draw_rectangle, rand::gen_range};
use maze_utils::From;

#[derive(PartialEq, Eq, Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(From)]
pub struct Exports {
    path: Vec<(usize, usize)>,
    grid: Grid,
    stack: Vec<(usize, usize, usize, usize)>,
    state: State,
}

impl Exports {
    pub fn new() -> Self {
        let mut grid = [[EnumSet::all(); COLUMNS as usize]; ROWS as usize];
        for x in 0..COLUMNS as usize {
            grid[0][x].remove(Direction::North);
            grid[ROWS as usize - 1][x].remove(Direction::South);
        }
        for row in grid.iter_mut() {
            row[0].remove(Direction::West);
            row[COLUMNS as usize - 1].remove(Direction::East);
        }

        Self {
            path: vec![],
            grid,
            stack: vec![],
            state: State::Setup,
        }
    }

    fn choose_orientation(&mut self, width: usize, height: usize) -> Orientation {
        if width < height {
            Orientation::Horizontal
        } else if height < width {
            Orientation::Vertical
        } else if gen_range(0, 2) == 0 {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Recursive Division")
    }
    fn re_init(&mut self, _variant: String) {
        self.from(Exports::new());
    }
    fn get_variant(&self) -> String {
        "unused".to_owned()
    }
    fn update(&mut self) {
        if self.state == State::Setup {
            self.stack.push((0, 0, COLUMNS as usize, ROWS as usize));
            self.state = State::Running;
            return;
        }

        if self.stack.is_empty() {
            self.state = State::Done;
            log::info!("Done!");
            return;
        }

        let (x, y, width, height) = self.stack.pop().unwrap();
        let orientation = self.choose_orientation(width, height);

        match orientation {
            Orientation::Horizontal => {
                // log::info!("GenRange 1 {}-{}", y, y + height);
                let wall_y = gen_range(y, y + height - 1);
                // log::info!("GenRange 2 {}-{}", x, x + width);
                let passage_x = gen_range(x, x + width);
                for i in x..x + width {
                    self.grid[wall_y][i].remove(Direction::South);
                }
                self.grid[wall_y][passage_x].insert(Direction::South);

                let new_height = wall_y - y + 1;
                if width >= 2 && new_height >= 2 {
                    self.stack.push((x, y, width, new_height));
                }

                let new_height = height - new_height;
                if width >= 2 && new_height >= 2 {
                    self.stack.push((x, wall_y + 1, width, new_height));
                }
            }
            Orientation::Vertical => {
                // log::info!("GenRange 3 {}-{}", x, x + width);
                let wall_x = gen_range(x, x + width - 1);
                // log::info!("GenRange 4 {}-{}", y, y + height);
                let passage_y = gen_range(y, y + height);
                for row in self.grid.iter_mut().skip(y).take(height) {
                    row[wall_x].remove(Direction::East);
                }
                self.grid[passage_y][wall_x].insert(Direction::East);

                let new_width = wall_x - x + 1;
                if new_width >= 2 && height >= 2 {
                    self.stack.push((x, y, new_width, height));
                }

                let new_width = width - new_width;
                if new_width >= 2 && height >= 2 {
                    self.stack.push((wall_x + 1, y, new_width, height));
                }
            }
        }
        self.stack.sort_by(|a, b| {
            let a_size = a.2 * a.3;
            let b_size = b.2 * b.3;
            b_size.partial_cmp(&a_size).unwrap()
        });
    }

    fn draw(&self) {
        draw_board(self.grid);

        if self.state == State::Running {
            for (i, (x, y, width, height)) in self.stack.iter().enumerate() {
                let mut cell_color = COLORS[i];
                if i != self.stack.len() - 1 {
                    cell_color.a = 0.3;
                }
                draw_rectangle(
                    *x as f32 * CELL_WIDTH + OFFSET,
                    *y as f32 * CELL_WIDTH + OFFSET,
                    *width as f32 * CELL_WIDTH,
                    *height as f32 * CELL_WIDTH,
                    cell_color,
                );
            }
        }
    }

    fn get_state(&self) -> State {
        self.state
    }

    fn move_to(&mut self, pos: (f32, f32)) {
        Playable::move_to(self, pos);
    }
}

impl Playable for Exports {
    fn get_grid(&self) -> Grid {
        self.grid
    }

    fn get_path_mut(&mut self) -> &mut Vec<(usize, usize)> {
        &mut self.path
    }
}
