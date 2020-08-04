use crate::util::{draw_board, Algorithm, Direction, CELL_WIDTH, COLORS, COLUMNS, OFFSET, ROWS};
use enumset::EnumSet;
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::Graphics,
    log, Result,
};
use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}

pub struct Exports {
    edges: Vec<(usize, usize, Direction)>,
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    parents: [[Option<(usize, usize)>; COLUMNS as usize]; ROWS as usize],
    rng: ThreadRng,
    roots: Vec<(usize, usize, usize)>,
    state: State,
}

impl Exports {
    pub fn new() -> Self {
        let edges = vec![];
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let parents = [[None; COLUMNS as usize]; ROWS as usize];
        let rng = thread_rng();
        let roots = vec![];
        let state = State::Setup;
        Self {
            edges,
            grid,
            parents,
            rng,
            roots,
            state,
        }
    }
    fn from(&mut self, other: Self) {
        self.edges = other.edges;
        self.grid = other.grid;
        self.parents = other.parents;
        self.rng = other.rng;
        self.roots = other.roots;
        self.state = other.state;
    }

    fn find_root(&self, x: usize, y: usize) -> (usize, usize) {
        let (mut x, mut y) = (x, y);
        while let Some((new_x, new_y)) = self.parents[y][x] {
            x = new_x;
            y = new_y;
        }
        (x, y)
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Kruskal")
    }
    fn re_init(&mut self) {
        self.from(Exports::new());
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                for x in 0..COLUMNS as usize {
                    for y in 0..ROWS as usize {
                        if y > 0 {
                            self.edges.push((x, y, Direction::North));
                        }
                        if x > 0 {
                            self.edges.push((x, y, Direction::West));
                        }
                    }
                }
                self.edges.shuffle(&mut self.rng);

                self.state = State::Running;
                return;
            }
            State::Done => {
                return;
            }
            _ => {}
        }

        let mut found = false;

        while !found {
            if self.edges.is_empty() {
                self.state = State::Done;
                log::info!("Done!");
                return;
            }

            let (x, y, direction) = self.edges.pop().unwrap();
            let root = self.find_root(x, y);

            let (new_x, new_y) = match direction {
                Direction::North => (x, (y as i32 - 1) as usize),
                Direction::East => (x + 1, y),
                Direction::South => (x, y + 1),
                Direction::West => ((x as i32 - 1) as usize, y),
            };
            let new_root = self.find_root(new_x, new_y);

            if root != new_root {
                // Connect the cells
                self.grid[y][x] |= direction;
                self.grid[new_y][new_x] |= direction.opposite();

                // Join the smaller set to the bigger set.
                let old_pos = self
                    .roots
                    .iter()
                    .position(|&r| r.0 == root.0 && r.1 == root.1);
                let new_pos = self
                    .roots
                    .iter()
                    .position(|&r| r.0 == new_root.0 && r.1 == new_root.1);

                let (old_found, old_size) = old_pos.map_or((false, 1), |r| (true, self.roots[r].2));
                let (new_found, new_size) = new_pos.map_or((false, 1), |r| (true, self.roots[r].2));

                let size = old_size + new_size;

                match (old_found, new_found) {
                    (true, true) => {
                        if old_size >= new_size {
                            self.parents[new_root.1][new_root.0] = Some(root);
                            self.roots[old_pos.unwrap()].2 = size;
                        } else {
                            self.parents[root.1][root.0] = Some(new_root);
                            self.roots[new_pos.unwrap()].2 = size;
                        }
                    }
                    (true, false) => {
                        self.parents[new_root.1][new_root.0] = Some(root);
                        self.roots[old_pos.unwrap()].2 = size;
                    }
                    (false, true) => {
                        self.parents[root.1][root.0] = Some(new_root);
                        self.roots[new_pos.unwrap()].2 = size;
                    }
                    (false, false) => {
                        self.parents[new_root.1][new_root.0] = Some(root);
                        self.roots.push((root.0, root.1, size));
                    }
                }

                found = true;
            }
        }
    }

    fn draw(&self, gfx: &mut Graphics) -> Result<()> {
        let elements = draw_board(self.grid)?;
        gfx.draw_mesh(&elements);

        if self.state == State::Running {
            for x in 0..COLUMNS as usize {
                for y in 0..ROWS as usize {
                    if self.grid[y][x] != EnumSet::empty() {
                        let root = self.find_root(x, y);
                        let index = self
                            .roots
                            .iter()
                            .position(|&r| r.0 == root.0 && r.1 == root.1)
                            .unwrap();
                        let mut color = COLORS[index % COLORS.len()];
                        color.a = 0.5;
                        let rect = Rectangle::new(
                            Vector::new(
                                x as f32 * CELL_WIDTH + OFFSET,
                                y as f32 * CELL_WIDTH + OFFSET,
                            ),
                            Vector::new(CELL_WIDTH, CELL_WIDTH),
                        );
                        gfx.fill_rect(&rect, color);
                    };
                }
            }
        }

        Ok(())
    }
}
