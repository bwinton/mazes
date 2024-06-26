use crate::util::{
    draw_board, Algorithm, ChooseRandom, Direction, CELL_WIDTH, COLORS, COLUMNS, LINE_WIDTH,
    OFFSET, ROWS,
};
use array_init::array_init;
use enumset::EnumSet;
use macroquad::{logging as log, prelude::draw_rectangle, rand::gen_range};
use maze_utils::From;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Merging,
    NextLine,
    Dropping,
    Done,
}

#[derive(From)]
pub struct Exports {
    path: Vec<(usize, usize)>,
    current_row: usize,
    current_column: usize,
    empty_sets: Vec<usize>,
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    grid_sets: [[Option<usize>; COLUMNS as usize]; ROWS as usize],
    sets: Vec<(Vec<usize>, usize)>,
    state: State,
}

impl Exports {
    pub fn new() -> Self {
        let grid = [[EnumSet::new(); COLUMNS as usize]; ROWS as usize];
        let grid_sets = [[None; COLUMNS as usize]; ROWS as usize];

        Self {
            path: vec![],
            current_row: 0,
            current_column: 0,
            empty_sets: vec![],
            grid,
            grid_sets,
            sets: vec![],
            state: State::Setup,
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Eller")
    }
    fn re_init(&mut self, _variant: String) {
        self.from(Exports::new());
    }
    fn get_variant(&self) -> String {
        "unused".to_owned()
    }
    fn update(&mut self) {
        // log::info!("{}, {:?}", self.current_row, self.state);
        match self.state {
            State::Setup => {
                for x in 0..COLUMNS as usize {
                    self.grid_sets[self.current_row][x] = Some(x);
                }
                self.state = State::Merging;
            }
            State::Done => {}
            State::Merging => {
                if self.grid_sets[self.current_row][self.current_column].is_none() {
                    self.grid_sets[self.current_row][self.current_column] = self.empty_sets.pop();
                }
                if self.grid_sets[self.current_row][self.current_column + 1].is_none() {
                    self.grid_sets[self.current_row][self.current_column + 1] =
                        self.empty_sets.pop();
                }
                if gen_range(0, 2) == 0 || self.current_row == (ROWS - 1.0) as usize {
                    // Merge the cells, if they're in different sets.
                    let old_set = self.grid_sets[self.current_row][self.current_column + 1];
                    let new_set = self.grid_sets[self.current_row][self.current_column];
                    if new_set != old_set {
                        // log::info!(
                        //     "Merging {}: {:?} and {:?}…",
                        //     x, self.grid[self.current_row][x], self.grid[self.current_row][x]
                        // );

                        self.grid[self.current_row][self.current_column] |= Direction::East;
                        self.grid[self.current_row][self.current_column + 1] |= Direction::West;

                        for i in 0..COLUMNS as usize {
                            if self.grid_sets[self.current_row][i] == old_set {
                                self.grid_sets[self.current_row][i] = new_set;
                            }
                        }
                    }
                }
                self.current_column += 1;
                if self.current_column == (COLUMNS - 1.0) as usize {
                    if self.current_row != (ROWS - 1.0) as usize {
                        self.state = State::NextLine;
                    } else {
                        self.current_row += 1;
                        self.state = State::Done;
                        log::info!("Done!");
                    }
                }
            }
            State::NextLine => {
                // Find the current sets.
                self.sets.clear();
                self.empty_sets.clear();
                let mut sets: [Vec<usize>; COLUMNS as usize] = array_init(|_| Vec::new());
                for x in 0..COLUMNS as usize {
                    let i = self.grid_sets[self.current_row][x].unwrap();
                    sets[i].push(x);
                }
                for (i, set) in sets.iter().enumerate() {
                    if set.is_empty() {
                        self.empty_sets.push(i);
                    } else {
                        self.sets.push((set.clone(), i));
                    }
                }
                self.sets.sort();
                self.empty_sets.sort_unstable();
                self.empty_sets.reverse();
                self.current_column = 0;

                self.state = State::Dropping;
            }
            State::Dropping => {
                // Pick 1..n of each set and drop it.
                if let Some((set, i)) = self.sets.pop() {
                    // print!("{}: {:?}, Dropping: ", i, set);
                    let count = gen_range(1, set.len() + 1);
                    for &cell in set.choose_multiple(count) {
                        // print!("{}, ", cell);
                        self.grid[self.current_row][cell] |= Direction::South;
                        self.grid[self.current_row + 1][cell] |= Direction::North;
                        self.grid_sets[self.current_row + 1][cell] = Some(i);
                    }
                    // log::info!();
                }
                // self.current_column += 1;

                if self.sets.is_empty() {
                    self.current_row += 1;
                    self.state = State::Merging;
                }
            }
        }
    }

    fn draw(&self) {
        // Draw code here...
        draw_board(self.grid);

        for row in self.current_row..self.current_row + 2 {
            if row < ROWS as usize {
                for x in 0..COLUMNS as usize {
                    // log::info!("{:?}.", self.grid[self.current_row][x]);
                    if let Some(i) = self.grid_sets[row][x] {
                        let curr_color = COLORS[i + 1];
                        let mut cell_color = COLORS[i + 1];
                        cell_color.a = 0.5;
                        draw_rectangle(
                            x as f32 * CELL_WIDTH + OFFSET,
                            row as f32 * CELL_WIDTH + OFFSET,
                            CELL_WIDTH,
                            CELL_WIDTH,
                            cell_color,
                        );

                        if row == self.current_row
                            && x == self.current_column
                            && self.state == State::Merging
                        {
                            draw_rectangle(
                                x as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                                row as f32 * CELL_WIDTH + LINE_WIDTH + OFFSET,
                                CELL_WIDTH - LINE_WIDTH * 2.0,
                                CELL_WIDTH - LINE_WIDTH * 2.0,
                                curr_color,
                            );
                        }
                    }
                }
            }
        }
    }
}
