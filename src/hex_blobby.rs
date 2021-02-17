use crate::{
    hex_util::set_border,
    util::{Algorithm, ChooseRandom, COLORS, EMPTY_COLOR},
};

use crate::hex_util::{draw_board, draw_cell, init_grid, Direction, COLUMNS, ROWS};

use enumset::EnumSet;
use macroquad::{logging as log, rand::gen_range};
use maze_utils::From;

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Choosing,
    Expanding,
    Walling,
    Done,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Blob {
    None,
    First,
    Second,
    Outside,
}

#[derive(From)]
pub struct Exports {
    finished: [[Option<bool>; COLUMNS as usize]; ROWS as usize],
    grid: [[Option<EnumSet<Direction>>; COLUMNS as usize]; ROWS as usize],
    stack: Vec<[[Option<Blob>; COLUMNS as usize]; ROWS as usize]>,
    state: State,
}

impl Exports {
    pub fn new() -> Self {
        let mut grid = init_grid(EnumSet::all());
        set_border(&mut grid);
        let finished = init_grid(false);

        let stack = vec![];
        let state = State::Setup;
        log::info!("Init ({},{}): {:?}", 52, 0, grid[0][52]);

        Self {
            finished,
            grid,
            stack,
            state,
        }
    }

    fn choose_starts(
        board: &[[Option<Blob>; COLUMNS as usize]; ROWS as usize],
    ) -> [(usize, usize); 2] {
        let mut potentials = vec![];
        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell == &Some(Blob::None) {
                    potentials.push((x, y));
                }
            }
        }
        let mut rv = [(0usize, 0usize), (0usize, 0usize)];
        for (b, slot) in potentials.choose_multiple(rv.len()).zip(rv.iter_mut()) {
            *slot = *b;
        }
        // log::info!("Chose {:?} from {:?}", rv, potentials);
        rv
    }

    fn expand_blobs(
        board: &[[Option<Blob>; COLUMNS as usize]; ROWS as usize],
    ) -> ([[Option<Blob>; COLUMNS as usize]; ROWS as usize], usize) {
        let mut remaining = 0;
        let mut new_board = *board;
        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell == &Some(Blob::None) {
                    let mut potentials: Vec<Direction> = EnumSet::all().iter().collect();
                    potentials.shuffle();
                    for direction in potentials {
                        let (new_x, new_y) = direction.next(x as i32, y as i32);
                        // log::info!("{:?} / {:?} -> {:?}", (x,y), direction, (new_x, new_y));
                        if 0 <= new_x && new_x < COLUMNS as i32 && 0 <= new_y && new_y < ROWS as i32
                        {
                            let (new_x, new_y) = (new_x as usize, new_y as usize);
                            if [Some(Blob::First), Some(Blob::Second)]
                                .contains(&board[new_y][new_x])
                                && (gen_range(0, 2) == 0)
                            {
                                // Only expand half the time.
                                new_board[y][x] = board[new_y][new_x];
                                break;
                            }
                        }
                    }
                }
                if new_board[y][x] == Some(Blob::None) {
                    remaining += 1;
                }
            }
        }
        (new_board, remaining)
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        String::from("Blobby Recursive Division")
    }
    fn re_init(&mut self, _variant: String) {
        self.from(Exports::new());
    }
    fn get_variant(&self) -> String {
        "unused".to_owned()
    }
    fn update(&mut self) {
        match self.state {
            State::Setup => {
                self.stack.push(init_grid(Blob::None));
                self.state = State::Choosing;
                return;
            }
            State::Done => {
                return;
            }
            _ => {}
        }

        if self.stack.is_empty() {
            self.state = State::Done;
            log::info!("Done!");
            return;
        }

        match self.state {
            State::Choosing => {
                let board = self.stack.last_mut().unwrap();
                let [a, b] = Self::choose_starts(board);
                board[a.1][a.0] = Some(Blob::First);
                board[b.1][b.0] = Some(Blob::Second);
                self.state = State::Expanding;
            }
            State::Expanding => {
                let board = self.stack.last_mut().unwrap();
                let (new_board, remaining) = Self::expand_blobs(board);
                *board = new_board;

                // log::info!("Expanding: {} remaining…", remaining);
                if remaining == 0 {
                    self.state = State::Walling;
                }
            }
            State::Walling => {
                // Draw the walls!
                let board = self.stack.pop().unwrap();
                let mut first_size = 0;
                let mut second_size = 0;
                let mut walls = vec![];
                for x in 0..COLUMNS as usize {
                    for y in 0..ROWS as usize {
                        match board[y][x] {
                            Some(Blob::First) => first_size += 1,
                            Some(Blob::Second) => second_size += 1,
                            _ => {}
                        }
                        for direction in EnumSet::all().iter() {
                            let direction: Direction = direction;
                            let (new_x, new_y) = direction.next(x as i32, y as i32);
                            if 0 <= new_x
                                && new_x < COLUMNS as i32
                                && 0 <= new_y
                                && new_y < ROWS as i32
                            {
                                let (new_x, new_y) = (new_x as usize, new_y as usize);
                                match (board[y][x], board[new_y][new_x]) {
                                    (Some(Blob::First), Some(Blob::Second))
                                    | (Some(Blob::Second), Some(Blob::First)) => {
                                        // Draw a wall!
                                        walls.push((x, y, direction));
                                        if let Some(cell) = &mut self.grid[y][x] {
                                            cell.remove(direction);
                                        };
                                        if let Some(cell) = &mut self.grid[new_y][new_x] {
                                            cell.remove(direction.opposite());
                                        };
                                    }
                                    _ => {
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
                // Carve a door in the wall.
                walls.shuffle();
                let (x, y, direction) = walls.pop().unwrap();
                let (new_x, new_y) = direction.next(x as i32, y as i32);
                if let Some(cell) = &mut self.grid[y][x] {
                    cell.insert(direction);
                };
                if let Some(cell) = &mut self.grid[new_y as usize][new_x as usize] {
                    cell.insert(direction.opposite());
                };

                // log::info!("Carving {:?} out of {:?}", (x,y,direction), walls);
                if first_size <= 3 || second_size <= 3 {
                    // Set too-small blobs as finished.
                    for (y, row) in board.iter().enumerate() {
                        for (x, cell) in row.iter().enumerate() {
                            match cell {
                                Some(Blob::First) if first_size <= 2 => {
                                    self.finished[y][x] = Some(true)
                                }
                                Some(Blob::Second) if second_size <= 2 => {
                                    self.finished[y][x] = Some(true)
                                }
                                _ => {}
                            };
                        }
                    }
                }

                // If either of the blobs are big enough, add them to the stack.
                if first_size > 2 {
                    let mut new_board = board;
                    for (y, row) in board.iter().enumerate() {
                        for (x, cell) in row.iter().enumerate() {
                            new_board[y][x] = match cell {
                                Some(Blob::First) => Some(Blob::None),
                                Some(_) => Some(Blob::Outside),
                                _ => None,
                            };
                        }
                    }
                    self.stack.push(new_board);
                }
                if second_size > 2 {
                    let mut new_board = board;
                    for (y, row) in board.iter().enumerate() {
                        for (x, cell) in row.iter().enumerate() {
                            new_board[y][x] = match cell {
                                Some(Blob::Second) => Some(Blob::None),
                                Some(_) => Some(Blob::Outside),
                                _ => None,
                            };
                        }
                    }
                    self.stack.push(new_board);
                }
                self.state = State::Choosing;
            }
            _ => {
                panic!(
                    "Should be unable to hit state {:?} in this match!",
                    self.state
                );
            }
        }

        self.stack.sort_by(|a, b| {
            let mut a_size = 0;
            let mut b_size = 0;
            for (y, row) in a.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if cell == &Some(Blob::None) {
                        a_size += 1;
                    }
                    if b[y][x] == Some(Blob::None) {
                        b_size += 1;
                    }
                }
            }
            b_size.partial_cmp(&a_size).unwrap()
        });
    }

    fn draw(&self) {
        draw_board(self.grid);

        if self.state == State::Done {
            return;
        }

        let mut none_color = COLORS[1];
        none_color.a = 0.3;
        let mut first_color = COLORS[2];
        first_color.a = 0.3;
        let mut second_color = COLORS[3];
        second_color.a = 0.3;

        if let Some(board) = self.stack.last() {
            for (y, row) in board.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if Some(false) == self.finished[y][x] {
                        let color = match cell {
                            Some(Blob::None) => none_color,
                            Some(Blob::First) => first_color,
                            Some(Blob::Second) => second_color,
                            Some(Blob::Outside) => EMPTY_COLOR,
                            _ => panic!("Out of the grid!"),
                        };
                        draw_cell(x, y, 0.0, color);
                    }
                }
            }
        }
    }
}
