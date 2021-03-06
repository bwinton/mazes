use crate::util::{
    draw_board, Algorithm, ChooseRandom, Direction, CELL_WIDTH, COLORS, COLUMNS, EMPTY_COLOR,
    OFFSET, ROWS,
};
use enumset::EnumSet;
use macroquad::{logging as log, prelude::draw_rectangle, rand::gen_range};
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
    path: Vec<(usize, usize)>,
    finished: [[bool; COLUMNS as usize]; ROWS as usize],
    grid: [[EnumSet<Direction>; COLUMNS as usize]; ROWS as usize],
    stack: Vec<[[Blob; COLUMNS as usize]; ROWS as usize]>,
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
        let finished = [[false; COLUMNS as usize]; ROWS as usize];

        Self {
            path: vec![],
            finished,
            grid,
            stack: vec![],
            state: State::Setup,
        }
    }

    fn choose_starts(board: &[[Blob; COLUMNS as usize]; ROWS as usize]) -> [(usize, usize); 2] {
        let mut potentials = vec![];
        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell == &Blob::None {
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
        board: &[[Blob; COLUMNS as usize]; ROWS as usize],
    ) -> ([[Blob; COLUMNS as usize]; ROWS as usize], usize) {
        let mut remaining = 0;
        let mut new_board = *board;
        for (y, row) in board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell == &Blob::None {
                    let mut potentials: Vec<Direction> = EnumSet::all().iter().collect();
                    potentials.shuffle();
                    for direction in potentials {
                        let (new_x, new_y) = match direction {
                            Direction::North => (x as i32, y as i32 - 1),
                            Direction::East => (x as i32 + 1, y as i32),
                            Direction::South => (x as i32, y as i32 + 1),
                            Direction::West => (x as i32 - 1, y as i32),
                        };
                        // log::info!("{:?} / {:?} -> {:?}", (x,y), direction, (new_x, new_y));
                        if 0 <= new_x && new_x < COLUMNS as i32 && 0 <= new_y && new_y < ROWS as i32
                        {
                            let (new_x, new_y) = (new_x as usize, new_y as usize);
                            if [Blob::First, Blob::Second].contains(&board[new_y][new_x])
                                && (gen_range(0, 2) == 0)
                            {
                                // Only expand half the time.
                                new_board[y][x] = board[new_y][new_x];
                                break;
                            }
                        }
                    }
                }
                if new_board[y][x] == Blob::None {
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
                self.stack
                    .push([[Blob::None; COLUMNS as usize]; ROWS as usize]);
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
                board[a.1][a.0] = Blob::First;
                board[b.1][b.0] = Blob::Second;
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
                            Blob::First => first_size += 1,
                            Blob::Second => second_size += 1,
                            _ => {}
                        }
                        for direction in EnumSet::all().iter() {
                            let (new_x, new_y) = match direction {
                                Direction::North => (x as i32, y as i32 - 1),
                                Direction::East => (x as i32 + 1, y as i32),
                                Direction::South => (x as i32, y as i32 + 1),
                                Direction::West => (x as i32 - 1, y as i32),
                            };
                            if 0 <= new_x
                                && new_x < COLUMNS as i32
                                && 0 <= new_y
                                && new_y < ROWS as i32
                            {
                                let (new_x, new_y) = (new_x as usize, new_y as usize);
                                match (board[y][x], board[new_y][new_x]) {
                                    (Blob::First, Blob::Second) | (Blob::Second, Blob::First) => {
                                        // Draw a wall!
                                        walls.push((x, y, direction));
                                        self.grid[y][x].remove(direction);
                                        self.grid[new_y][new_x].remove(direction.opposite());
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
                let (new_x, new_y) = match direction {
                    Direction::North => (x, y - 1),
                    Direction::East => (x + 1, y),
                    Direction::South => (x, y + 1),
                    Direction::West => (x - 1, y),
                };
                self.grid[y][x].insert(direction);
                self.grid[new_y][new_x].insert(direction.opposite());

                // log::info!("Carving {:?} out of {:?}", (x,y,direction), walls);
                if first_size <= 3 || second_size <= 3 {
                    // Set too-small blobs as finished.
                    for (y, row) in board.iter().enumerate() {
                        for (x, cell) in row.iter().enumerate() {
                            match cell {
                                Blob::First if first_size <= 3 => self.finished[y][x] = true,
                                Blob::Second if second_size <= 3 => self.finished[y][x] = true,
                                _ => {}
                            };
                        }
                    }
                }

                // If either of the blobs are big enough, add them to the stack.
                if first_size > 3 {
                    let mut new_board = board;
                    for (y, row) in board.iter().enumerate() {
                        for (x, cell) in row.iter().enumerate() {
                            new_board[y][x] = match cell {
                                Blob::First => Blob::None,
                                _ => Blob::Outside,
                            };
                        }
                    }
                    self.stack.push(new_board);
                }
                if second_size > 3 {
                    let mut new_board = board;
                    for (y, row) in board.iter().enumerate() {
                        for (x, cell) in row.iter().enumerate() {
                            new_board[y][x] = match cell {
                                Blob::Second => Blob::None,
                                _ => Blob::Outside,
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
                    if cell == &Blob::None {
                        a_size += 1;
                    }
                    if b[y][x] == Blob::None {
                        b_size += 1;
                    }
                }
            }
            b_size.partial_cmp(&a_size).unwrap()
        });
    }

    fn draw(&self) {
        draw_board(self.grid);

        // if self.state != State::Done {
        let mut none_color = COLORS[1];
        none_color.a = 0.3;
        let mut first_color = COLORS[2];
        first_color.a = 0.3;
        let mut second_color = COLORS[3];
        second_color.a = 0.3;

        if let Some(board) = self.stack.last() {
            for (y, row) in board.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if !self.finished[y][x] {
                        let color = match cell {
                            Blob::None => none_color,
                            Blob::First => first_color,
                            Blob::Second => second_color,
                            Blob::Outside => EMPTY_COLOR,
                        };
                        draw_rectangle(
                            x as f32 * CELL_WIDTH + OFFSET,
                            y as f32 * CELL_WIDTH + OFFSET,
                            CELL_WIDTH,
                            CELL_WIDTH,
                            color,
                        );
                    }
                }
            }
        }
    }
}
