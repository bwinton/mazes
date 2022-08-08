use std::{f32::consts::PI, fmt::Display};

use crate::util::{Algorithm, CELL_WIDTH, COLORS, COLUMNS, LINE_WIDTH, OFFSET, ROWS, WHITE};
use maze_utils::From;

use macroquad::{
    logging as log,
    prelude::Vec2,
    shapes::{draw_line, draw_rectangle, draw_rectangle_lines, draw_triangle},
};

const LENGTH: f32 = 300.0;
lazy_static! {
    static ref GOLDEN_RATIO: f32 = (1.0 + 5.0f32.sqrt()) / 2.0;
    static ref SMALL_LENGTH: f32 = LENGTH / *GOLDEN_RATIO;
}

#[derive(PartialEq, Eq, Debug)]
enum State {
    Setup,
    Deflating,
    Growing,
    Done,
}

#[derive(Debug, Copy, Clone)]
struct Point(f32, f32);

impl Point {
    fn offset(&self, x: f32, y: f32) -> Self {
        Self(self.0 + x, self.1 + y)
    }

    pub(crate) fn polar_to_rect(r: f32, theta: f32) -> Self {
        Self(r * theta.cos(), r * theta.sin())
    }

    fn move_to(&self, a: &Point, ratio: f32) -> Self {
        let Point(cx, cy) = *self;
        let Point(ax, ay) = *a;
        let x = cx + (ax - cx) * ratio;
        let y = cy + (ay - cy) * ratio;
        Point(x, y)
    }

    fn inside(&self, start: Point, end: Point) -> bool {
        let Point(x, y) = start;
        let Point(w, h) = end;
        (self.0 >= x && self.0 <= w) && (self.1 >= y && self.1 <= h)
    }
}

impl From<Point> for Vec2 {
    fn from(p: Point) -> Self {
        Vec2::new(p.0, p.1)
    }
}

#[derive(Debug, Copy, Clone)]
struct Tile(bool, Point, Point, Point);

impl Tile {
    fn draw(&self) {
        let Tile(is_kite, a, b, c) = *self;
        let color = if is_kite { COLORS[3] } else { COLORS[4] };
        draw_line(a.0, a.1, b.0, b.1, LINE_WIDTH, COLORS[2]);
        draw_line(b.0, b.1, c.0, c.1, LINE_WIDTH, COLORS[2]);
        // draw_line(c.0, c.1, a.0, a.1, LINE_WIDTH, COLORS[5]);
        let b2 = c.move_to(&a, 0.5);

        draw_triangle(
            c.move_to(&a, 0.1).into(),
            a.move_to(&c, 0.1).into(),
            b.move_to(&b2, 0.2).into(),
            color,
        );
    }

    fn build_tile(i: f32, x_center: f32, y_center: f32, is_kite: bool) -> [Tile; 2] {
        let length = if is_kite { LENGTH } else { *SMALL_LENGTH };
        let a = Point(x_center, y_center);
        let b = Point::polar_to_rect(LENGTH, i * PI / 10.0).offset(x_center, y_center);
        let c = Point::polar_to_rect(length, (i + 2.0) * PI / 10.0).offset(x_center, y_center);
        let d = Point::polar_to_rect(LENGTH, (i + 4.0) * PI / 10.0).offset(x_center, y_center);
        // [Tile(is_kite, a, b, c), Tile(is_kite, a, b, c)]
        [Tile(is_kite, a, b, c), Tile(is_kite, a, d, c)]
    }

    fn subdivide(&self) -> Vec<Tile> {
        let Tile(is_kite, a, b, c) = *self;
        if is_kite {
            // Subdivide half kite triangle
            // Q = A + (B - A) / goldenRatio
            let q = b.move_to(&a, 1.0 / *GOLDEN_RATIO);
            // R = B + (C - B) / goldenRatio
            let r = a.move_to(&c, 1.0 / *GOLDEN_RATIO);
            // [(1, R, Q, B), (0, Q, A, R), (0, C, A, R)]
            vec![
                Tile(false, a, r, q),
                Tile(true, b, q, r),
                Tile(true, b, c, r),
            ]
        } else {
            // Subdivide half dart triangle
            // P = C + (A - C) / goldenRatio
            let p = a.move_to(&b, 1.0 / *GOLDEN_RATIO);
            // [(1, B, P, A), (0, P, C, B)]
            vec![Tile(true, a, p, c), Tile(false, b, c, p)]
        }
    }

