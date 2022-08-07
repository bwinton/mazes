use std::{f32::consts::PI, fmt::Display};

use crate::util::{Algorithm, CELL_WIDTH, COLORS, COLUMNS, LINE_WIDTH, OFFSET, ROWS};
use maze_utils::From;

use macroquad::{
    logging as log,
    shapes::{draw_circle, draw_line, draw_rectangle_lines},
};

const LENGTH: f32 = 50.0;
lazy_static! {
    static ref SMALL_LENGTH: f32 = LENGTH * 2.0 / (1.0 + 5.0f32.sqrt());
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
}

#[derive(Debug)]
struct Point(f32, f32);

impl Point {
    fn offset(&self, x: f32, y: f32) -> Self {
        Self(self.0 + x, self.1 + y)
    }

    pub(crate) fn polar_to_rect(r: f32, theta: f32) -> Self {
        Self(r * theta.cos(), r * theta.sin())
    }
}

#[derive(Debug)]
enum Tile {
    Kite(Point, Point, Point, Point),
    Dart(Point, Point, Point, Point),
}

impl Tile {
    fn draw(&self) {
        match self {
            Tile::Kite(a, b, c, d) | Tile::Dart(a, b, c, d) => {
                draw_line(a.0, a.1, b.0, b.1, LINE_WIDTH, COLORS[2]);
                draw_line(b.0, b.1, c.0, c.1, LINE_WIDTH, COLORS[2]);
                draw_line(c.0, c.1, d.0, d.1, LINE_WIDTH, COLORS[2]);
                draw_line(d.0, d.1, a.0, a.1, LINE_WIDTH, COLORS[2]);
            }
        }
    }

    fn build_kite(i: f32, x_center: f32, y_center: f32) -> Tile {
        let a = Point(x_center, y_center);
        let b = Point::polar_to_rect(LENGTH, i * PI / 10.0).offset(x_center, y_center);
        let c = Point::polar_to_rect(LENGTH, (i + 2.0) * PI / 10.0).offset(x_center, y_center);
        let d = Point::polar_to_rect(LENGTH, (i + 4.0) * PI / 10.0).offset(x_center, y_center);
        Tile::Kite(a, b, c, d)
    }

    fn build_dart(i: f32, x_center: f32, y_center: f32) -> Tile {
        let small_length = *SMALL_LENGTH;
        let a = Point(x_center, y_center);
        let b = Point::polar_to_rect(LENGTH, i * PI / 10.0).offset(x_center, y_center);
        let c =
            Point::polar_to_rect(small_length, (i + 2.0) * PI / 10.0).offset(x_center, y_center);
        let d = Point::polar_to_rect(LENGTH, (i + 4.0) * PI / 10.0).offset(x_center, y_center);
        Tile::Dart(a, b, c, d)
    }
}

impl Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Variant::Sun => f.write_str("Sun"),
            Variant::Star => f.write_str("Star"),
            Variant::Ace => f.write_str("Ace"),
            Variant::Deuce => f.write_str("Deuce"),
            Variant::Jack => f.write_str("Jack"),
            Variant::Queen => f.write_str("Queen"),
            Variant::King => f.write_str("King"),
        }
    }
}

#[derive(Debug)]
enum Variant {
    Sun,
    Star,
    Ace,
    Deuce,
    Jack,
    Queen,
    King,
}

