use std::{f32::consts::PI, fmt::Display};

use crate::util::{Algorithm, CELL_WIDTH, COLORS, COLUMNS, LINE_WIDTH, OFFSET, ROWS};
use maze_utils::From;

use macroquad::{
    logging as log,
    shapes::{draw_circle, draw_line, draw_rectangle_lines},
};

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Running,
    Done,
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
        let length = 50.0;
        let small_length = length * 2.0 / (1.0 + 5.0f32.sqrt());

        let tiles = match variant {
            Variant::Sun => {
                // Create wheel of red triangles around the origin
                let mut rv = vec![];
                for i in 0..5 {
                    let a = Point(x_center, y_center);
                    let b = Point::polar_to_rect(length, (i as f32 * 4.0 + 1.0) * PI / 10.0)
                        .offset(x_center, y_center);
                    let c = Point::polar_to_rect(length, (i as f32 * 4.0 + 3.0) * PI / 10.0)
                        .offset(x_center, y_center);
                    let d = Point::polar_to_rect(length, (i as f32 * 4.0 + 5.0) * PI / 10.0)
                        .offset(x_center, y_center);
                    rv.push(Tile::Kite(a, b, c, d));
                }
                rv
            }
            Variant::Star => {
                let mut rv = vec![];
                for i in 0..5 {
                    let a = Point(x_center, y_center);
                    let b = Point::polar_to_rect(length, (i as f32 * 4.0 - 1.0) * PI / 10.0)
                        .offset(x_center, y_center);
                    let c = Point::polar_to_rect(small_length, (i as f32 * 4.0 + 1.0) * PI / 10.0)
                        .offset(x_center, y_center);
                    let d = Point::polar_to_rect(length, (i as f32 * 4.0 + 3.0) * PI / 10.0)
                        .offset(x_center, y_center);
                    rv.push(Tile::Dart(a, b, c, d));
                }
                rv
            }

            _ => todo!(),
        };
        log::info!("Drawing {}, {:?}", variant, &tiles);
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
        log::info!("Re-initing with {}", variant);

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