    fn inside(&self, start: Point, end: Point) -> bool {
        self.1.inside(start, end) || self.2.inside(start, end) || self.3.inside(start, end)
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
                    tiles.extend(Tile::build_tile(
                        i as f32 * 4.0 + 1.0,
                        x_center,
                        y_center,
                        true,
                    ));
                }
            }
            Variant::Star => {
                for i in 0..5 {
                    tiles.extend(Tile::build_tile(
                        i as f32 * 4.0 - 1.0,
                        x_center,
                        y_center,
                        false,
                    ));
                }
            }
            Variant::Ace => {
                let tile = Tile::build_tile(1.0, x_center, y_center - LENGTH, true);
                tiles.extend(tile);
                let tile = Tile::build_tile(5.0, x_center, y_center - LENGTH, true);
                tiles.extend(tile);
                let tile = Tile::build_tile(13.0, x_center, y_center + small_length, false);
                tiles.extend(tile);
            }
            Variant::Deuce => {
                let tile = Tile::build_tile(1.0, x_center, y_center - LENGTH, false);
                tiles.extend(tile);
                let tile = Tile::build_tile(5.0, x_center, y_center - LENGTH, false);
                tiles.extend(tile);
                let offset = Point::polar_to_rect(LENGTH, 1.0 * PI / 10.0);
                let tile = Tile::build_tile(17.0, x_center - offset.0, y_center + offset.1, true);
                tiles.extend(tile);
                let tile = Tile::build_tile(9.0, x_center + offset.0, y_center + offset.1, true);
                tiles.extend(tile);
            }
            Variant::Jack => {
                let tile = Tile::build_tile(11.0, x_center, y_center, true);
                tiles.extend(tile);
                let tile = Tile::build_tile(15.0, x_center, y_center, true);
                tiles.extend(tile);
                let tile = Tile::build_tile(13.0, x_center, y_center + LENGTH, true);
                tiles.extend(tile);

                let offset = Point::polar_to_rect(LENGTH, 19.0 * PI / 10.0);
                let tile = Tile::build_tile(5.0, x_center + offset.0, y_center + offset.1, false);
                tiles.extend(tile);
                let tile = Tile::build_tile(1.0, x_center - offset.0, y_center + offset.1, false);
                tiles.extend(tile);
            }
            Variant::Queen => {
                let tile = Tile::build_tile(13.0, x_center, y_center, false);
                tiles.extend(tile);

                let offset = Point::polar_to_rect(LENGTH, 17.0 * PI / 10.0);
                let tile = Tile::build_tile(3.0, x_center + offset.0, y_center + offset.1, true);
                tiles.extend(tile);
                let tile = Tile::build_tile(3.0, x_center - offset.0, y_center + offset.1, true);
                tiles.extend(tile);

                let tile = Tile::build_tile(11.0, x_center, y_center + LENGTH, true);
                tiles.extend(tile);
                let tile = Tile::build_tile(15.0, x_center, y_center + LENGTH, true);
                tiles.extend(tile);
            }
            Variant::King => {
                for i in 2..5 {
                    tiles.extend(Tile::build_tile(
                        i as f32 * 4.0 + 1.0,
                        x_center,
                        y_center,
                        false,
                    ));
                }

                let offset = Point::polar_to_rect(LENGTH, 1.0 * PI / 10.0);
                let tile = Tile::build_tile(7.0, x_center + offset.0, y_center + offset.1, true);
                tiles.extend(tile);
                let tile = Tile::build_tile(-1.0, x_center - offset.0, y_center + offset.1, true);
                tiles.extend(tile);
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
                self.state = State::Deflating;
                return;
            }
            State::Done => {
                return;
            }
            State::Growing => {
                let w = COLUMNS * CELL_WIDTH;
                let h = ROWS * CELL_WIDTH;
                let x_center = w / 2.0 + OFFSET;
                let y_center = h / 2.0 + OFFSET;

                let grow_ratio = -0.03;
                for tile in self.tiles.iter_mut() {
                    tile.1 = tile.1.move_to(&Point(x_center, y_center), grow_ratio);
                    tile.2 = tile.2.move_to(&Point(x_center, y_center), grow_ratio);
                    tile.3 = tile.3.move_to(&Point(x_center, y_center), grow_ratio);
                }
                // Prune out the invisible tiles…
                // let size = self.tiles.len();
                self.tiles = self
                    .tiles
                    .iter()
                    .filter_map(|tile| {
                        if tile.inside(Point(OFFSET, OFFSET), Point(w + OFFSET, h + OFFSET)) {
                            Some(*tile)
                        } else {
                            None
                        }
                    })
                    .collect();
                // log::info!("Shrank from {} to {} tiles.", size, self.tiles.len());
                if self.tiles.len() < 2000 {
                    self.state = State::Deflating;
                }
                return;
            }
            _ => {}
        }

        let mut next = vec![];
        for tile in &self.tiles {
            next.extend(tile.subdivide());
        }
        self.tiles = next;
        self.state = State::Growing;

        if self.state == State::Done {
            log::info!("Done!");
        }
    }

    fn draw(&self) {
        for tile in &self.tiles {
            tile.draw();
        }

        let x = OFFSET;
        let y = OFFSET;
        let w = COLUMNS * CELL_WIDTH;
        let h = ROWS * CELL_WIDTH;
        draw_rectangle(0.0, 0.0, x, h, WHITE);
        draw_rectangle(0.0, 0.0, w, y, WHITE);
        draw_rectangle(x + w, 0.0, x, h, WHITE);
        draw_rectangle(0.0, y + h, w, y, WHITE);
        draw_rectangle_lines(x, y, w, h, LINE_WIDTH, COLORS[0]);
    }
}
