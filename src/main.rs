#[macro_use]
extern crate clap;

use clap::Arg;

use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, quit, EventHandler};
use ggez::graphics::{Color, DrawMode, DrawParam, LineCap, MeshBuilder, StrokeOptions};
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{graphics, Context, ContextBuilder, GameResult};

const LINE_WIDTH: f32 = 4.0;
const CELL_WIDTH: f32 = 20.0;
const COLUMNS: f32 = 40.0;
const ROWS: f32 = 30.0;

struct MyGame {
    // Your state here...
    lines: Vec<[[f32; 2]; 2]>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        let mut lines = vec![];
        for i in 0..=(COLUMNS as i32) {
            for j in 0..=(ROWS as i32) {
                let x = i as f32;
                let y = j as f32;
                lines.push([
                    [(x - 1.0) * CELL_WIDTH, y * CELL_WIDTH],
                    [x * CELL_WIDTH, y * CELL_WIDTH],
                ]);
                lines.push([
                    [x * CELL_WIDTH, (y - 1.0) * CELL_WIDTH],
                    [x * CELL_WIDTH, y * CELL_WIDTH],
                ]);
            }
        }
        MyGame { lines }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        if keycode == KeyCode::Q && keymods.contains(KeyMods::LOGO) {
            quit(ctx);
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        let mut builder = MeshBuilder::new();
        let options = StrokeOptions::default()
            .with_line_width(LINE_WIDTH)
            .with_line_cap(LineCap::Round);
        let color = Color::from_rgba_u32(0x88_00_44_FF);
        for line in &self.lines {
            builder.polyline(DrawMode::Stroke(options), line, color)?;
        }
        let mesh = builder.build(ctx)?;
        let dest = DrawParam::default().dest([LINE_WIDTH / 2.0, LINE_WIDTH / 2.0]);

        graphics::draw(ctx, &mesh, dest)?;
        graphics::present(ctx)
    }
}

fn main() {
    let matches = app_from_crate!("\n")
        .arg(
            Arg::with_name("algorithm")
                .short('a')
                .about("Which algorithm to run")
                .long_about("Specify an algorithm to run.")
                .takes_value(true)
                .possible_values(&[
                    "backtrack",
                    "parallel",
                    "eller",
                    "kruskal",
                    "prim",
                    "recdiv",
                    "blobby",
                    "aldousbroder",
                    "wilson",
                    "houston",
                    "huntandkill",
                    "tree",
                    "growingbintree",
                    "bintree",
                    "sidewinder",
                ])
                .default_value("backtrack"),
        )
        .get_matches();

    let algorithm = matches.value_of("algorithm").unwrap();
    println!("Algorithm: {:?}", algorithm);

    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("mazes", "Blake Winton")
        .window_setup(WindowSetup::default().title("Some mazes…"))
        .window_mode(WindowMode::default().dimensions(
            COLUMNS * CELL_WIDTH + LINE_WIDTH,
            ROWS * CELL_WIDTH + LINE_WIDTH,
        ))
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}