impl Variant {
    fn start_tiles(&self, x_center: f32, y_center: f32) -> Vec<Tile> {
        let mut tiles = vec![];
        let small_length = *SMALL_LENGTH;

        match self {
            Variant::Sun => {
                // Create wheel of red triangles around the origin
                for i in 0..5 {
                    tiles.push(Tile::build_kite(i as f32 * 4.0 + 1.0, x_center, y_center));
                }
            }
            Variant::Star => {
                for i in 0..5 {
                    tiles.push(Tile::build_dart(i as f32 * 4.0 - 1.0, x_center, y_center));
                }
            }
            Variant::Ace => {
                let tile = Tile::build_kite(1.0, x_center, y_center - LENGTH);
                tiles.push(tile);
                let tile = Tile::build_kite(5.0, x_center, y_center - LENGTH);
                tiles.push(tile);
                let tile = Tile::build_dart(13.0, x_center, y_center + small_length);
                tiles.push(tile);
            }
            Variant::Deuce => {
                let tile = Tile::build_dart(1.0, x_center, y_center - LENGTH);
                tiles.push(tile);
                let tile = Tile::build_dart(5.0, x_center, y_center - LENGTH);
                tiles.push(tile);
                let offset = Point::polar_to_rect(LENGTH, 1.0 * PI / 10.0);
                let tile = Tile::build_kite(17.0, x_center - offset.0, y_center + offset.1);
                tiles.push(tile);
                let tile = Tile::build_kite(9.0, x_center + offset.0, y_center + offset.1);
                tiles.push(tile);
            }
            Variant::Jack => {
                let tile = Tile::build_kite(11.0, x_center, y_center);
                tiles.push(tile);
                let tile = Tile::build_kite(15.0, x_center, y_center);
                tiles.push(tile);
                let tile = Tile::build_kite(13.0, x_center, y_center + LENGTH);
                tiles.push(tile);

                let offset = Point::polar_to_rect(LENGTH, 19.0 * PI / 10.0);
                let tile = Tile::build_dart(5.0, x_center + offset.0, y_center + offset.1);
                tiles.push(tile);
                let tile = Tile::build_dart(1.0, x_center - offset.0, y_center + offset.1);
                tiles.push(tile);
            }
            Variant::Queen => {
                let tile = Tile::build_dart(13.0, x_center, y_center);
                tiles.push(tile);

                let offset = Point::polar_to_rect(LENGTH, 17.0 * PI / 10.0);
                let tile = Tile::build_kite(3.0, x_center + offset.0, y_center + offset.1);
                tiles.push(tile);
                let tile = Tile::build_kite(3.0, x_center - offset.0, y_center + offset.1);
                tiles.push(tile);

                let tile = Tile::build_kite(11.0, x_center, y_center + LENGTH);
                tiles.push(tile);
                let tile = Tile::build_kite(15.0, x_center, y_center + LENGTH);
                tiles.push(tile);
            }
            Variant::King => {
                for i in 2..5 {
                    tiles.push(Tile::build_dart(i as f32 * 4.0 + 1.0, x_center, y_center));
                }

                let offset = Point::polar_to_rect(LENGTH, 1.0 * PI / 10.0);
                let tile = Tile::build_kite(7.0, x_center + offset.0, y_center + offset.1);
                tiles.push(tile);
                let tile = Tile::build_kite(-1.0, x_center - offset.0, y_center + offset.1);
                tiles.push(tile);
            }
        };
        tiles
    }
}

#[derive(From)]
pub struct Exports {
    state: State,
    variant: Variant,
    tiles: Vec<Tile>,
}

impl Exports {
    pub fn new(variant: String) -> Self {
        let variant = match variant.as_str() {
            "sun" => Variant::Sun,
            "star" => Variant::Star,
            "ace" => Variant::Ace,
            "deuce" => Variant::Deuce,
            "jack" => Variant::Jack,
            "queen" => Variant::Queen,
            "king" => Variant::King,
            _ => panic!("Unknown Variant \"{}\"!", variant),
        };

        let w = COLUMNS * CELL_WIDTH;
        let h = ROWS * CELL_WIDTH;
        let x_center = w / 2.0 + OFFSET;
        let y_center = h / 2.0 + OFFSET;

        let tiles = variant.start_tiles(x_center, y_center);
        Self {
            state: State::Setup,
            variant,
            tiles,
        }
    }
}

impl Algorithm for Exports {
    fn name(&self) -> String {
        format!("Penrose {}", self.variant)
    }
    fn re_init(&mut self, variant: String) {
        self.from(Exports::new(variant));
    }
    fn get_variant(&self) -> String {
        self.variant.to_string().to_lowercase()
    }
    fn update(&mut self) {
        // log::info!("Updating {}", self.name());
        match self.state {
            State::Setup => {
                self.state = State::Running;
                return;
            }
            State::Done => {
                return;
            }
            _ => {}
        }

        let done = true;

        if done {
            self.state = State::Done;
            log::info!("Done!");
        }
    }

    fn draw(&self) {
        // draw_board(self.grid);
        let x = OFFSET;
        let y = OFFSET;
        let w = COLUMNS * CELL_WIDTH;
        let h = ROWS * CELL_WIDTH;
        draw_rectangle_lines(x, y, w, h, LINE_WIDTH, COLORS[0]);

        let x_center = w / 2.0 + x;
        let y_center = h / 2.0 + y;

        for tile in &self.tiles {
            tile.draw();
        }

        draw_circle(x_center, y_center, 3.0, COLORS[1]);

        // if self.state == State::Running {}
    }
}
